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

use kernel_common::Content;
use kernel_common::Message;

// sender 1. python stdout/stderr stream
// sender 2. execute code
#[cfg(feature = "tcp")]
pub fn spawn_output_sender_thread(
    output_tx: crossbeam_channel::Sender<Message>,
    output_receiver: std::sync::mpsc::Receiver<Message>,
) {
    std::thread::Builder::new()
        .name("output".to_string())
        .spawn(move || {
            let mut last_req_cell_id = "".to_string();
            loop {
                let msg = match output_receiver.recv() {
                    Ok(msg) => msg,
                    Err(err) => {
                        tracing::info!("err output_receiver.recv() {err}");
                        continue;
                    }
                };
                let msg_content_str = format!("{:?}", msg.content);
                if msg_content_str.len() > 200 {
                    tracing::debug!("publish large msg content");
                } else {
                    tracing::debug!("publish msg content {:?}", msg.content);
                }
                match msg.content {
                    Content::UpdateLastReq {} => {
                        last_req_cell_id = msg.header.cell_id;
                    }
                    // only stream and input_request need to modify header
                    // Content::Stream { .. } | Content::InputRequest { .. } => {
                    //     let mut msg = msg;
                    //     msg.header.cell_id = last_req_cell_id.clone();
                    //     if let Err(err) = output_tx.send(msg) {
                    //         tracing::error!("{err}");
                    //         std::process::exit(1);
                    //     }
                    // }
                    _ => {
                        let mut msg = msg;
                        msg.header.cell_id = last_req_cell_id.clone();
                        if let Err(err) = output_tx.send(msg) {
                            tracing::error!("{err} output_tx cap {:?}", output_tx.capacity());
                            std::process::exit(1);
                        }
                    }
                }
            }
        })
        .unwrap();
}

#[cfg(not)]
#[cfg(feature = "fifo")]
pub fn spawn_output_sender_thread(output_receiver: std::sync::mpsc::Receiver<Message>, inode: u64) {
    use std::io::Write;
    std::thread::Builder::new()
        .name("output".to_string())
        .spawn(move || {
            let fifo_name = kernel_common::transport::kernel_rsp_pipe(std::process::id(), inode);
            let mut output = std::fs::OpenOptions::new()
                .write(true)
                .open(&fifo_name)
                .unwrap();

            let mut last_req_cell_id = "".to_string();
            loop {
                let msg = match output_receiver.recv() {
                    Ok(msg) => msg,
                    Err(err) => {
                        tracing::info!("err output_receiver.recv() {err}");
                        continue;
                    }
                };
                let msg_content_str = format!("{:?}", msg.content);
                if msg_content_str.len() > 200 {
                    tracing::info!("publish large msg content");
                } else {
                    tracing::info!("publish msg content {:?}", msg.content);
                }
                match msg.content {
                    Content::UpdateLastReq {} => {
                        last_req_cell_id = msg.header.cell_id;
                    }
                    // only stream type need to modify header
                    Content::Stream { .. } => {
                        let mut msg = msg;
                        msg.header.cell_id = last_req_cell_id.clone();
                        if let Err(err) = output.write_all(&msg.to_json()) {
                            tracing::error!("{err}");
                            // output = std::fs::OpenOptions::new()
                            //     .write(true)
                            //     .open(&fifo_name)
                            //     .unwrap();
                        }
                    }

                    _ => {
                        if let Err(err) = output.write_all(&msg.to_json()) {
                            tracing::error!("{err}");
                            // output = std::fs::OpenOptions::new()
                            //     .write(true)
                            //     .open(&fifo_name)
                            //     .unwrap();
                        }
                    }
                }
            }
        })
        .unwrap();
}
