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

use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[cfg(FALSE)]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[allow(non_camel_case_types)]
pub enum MsgType {
    execute_request,
    execute_input,
    /// e.g. pandas df
    execute_result,
    execute_reply,
    display_data,
    status,
    /// stdout or stderr
    stream,
    error,

    update_parent_header,
    /// idp-note custom message type
    runtime_error,
    reply_on_stop,
    interrupt_reply,
    // state: "[{\"state\":\"RUNNING\",\"cellId\":\"870100d7-a4cd-4415-a357-5f2087326922\"}]"
    kernel_state,
    /// {"duration": 8}
    duration,
    /// same as execute_input in jupyter
    reply_on_receive,
    error_on_receive,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "msgType", content = "content", rename_all = "snake_case")]
pub enum Content {
    ExecuteRequest(ExecuteRequest),
    // #[serde(rename_all = "camelCase")]
    ExecuteInput {
        code: String,
        /// nth run after kernel start, display in left side of cell e.g. `[1]`
        execution_count: u32,
    },
    /// execute_reply: frontend used to update execute_count
    ExecuteReply(ExecuteReply),

    ShutdownRequest {
        restart: bool,
    },
    InterruptRequest,

    Status {
        execution_state: ExecutionState,
    },

    InputRequest {
        prompt: String,
        password: bool,
    },
    InputReply {
        value: String,
    },

    // output_type
    Error(Error),
    Stream {
        /// name is stdout or stderr
        name: String,
        text: String,
        is_busy: bool,
    },
    DisplayData {
        data: std::collections::HashMap<String, String>,
    },
    ExecuteResult {
        data: std::collections::HashMap<String, String>,
    },

    // ShutdownRequest { restart: bool },
    // InterruptRequest {},

    // idp-kernel custom
    UpdateLastReq {},
    // kernel_state is unused
    // KernelState { state: String },
    Duration {
        code: String,
        run_at: u64,
        duration: u32,
    },
    // stop all
    ReplyOnStop {},
    /// kernel_manage error
    RuntimeError {
        message: String,
    },
    StartKernel {},
    Pong {
        client_id: u128,
    },
    // idp_kernel OOM killed or core dumped
    // KernelCoreDumped {
    // error_reason: String,
    // },
}

impl Content {
    pub fn warp_to_cell_output(&self) -> Option<serde_json::Map<String, Value>> {
        fn warp_display_data_or_execute_result(
            data: &std::collections::HashMap<String, String>,
            output_type: &str,
        ) -> serde_json::Map<String, Value> {
            let mut datamap = serde_json::Map::new();
            for (k, v) in data.iter() {
                datamap.insert(
                    k.clone(),
                    Value::Array(
                        v.split_inclusive(r"\n")
                            .map(|s| Value::String(s.to_string()))
                            .collect::<Vec<_>>(),
                    ),
                );
            }
            let mut map = serde_json::Map::new();
            map.insert("data".to_string(), Value::Object(datamap));
            map.insert(
                "output_type".to_string(),
                Value::String(output_type.to_string()),
            );
            map
        }
        let val = match self {
            Content::RuntimeError { message } => {
                return Self::warp_to_cell_output(&Content::Error(Error {
                    ename: if message.contains("out of memory") {
                        "OutOfMemory".to_string()
                    } else {
                        "RuntimeError".to_string()
                    },
                    evalue: message.to_string(),
                    traceback: vec![],
                }));
            }
            Content::Error(err) => {
                let err = serde_json::to_value(&err).unwrap();
                let mut map = err.as_object().unwrap().to_owned();
                map.insert(
                    "output_type".to_string(),
                    Value::String("error".to_string()),
                );
                map
            }
            Content::Stream {
                name,
                text,
                is_busy: true,
            } => {
                let mut map = serde_json::Map::new();
                map.insert(
                    "output_type".to_string(),
                    Value::String("stream".to_string()),
                );
                map.insert("name".to_string(), Value::String(name.clone()));
                map.insert(
                    "text".to_string(),
                    Value::Array(
                        text.split_inclusive(r"\n")
                            .map(|s| Value::String(s.to_string()))
                            .collect::<Vec<_>>(),
                    ),
                );
                map
            }
            Content::DisplayData { data } => {
                warp_display_data_or_execute_result(data, "display_data")
            }
            Content::ExecuteResult { data } => {
                warp_display_data_or_execute_result(data, "execute_result")
            }
            _ => return None,
        };
        Some(val)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(deny_unknown_fields)]
pub struct ExecuteRequest {
    pub code: String,
    pub silent: bool,
    pub store_history: bool,
    pub user_expressions: Value,
    pub allow_stdin: bool,
    pub stop_on_error: bool,
    // IDP custom field
    pub enable_save_session: Option<bool>,
}

impl Default for ExecuteRequest {
    fn default() -> Self {
        Self {
            code: "".to_string(),
            silent: false,
            store_history: true,
            user_expressions: serde_json::Value::Object(serde_json::Map::new()),
            allow_stdin: false,
            stop_on_error: true,
            enable_save_session: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionState {
    Busy,
    Idle,
    // Starting,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExecuteReply {
    /// common fields
    pub execution_count: u32,
    // pub payload: serde_json::Value,
    // pub user_expressions: serde_json::Value,
    /// extra fields when error
    #[serde(flatten)]
    pub reply_status: ReplyStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "status")]
pub enum ReplyStatus {
    Ok,
    Error(Error),
}

#[test]
fn test_execute_reply_deserialize() {
    const OK_JSON: &str = r#"{
        "status": "ok",
        "execution_count": 3,
        "user_expressions": {},
        "payload": []
    }"#;
    const ERR_JSON: &str = r#"{
        "status": "error",
        "execution_count": 2,
        "user_expressions": {},
        "payload": [],

        "traceback": [
          "\u001b[0;36m  Input \u001b[0;32mIn [2]\u001b[0;36m\u001b[0m\n\u001b[0;31m    print(1.eq(2))\u001b[0m\n\u001b[0m           ^\u001b[0m\n\u001b[0;31mSyntaxError\u001b[0m\u001b[0;31m:\u001b[0m invalid decimal literal\n"
        ],
        "ename": "SyntaxError",
        "evalue": "invalid decimal literal (1616495749.py, line 1)",

        "engine_info": {
          "engine_uuid": "49180d62-077c-4ce2-a9e4-54a78799fb99",
          "engine_id": -1,
          "method": "execute"
        }
    }"#;
    serde_json::from_str::<ExecuteReply>(OK_JSON).unwrap();
    serde_json::from_str::<ExecuteReply>(ERR_JSON).unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Ok,
    Error,
    // #[deprecated]
    // Abort,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Error {
    /// e.g. SyntaxError
    pub ename: String,
    pub evalue: String,
    pub traceback: Vec<String>,
}

/// e.g. pandas dataframe
#[derive(Serialize, Deserialize, Debug)]
pub struct ExecuteResult {
    /// e.g. text/html
    pub data: std::collections::HashMap<String, String>,
    pub execution_count: u32,
}

/// e.g. matplotlib
#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayData {
    pub data: std::collections::HashMap<String, String>,
}

#[cfg(FALSE)]
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Content {
    ExecuteRequest(ExecuteRequest),
    Status(ExecutionStateStatus),
    ExecuteInput(ExecuteInput),
    ExecuteResult(ExecuteResult),
    DisplayData(DisplayData),
    Stream(Stream),
    ExecuteReply(ExecuteReply),
    Error(Error),
    Shutdown(Shutdown),
    PreParentHeader,
}

#[cfg(FALSE)]
// deserialize by msg_type
impl Content {
    pub(crate) fn deserialize_by_msg_type(
        msg_type: &MsgType,
        bytes: &[u8],
    ) -> Result<Self, serde_json::Error> {
        let ret = match msg_type {
            MsgType::execute_input => Self::ExecuteInput(from_slice::<ExecuteInput>(bytes)?),
            MsgType::ExecuteRequest => Self::ExecuteRequest(from_slice::<ExecuteRequest>(bytes)?),
            MsgType::execute_result => Self::ExecuteResult(from_slice::<ExecuteResult>(bytes)?),
            MsgType::ExecuteReply => Self::ExecuteReply(from_slice::<ExecuteReply>(bytes)?),
            MsgType::display_data => Self::DisplayData(from_slice::<DisplayData>(bytes)?),
            MsgType::status => Self::Status(from_slice::<ExecutionStateStatus>(bytes)?),
            MsgType::stream => Self::Stream(from_slice::<Stream>(bytes)?),
            MsgType::error => Self::Error(from_slice::<Error>(bytes)?),
            MsgType::ShutdownRequest | MsgType::ShutdownReply => {
                Self::Shutdown(serde_json::from_slice::<Shutdown>(bytes)?)
            }
            _ => {
                panic!("unsupported msg_type")
            }
        };
        Ok(ret)
    }
}
