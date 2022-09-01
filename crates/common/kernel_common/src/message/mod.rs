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

pub mod content;
pub mod header;

use content::Content;
use header::Header;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    #[serde(flatten)]
    pub header: Header,
    /// reserve field for frontend, just echo this field value to frontend
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<serde_json::Value>,
    // pub parent_header: ParentHeader,
    // #[serde(skip_serializing)]
    // pub metadata: Vec<u8>,
    // T: Serialize + for<'de> Deserialize<'de> + Debug
    // #[serde(serialize_with = "ser_content")]
    #[serde(flatten)]
    pub content: Content,
}

impl Message {
    pub fn is_idle(&self) -> bool {
        if let Content::Status { execution_state } = &self.content {
            if matches!(execution_state, content::ExecutionState::Idle) {
                return true;
            }
        }
        false
    }
    pub fn is_error(&self) -> bool {
        matches!(
            self.content,
            Content::Error(_) | Content::RuntimeError { .. }
        )
    }

    // bincode not support serde[(flatten)]
    #[cfg(FALSE)]
    pub fn to_bincode(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn to_json(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
    pub fn from_json(bytes: &[u8]) -> Self {
        serde_json::from_slice(bytes).unwrap()
    }
}

#[test]
fn test_warp_to_cell_output() {
    // {"header": {"msg_id": "2a8eebae-c677e9237db71406e5313fbc_216327_45", "msg_type": "stream", "username": "w", "session": "2a8eebae-c677e9237db71406e5313fbc", "date": "2022-05-06T07:32:16.367693Z", "version": "5.3"}, "msg_id": "2a8eebae-c677e9237db71406e5313fbc_216327_45", "msg_type": "stream", "parent_header": {"date": "2022-05-06T07:32:16.363000Z", "msg_id": "1ecefd9b38d345c2911526652ac27733", "username": "username", "session": "4aed263cdf1849e58a03d3de23f7cece", "msg_type": "execute_request", "version": "5.2"}, "metadata": {}, "content": {"name": "stdout", "text": "1\n2\n3\n"}, "buffers": [], "channel": "iopub"}
    let msg = Message {
        content: Content::Stream {
            name: "stdout".to_string(),
            text: r"1\n2\n3\n".to_string(),
            is_busy: true,
        },
        ..Default::default()
    };
    assert_eq!(
        msg.content.warp_to_cell_output().unwrap(),
        serde_json::json!({
            "name": "stdout",
            "output_type": "stream",
            "text": [
                r"1\n",
                r"2\n",
                r"3\n"
            ]
        })
        .as_object()
        .unwrap()
        .to_owned()
    );
}

impl Default for Message {
    fn default() -> Self {
        Self {
            // peer_id: Vec::new(),
            header: Header::default(),
            // metadata: b"{}".to_vec(),
            content: content::Content::RuntimeError {
                message: "this is Message::default(), should be unreachable".to_string(),
            },
            request: None,
        }
    }
}
