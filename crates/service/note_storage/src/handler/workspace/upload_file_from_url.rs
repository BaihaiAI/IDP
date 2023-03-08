// Copyright 2023 BaihaiAI, Inc.
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

use axum::extract::Json;
use axum::response::sse::Event;
use axum::response::sse::KeepAlive;
use axum::response::sse::Sse;
use err::ErrorTrace;
use futures::Stream;
use futures::StreamExt;
use tokio::io::AsyncWriteExt;
use tracing::error;

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Req {
    url: String,
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    team_id: u64,
    project_id: u64,
    /// upload to dir
    file_path: String,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct StreamRspItem {
    saved_bytes: usize,
    total_bytes: u64,
    error: String,
}

impl StreamRspItem {
    fn to_sse_event(self) -> Event {
        Event::default().data(serde_json::to_string(&self).unwrap())
    }
}

pub async fn upload_from_url(
    req: Json<Req>,
) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let stream = match upload_file_from_url_handler_inner(req).await {
        Ok(x) => x.left_stream(),
        Err(err) => futures::stream::once(async move {
            StreamRspItem {
                saved_bytes: 0,
                total_bytes: 0,
                error: err.to_string(),
            }
            .to_sse_event()
        })
        .map(Ok)
        .right_stream(),
    };
    Sse::new(stream).keep_alive(KeepAlive::new())
}

async fn upload_file_from_url_handler_inner(
    Json(Req {
        url,
        team_id,
        project_id,
        file_path: upload_to_dir,
    }): Json<Req>,
) -> Result<impl Stream<Item = Result<Event, std::io::Error>>, ErrorTrace> {
    let url = url.parse::<reqwest::Url>()?;
    if url.scheme() != "http" && url.scheme() != "https" {
        return Err(ErrorTrace::new("only support http url"));
    }
    // http url must has path_segments
    let filename = url.path_segments().unwrap().last().unwrap();
    let filename = if filename.is_empty() {
        url.host().unwrap().to_string()
    } else {
        filename.to_string()
    };
    let base_path = business::path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::NOTEBOOKS,
    );
    let base_path = base_path.to_str().unwrap();
    // concat str prevent base_path.join("/") would cd to root
    let save_dir = format!("{base_path}/{upload_to_dir}");
    let save_path = crate::handler::workspace::decompress::rename_path_if_path_exist(
        std::path::Path::new(&save_dir).join(&filename),
    );

    let client = reqwest::ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .connect_timeout(std::time::Duration::from_secs(5))
        .build()?;
    tracing::info!("{url} -> {save_path:?}");
    let rsp = client.get(url).send().await?;
    let status = rsp.status();
    if !status.is_success() {
        return Err(ErrorTrace::new(&format!("request fail {status}")));
    }

    let content_length = match rsp.content_length() {
        Some(x) => x,
        None => return Err(ErrorTrace::new("response no content length")),
    };

    let init_stream = futures::stream::once(async move {
        Ok::<_, std::io::Error>(
            StreamRspItem {
                saved_bytes: 0,
                total_bytes: content_length,
                error: String::new(),
            }
            .to_sse_event(),
        )
    });

    let stream =
        futures::StreamExt::scan(rsp.bytes_stream(), 0usize, |saved_bytes, incoming_res| {
            let incoming = match incoming_res {
                Ok(incoming) => incoming,
                Err(err) => {
                    error!("{err}");
                    return futures::future::ready(None);
                }
            };
            *saved_bytes += incoming.len();
            futures::future::ready(Some((incoming.to_vec(), *saved_bytes)))
        })
        .then(move |(incoming, saved_bytes)| {
            let save_path = save_path.clone();
            async move {
                let mut file = tokio::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(save_path)
                    .await?;
                file.write_all(&incoming).await?;
                Ok::<_, std::io::Error>(
                    StreamRspItem {
                        saved_bytes,
                        total_bytes: content_length,
                        error: String::new(),
                    }
                    .to_sse_event(),
                )
            }
        });
    Ok(init_stream.chain(stream))
}
