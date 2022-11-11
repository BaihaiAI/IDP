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



use kernel_common::Message;
use kernel_common::MsgType;

use super::TestContext;

#[test]
fn test_try_catch() {
    let ctx = TestContext::new();
    let msg = Message::execute_req(
        "import time
try:
    time.ctime()
except:
    pass",
    );
    ctx.client_shell
        .send_multipart(msg.encode_to_multipart(), 0)
        .unwrap();
    let execute_reply = ctx.client_shell.recv_multipart(0).unwrap();
    let execute_reply = Message::decode_from_multipart(execute_reply);
    dbg!(execute_reply);

    for msg_type in [
        MsgType::status,
        MsgType::execute_input,
        // MsgType::stream,
        MsgType::status,
    ] {
        let resp = ctx.client_iopub.recv_multipart(0).unwrap();
        let resp = Message::decode_from_multipart(resp);
        assert_eq!(resp.header.msg_type, msg_type);
        dbg!(resp);
    }
}

#[test]
fn test_try_catch2() {
    let ctx = TestContext::new();
    let msg = Message::execute_req(
        "
try:
    print(1)
except:
    pass",
    );
    ctx.client_shell
        .send_multipart(msg.encode_to_multipart(), 0)
        .unwrap();
    let execute_reply = ctx.client_shell.recv_multipart(0).unwrap();
    let _execute_reply = Message::decode_from_multipart(execute_reply);
    // dbg!(execute_reply);

    for msg_type in [
        MsgType::status,
        MsgType::execute_input,
        MsgType::stream,
        MsgType::status,
    ] {
        let resp = ctx.client_iopub.recv_multipart(0).unwrap();
        let resp = Message::decode_from_multipart(resp);
        assert_eq!(resp.header.msg_type, msg_type);
        dbg!(resp);
    }
}
