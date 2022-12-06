// Copyright 2022 BaihaiAI, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![deny(unused_crate_dependencies)]
#![deny(clippy::unused_async)]
mod kernel_app;
mod kernel_init;
use std::net::TcpStream;

use kernel_common::Content;
use kernel_init::py_stdin::IS_WAITING_INPUT_REPLY;
use tracing::error;
use ws_tool::codec::WsStringCodec;
use ws_tool::errors::WsError;
use ws_tool::frame::OpCode;
use ws_tool::stream::WsStream;

// #[cfg(test)]
// mod test_kernel;

pub(crate) mod execute_code;

#[cfg(feature = "fifo")]
#[cfg(not)]
pub fn main() {
    logger::init_logger();
    let args = <arg::KernelArgs as clap::Parser>::parse();
    // let kernel_info = idp_kernel::kernel_info::KernelInfo::new();
    let kernel = kernel_app::KernelApp::new(args);
    kernel.main_loop();
}

fn kernel_manage_ws_connect(
    pod_id: String,
    header: kernel_common::Header,
) -> WsStringCodec<WsStream<TcpStream>> {
    // let team_id = header.team_id;
    // if ipynb path contains chinese, must encoding it
    let json_str = serde_json::to_string(&kernel_common::KernelInfo {
        // ip would set again by server from remote_addr, so we use a default value
        ip: os_utils::network::dns_resolve(&os_utils::get_hostname()),
        pid: std::process::id(),
        pod_id,
        header,
    })
    .unwrap();

    // HTTP headers can't has chinese, HTTP header is ASCII only
    // let ascii_json_str = if json_str.is_ascii() {
    //     json_str
    // } else {
    //     urlencoding::encode(&json_str).to_string()
    // };
    let ascii_json_str = urlencoding::encode(&json_str).to_string();

    let hostname = business::kubernetes::tenant_cluster_header_k8s_svc();
    let url = format!(
        "ws://{hostname}:{}/api/v1/execute/ws/kernel/connect",
        business::kernel_manage_port(),
    );

    /*
    match ws_tool::ClientBuilder::new(&url)
        .header(kernel_common::KernelInfo::HTTP_HEADER, &ascii_json_str)
        .connect(WsStringCodec::check_fn)
    {
        Ok(stream) => {
            return stream;
        }
        Err(err) => {
            tracing::error!("hostname={hostname} resolve fail, use public executor hostname {err}");
            let url = url.replace(&team_id.to_string(), "executor");
            ws_tool::ClientBuilder::new(&url)
                .header(kernel_common::KernelInfo::HTTP_HEADER, ascii_json_str)
                .connect(WsStringCodec::check_fn)
                .expect(&url)
        }
    }
    */
    ws_tool::ClientBuilder::new(&url)
        .header(kernel_common::KernelInfo::HTTP_HEADER, ascii_json_str)
        .connect(WsStringCodec::check_fn)
        .expect(&url)
}

pub fn main(args: Vec<String>) {
    logger::init_logger();

    if args.len() != 2 && args.len() != 3 {
        eprintln!("usage: kernel --version or kernel $json_base64");
        eprintln!("wrong args = {args:#?}");
        std::process::exit(libc::EXIT_FAILURE);
    }
    if args[1] == "--version" {
        println!("{}", env!("VERSION"));
        return;
    }

    #[cfg(not)]
    if business::kubernetes::is_k8s() && cfg!(target_os = "linux") {
        _ = std::fs::create_dir("/store/cores");
        std::process::Command::new("sudo")
            .arg("bash")
            .arg("-c")
            .arg(
                "echo '/store/core_dumped/core_dumped.%h.%e.%p.%t' > /proc/sys/kernel/core_pattern",
            )
            .spawn()
            .expect("sudo bash spawn error")
            .wait()
            .expect("set core_pattern");
    }

    let header_str = if args[1].starts_with('{') {
        // is json
        args[1].clone()
    } else {
        // is base64
        let header_str =
            base64::decode(&args[1]).unwrap_or_else(|_| panic!("base64 decode {}", args[1]));
        String::from_utf8(header_str).expect("String::from_utf8")
    };
    tracing::info!("header_str = {header_str}");
    let header =
        serde_json::from_str::<kernel_common::Header>(&header_str).expect("serde_json::from_str");
    let pod_id = match args.get(2) {
        Some(id) => id.clone(),
        None => format!("{}-{}", header.team_id, header.inode()),
    };

    let (output_tx, output_rx) = crossbeam_channel::unbounded();
    let (input_reply_tx, input_reply_rx) = std::sync::mpsc::channel();
    let (execute_tx, execute_rx) = std::sync::mpsc::channel();

    // kernel_manage would ensure only one msg is handling, so interrupt is safe to do
    let header_ = header.clone();
    std::thread::Builder::new()
        .name("ws".to_string())
        .spawn(move || {
            let (mut ws_r, mut ws_w) = kernel_manage_ws_connect(pod_id, header_).split();
            let (ws_r_tx, ws_r_rx) = crossbeam_channel::unbounded();
            std::thread::Builder::new()
                .name("ws_read".to_string())
                .spawn(move || {
                    loop {
                        match ws_r.receive() {
                            Ok(req) => {
                                if let Err(err) = ws_r_tx.send(req) {
                                    tracing::error!("{err}");
                                }
                            }
                            Err(err) => {
                                tracing::error!("{err}");
                                match err {
                                    WsError::IOError(_err) => {
                                        std::process::exit(0);
                                    }
                                    _ => break,
                                }
                            }
                        }
                    }
                })
                .unwrap();
            let (ws_ping_tx, ws_ping_rx) = crossbeam_channel::bounded(1);
            std::thread::Builder::new()
                .name("ws_ping".to_string())
                .spawn(move || {
                    loop {
                        if let Err(err) = ws_ping_tx.send(()) {
                            error!("{err}");
                        }
                        std::thread::sleep(std::time::Duration::from_secs(12));
                    }
                })
                .unwrap();
            loop {
                crossbeam_channel::select! {
                    recv(ws_r_rx) -> msg_res => {
                        if handle_ws_msg(msg_res, &execute_tx, &input_reply_tx) {
                            break;
                        }
                    }
                    recv(output_rx) -> rsp_res => {
                        let rsp = match rsp_res {
                            Ok(rsp) => rsp,
                            Err(err) => {
                                error!("{err}");
                                continue;
                            },
                        };
                        let rsp = serde_json::to_string(&rsp).unwrap();
                        if let Err(err) = ws_w.send(rsp) {
                            error!("{err}");
                        }
                    }
                    recv(ws_ping_rx) -> _ => {
                        if let Err(err) = ws_w.send((OpCode::Ping, "".to_string())) {
                            error!("{err}");
                        }
                    }
                }
            }
            if let Err(err) = ws_w.send((OpCode::Close, "".to_string())) {
                error!("{err}");
            }
            std::process::exit(0);
        })
        .unwrap();

    let mut kernel = kernel_app::KernelApp::new(output_tx, input_reply_rx, header);
    // NOTE!: must ensure **pyo3 python_thread run in main thread then kill send signal can work** (main thread tid = process pid)
    while let Ok((req, header)) = execute_rx.recv() {
        tracing::info!("--> idp kernel main thread: kernel.handle_req");
        kernel.handle_req(req, header);
        tracing::info!("<-- idp kernel main thread: kernel.handle_req");
    }
}

fn handle_ws_msg(
    msg_res: Result<ws_tool::Message<String>, crossbeam_channel::RecvError>,
    execute_tx: &std::sync::mpsc::Sender<(
        kernel_common::content::ExecuteRequest,
        kernel_common::Header,
    )>,
    input_reply_tx: &std::sync::mpsc::Sender<String>,
) -> bool {
    let msg = match msg_res {
        Ok(msg) => msg.data,
        Err(err) => {
            tracing::error!("{err}");
            return true;
        }
    };
    if msg.is_empty() {
        // it's a pong frame
        return false;
    }

    /*
    let msg = match msg.header().opcode() {
        OpCode::Text => String::from_utf8(msg.payload().to_vec()).unwrap(),
        OpCode::Close => {
            return true;
        }
        _ => {
            error!("receive unexpected opcode {:?}", msg.header().opcode());
            return true;
        }
    };
    */
    let req = serde_json::from_str::<kernel_common::Message>(&msg).unwrap();
    match req.content {
        Content::ExecuteRequest(req_) => {
            tracing::info!("recv ExecuteRequest");
            if let Err(err) = execute_tx.send((req_, req.header)) {
                tracing::error!("{err}");
            }
        }
        Content::InputReply { value } => {
            tracing::info!("recv InputReply");
            input_reply_tx.send(value).unwrap();
            tracing::info!("after send InputReply to stdin");
        }

        Content::InterruptRequest => {
            tracing::info!("recv  InterruptRequest");
            #[cfg(unix)]
            if unsafe {
                libc::kill(
                    libc::getpid(),
                    kernel_init::init_signal_handler::INTERRUPT_SIGNAL,
                )
            } != 0
            {
                tracing::error!("{}", std::io::Error::last_os_error());
            }
            // FIXME windows not work
            #[cfg(windows)]
            unsafe {
                let h_process = winapi::um::processthreadsapi::OpenProcess(
                    winapi::um::winnt::PROCESS_ALL_ACCESS,
                    0,
                    libc::getpid().try_into().unwrap(),
                );
                let u_exit_code = 0_u32;
                winapi::um::processthreadsapi::TerminateProcess(h_process, u_exit_code);
            }
            // #[cfg(windows)]
            // {
            //     let mut cmd = std::process::Command::new("python");
            //     let code_str = format!("import os;os.kill(os.getpid(),__import__('signal').CRTL_C_EVENT)",);
            // }

            if unsafe { IS_WAITING_INPUT_REPLY } {
                // we can't send empty string because empty string is EOF
                input_reply_tx.send(" ".to_string()).unwrap();
                unsafe { IS_WAITING_INPUT_REPLY = false };
            }
            tracing::info!("after InterruptRequest");
        }
        Content::ShutdownRequest { .. } => {
            tracing::info!("recv ShutdownRequest");
            return true;
        }
        _ => {
            tracing::warn!("unsupported msg type");
        }
    };
    false
}
