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

#![deny(warnings)]
#![deny(unused_crate_dependencies)]
#![deny(clippy::unused_async)]
mod kernel_app;
mod kernel_init;
use kernel_init::py_stdin::IS_WAITING_INPUT_REPLY;

// #[cfg(test)]
// mod test_kernel;

pub(crate) mod execute_code;

#[cfg(feature = "fifo")]
#[cfg(not)]
pub fn main() {
    logger::init_logger();
    let args = <arg::KernelArgs as clap::Parser>::parse();
    tracing::info!("args = {args:?}");

    // let kernel_info = idp_kernel::kernel_info::KernelInfo::new();
    // tracing::info!("{:#?}", kernel_info);
    let kernel = kernel_app::KernelApp::new(args);
    kernel.main_loop();
}

fn kernel_manage_ws_req(
    ray_id: String,
    header: kernel_common::Header,
) -> tokio_tungstenite::tungstenite::handshake::client::Request {
    // if ipynb path contains chinese, must encoding it
    let json_str = serde_json::to_string(&kernel_common::KernelInfo {
        // ip would set again by server from remote_addr, so we use a default value
        ip: os_utils::network::dns_resolve(&os_utils::get_hostname()),
        pid: std::process::id(),
        ray_id,
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

    let hostname = business::kubernetes::cluster_header_k8s_svc();
    let url = format!(
        "ws://{hostname}:{}/api/v1/execute/ws/kernel/connect",
        business::kernel_manage_port(),
    );

    // https://github.com/snapview/tokio-tungstenite/issues/212
    let mut req =
        tokio_tungstenite::tungstenite::client::IntoClientRequest::into_client_request(url)
            .unwrap();
    req.headers_mut().insert(
        kernel_common::KernelInfo::HTTP_HEADER,
        ascii_json_str.parse().unwrap(),
    );
    req
}

#[cfg(feature = "tcp")]
pub fn main() {
    use futures_util::SinkExt;
    use futures_util::StreamExt;
    use kernel_common::Content;
    use tokio_tungstenite::connect_async;
    use tokio_tungstenite::tungstenite::Message;
    logger::init_logger();

    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 && args.len() != 3 {
        eprintln!("usage: kernel --version or kernel $header_json_str $ray_id");
        eprintln!("wrong args = {args:#?}");
        std::process::exit(libc::EXIT_FAILURE);
    }
    if args[1] == "--version" {
        println!("{}", env!("VERSION"));
        return;
    }
    let header_str = &args[1];
    let ray_id = args[2].clone();
    tracing::info!("header_str = {header_str}");
    let header = serde_json::from_str::<kernel_common::Header>(header_str).unwrap();

    let (output_tx, mut output_rx) = tokio::sync::mpsc::unbounded_channel();
    let (input_reply_tx, input_reply_rx) = std::sync::mpsc::channel();
    let (tx, rx) = std::sync::mpsc::channel();

    // kernel_manage would ensure only one msg is handling, so interrupt is safe to do
    let header_ = header.clone();
    std::thread::Builder::new()
        .name("ws".to_string())
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .enable_time()
                .build()
                .unwrap(); //  ::new().unwrap();
            rt.block_on(async {
                // unsafe {
                //     dbg!(&libc::pthread_self());
                //     dbg!(&libc::getppid());
                // }
                let req = kernel_manage_ws_req(ray_id, header_);
                let (stream, _) = connect_async(req).await.unwrap();
                let (mut w, mut r) = stream.split();

                let mut ping_interval = tokio::time::interval(std::time::Duration::from_secs(10));
                loop {
                    tokio::select! {
                        Some(Ok(req)) = r.next() => {
                            // tracing::info!("req = {req}");
                            let req = match req {
                                Message::Text(msg) => {
                                    msg
                                },
                                Message::Pong(_) => {
                                    continue
                                },
                                Message::Close(_) => break,
                                _ => {

                                    tracing::error!("only support ws Text, Pong, Close frame");
                                    continue
                                },
                            };
                            let msg = serde_json::from_str::<kernel_common::Message>(&req).unwrap();
                            match msg.content {
                                Content::ExecuteRequest(req) => {
                                    tracing::info!("recv ExecuteRequest");
                                    if let Err(err) = tx.send((req, msg.header)) {
                                        tracing::error!("{err}");
                                    }
                                },
                                Content::InputReply { value } => {
                                    tracing::info!("recv InputReply");
                                    input_reply_tx.send(value).unwrap();
                                    tracing::info!("after send InputReply to stdin");
                                }

                                Content::InterruptRequest => {
                                    tracing::info!("recv  InterruptRequest");
                                    #[cfg(unix)]
                                    if unsafe { libc::kill(libc::getpid(), kernel_init::init_signal_handler::INTERRUPT_SIGNAL) } != 0 {
                                        tracing::error!("{}", std::io::Error::last_os_error());
                                    }
                                    #[cfg(windows)]
                                    if unsafe{
                                        let h_process = winapi::um::processthreadsapi::OpenProcess(winapi::um::winnt::PROCESS_ALL_ACCESS, 0, libc::getpid().try_into().unwrap());
                                        let u_exit_code = 0_u32;
                                        winapi::um::processthreadsapi::TerminateProcess(h_process, u_exit_code)
                                    } == 0 {
                                        tracing::error!("{}", std::io::Error::last_os_error());
                                    }
                                    // #[cfg(windows)]
                                    // if unsafe{
                                    //     winapi::um::wincon::GenerateConsoleCtrlEvent(winapi::um::wincon::CTRL_C_EVENT,0)
                                    // } == 0 {
                                    //     tracing::error!("{}", std::io::Error::last_os_error());
                                    // }
                                    // #[cfg(windows)]
                                    // {
                                    //     let mut cmd = std::process::Command::new("python");
                                    //     let code_str = format!("import os;os.kill(os.getpid(),__import__('signal').CRTL_C_EVENT)",);
                                    //     cmd.arg("-c").arg(code_str);
                                    //     tracing::info!("cmd = {cmd:?}");
                                    //     cmd.spawn().unwrap().wait().unwrap();
                                    // }

                                    if unsafe { IS_WAITING_INPUT_REPLY } {
                                        // we can't send empty string because empty string is EOF
                                        input_reply_tx.send(" ".to_string()).unwrap();
                                        unsafe { IS_WAITING_INPUT_REPLY = false };
                                    }
                                    tracing::info!("after InterruptRequest");
                                },
                                Content::ShutdownRequest { .. } => {
                                    tracing::info!("recv ShutdownRequest");
                                    if let Err(err) = w.send(Message::Close(None)).await {
                                        tracing::error!("{err}");
                                    }
                                    std::process::exit(0);
                                },
                                _ => {
                                    tracing::warn!("unsupported msg type");
                                }
                            };
                        }
                        Some(rsp) = output_rx.recv() => {
                            let rsp = serde_json::to_string(&rsp).unwrap();
                            if let Err(err) = w.send(Message::Text(rsp)).await {
                                tracing::error!("{err}");
                            }
                        }
                        _ = ping_interval.tick() => {
                            if let Err(err) = w.send(Message::Ping(Vec::new())).await {
                                tracing::error!("exit process: {err}");
                                std::process::exit(1);
                            }
                        }
                    }
                }
            });
        })
        .unwrap();

    let mut kernel = kernel_app::KernelApp::new(output_tx, input_reply_rx, header);
    // NOTE!: must ensure **pyo3 python_thread run in main thread then kill send signal can work** (main thread tid = process pid)
    while let Ok((req, header)) = rx.recv() {
        tracing::debug!("--> kernel.handle_req");
        kernel.handle_req(req, header);
        tracing::debug!("<-- kernel.handle_req");
    }
}
