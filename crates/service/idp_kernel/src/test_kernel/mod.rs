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



pub use kernel_common::Message;

// mod test_adtk;
// mod test_compile_error;
// mod test_lets_plot;
// mod test_matplotlib;
// mod test_pandas;
// mod test_plotly;
// mod test_print1_print2;
// mod test_runtime_error;
// mod test_string_literal;
// mod test_try_catch;

struct TestContext {
    kernel_pid: libc::pid_t,
    execute_read_filename: String,
}

impl Drop for TestContext {
    fn drop(&mut self) {
        unsafe {
            libc::kill(self.kernel_pid, libc::SIGTERM);
        }
    }
}

/*
"rust-analyzer.server.extraEnv": {
    "PYO3_PYTHON": "/home/w/.conda/envs/python39/bin/python",
},
// sudo ln -s /home/w/.conda/envs/python39/lib/libpython3.9.so.1.0 /usr/lib/libpython3.9.so.1.0
"rust-analyzer.runnableEnv": {
    "PYO3_PYTHON": "/home/w/.conda/envs/python39/bin/python",
    "PYTHONHOME": "/home/w/.conda/envs/python39",
}

*/
impl TestContext {
    const FAKE_INODE: u64 = 0;
    pub fn new() -> Self {
        logger::init_logger();
        std::env::set_var("MPLBACKEND", "module://baihai_matplotlib_backend");

        // use fork in development
        // use fork+exec in production mode for better performance
        let pid = unsafe { libc::fork() };
        if pid == -1 {
            panic!("{}", std::io::Error::last_os_error());
        }
        if pid == 0 {
            let kernel = crate::kernel_app::KernelApp::new(
                crate::kernel_info::KernelInfo::new(),
                Self::FAKE_INODE,
            );
            kernel.main_loop(Self::FAKE_INODE);
            std::process::exit(0);
        }

        TestContext {
            kernel_pid: pid,
            execute_read_filename: kernel_common::transport::execute_read_from_kernel_pipe(
                pid as u32,
                Self::FAKE_INODE,
            ),
        }
    }

    pub fn execute_req(&self, code: &str) {
        use std::io::Write;
        let msg = Message {
            header: kernel_common::Header::default(),
            content: kernel_common::Content::ExecuteRequest(
                kernel_common::content::ExecuteRequest {
                    code: code.to_string(),
                    ..Default::default()
                },
            ),
            ..Default::default()
        };
        let mut write_to_kernel = std::fs::OpenOptions::new()
            .write(true)
            .open(kernel_common::transport::execute_write_to_kernel_pipe(
                self.kernel_pid as u32,
                Self::FAKE_INODE,
            ))
            .unwrap();
        write_to_kernel.write_all(&msg.to_json()).unwrap();
    }

    pub fn recv_multi(&self) -> Vec<Message> {
        let fifo_read_output = std::fs::OpenOptions::new()
            .read(true)
            .open(&self.execute_read_filename)
            .unwrap();
        let mut ret = Vec::new();
        for msg_res in
            serde_json::Deserializer::from_reader(fifo_read_output).into_iter::<Message>()
        {
            let msg = msg_res.unwrap();
            let is_idle = msg.is_idle();
            ret.push(msg);
            if is_idle {
                break;
            }
        }
        ret
    }
}

// pub(crate) fn code_with_python_error(code: &str) {
//     let ctx = TestContext::new();
//     let msg = Message::execute_req(code);
//     ctx.client_shell
//         .send_multipart(msg.encode_to_multipart(), 0)
//         .unwrap();
//     let execute_reply = ctx.client_shell.recv_multipart(0).unwrap();
//     let execute_reply = Message::decode_from_multipart(execute_reply);
//     dbg!(execute_reply);

//     for msg_type in [
//         MsgType::status,
//         MsgType::execute_input,
//         MsgType::error,
//         MsgType::status,
//     ]
//     .into_iter()
//     {
//         let resp = ctx.client_iopub.recv_multipart(0).unwrap();
//         let resp = Message::decode_from_multipart(resp);
//         assert_eq!(resp.header.msg_type, msg_type);
//     }
// }

#[test]
fn test_print_5_times() {
    let ctx = TestContext::new();
    ctx.execute_req(
        "
import time
for i in range(5):
    print(i)
    time.sleep(0.2)
    ",
    );

    for msg in ctx.recv_multi() {
        dbg!(msg);
    }
}

#[test]
fn test_empty_stmts() {
    let ctx = TestContext::new();
    ctx.execute_req(
        r#"# response = requests.get("http://admin:admin@grafana.istio-system:3000/api/datasources/proxy/1/api/v1/query_range?query=" + urllib.parse.quote_plus(promsql) + time_range)"#,
    );
    for msg in ctx.recv_multi() {
        dbg!(msg);
    }
}
