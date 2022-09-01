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



use super::Message;
use super::MsgType;
use super::TestContext;

#[test]
fn test_compile_error_syntax_error() {
    super::code_with_python_error("a=1\nprint(\nb=3");
}

#[test]
fn test_compile_error_indentation_error() {
    super::code_with_python_error("def a():\n1");
}

#[test]
fn test_compile_ok_if_last_expr_contains_comment() {
    let code = "#%matplotlib inline
import d2lzh as d2l
from mxnet import image
d2l.set_figsize()
img = image.imread('catdog.jpg').asnumpy()
d2l.plt.imshow(img);
#d2l.plt.show(img)";
    let code = code.replace(
        "catdog.jpg",
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/test_kernel/test_sample.png"
        ),
    );

    let ctx = TestContext::new();
    let msg = Message::execute_req(&code);
    ctx.client_shell
        .send_multipart(msg.encode_to_multipart(), 0)
        .unwrap();
    let execute_reply = ctx.client_shell.recv_multipart(0).unwrap();
    let _execute_reply = Message::decode_from_multipart(execute_reply);

    for msg_type in [
        MsgType::status,
        MsgType::execute_input,
        MsgType::execute_result,
        MsgType::display_data,
        MsgType::status,
    ] {
        let resp = ctx.client_iopub.recv_multipart(0).unwrap();
        let resp = Message::decode_from_multipart(resp);
        assert_eq!(resp.header.msg_type, msg_type);
        // dbg!(resp);
    }
}

#[test]
fn aa() {
    let code = "#%matplotlib inline
import d2lzh as d2l
from mxnet import image";
    let code = code.replace(
        "catdog.jpg",
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/test_kernel/test_sample.png"
        ),
    );

    let ctx = TestContext::new();
    let msg = Message::execute_req(&code);
    ctx.client_shell
        .send_multipart(msg.encode_to_multipart(), 0)
        .unwrap();
    let execute_reply = ctx.client_shell.recv_multipart(0).unwrap();
    let _execute_reply = Message::decode_from_multipart(execute_reply);

    for msg_type in [MsgType::status, MsgType::execute_input, MsgType::status] {
        let resp = ctx.client_iopub.recv_multipart(0).unwrap();
        let resp = Message::decode_from_multipart(resp);
        assert_eq!(resp.header.msg_type, msg_type);
        // dbg!(resp);
    }

    let code = "#%matplotlib inline
d2l.set_figsize()
img = image.imread('catdog.jpg').asnumpy()
d2l.plt.imshow(img);
#d2l.plt.show(img)";
    let code = code.replace(
        "catdog.jpg",
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/test_kernel/test_sample.png"
        ),
    );

    let msg = Message::execute_req(&code);
    ctx.client_shell
        .send_multipart(msg.encode_to_multipart(), 0)
        .unwrap();
    let execute_reply = ctx.client_shell.recv_multipart(0).unwrap();
    let _execute_reply = Message::decode_from_multipart(execute_reply);

    // code `img.show();` without ExecuteResult message
    for msg_type in [
        MsgType::status,
        MsgType::execute_input,
        MsgType::execute_result,
        MsgType::display_data,
        MsgType::status,
    ] {
        let resp = ctx.client_iopub.recv_multipart(0).unwrap();
        let resp = Message::decode_from_multipart(resp);
        assert_eq!(resp.header.msg_type, msg_type);
        // dbg!(resp);
    }
}
