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

pub mod file_writer {
    use std::collections::BTreeMap;
    use std::collections::HashMap;
    use std::collections::VecDeque;
    use std::path::PathBuf;

    use err::ErrorTrace;
    use tokio::io::AsyncWriteExt;
    use tokio::sync::mpsc;
    use tokio::sync::oneshot;

    type Req = (FileChunk, oneshot::Sender<i64>);
    pub type FileSender = mpsc::Sender<Req>;

    pub fn init() -> (FileState, mpsc::Receiver<Req>) {
        mpsc::channel::<Req>(200);
        let (writer_sender, writer_receiver) = mpsc::channel(200);

        let state = FileState {
            writer: writer_sender,
        };

        (state, writer_receiver)
    }

    #[derive(Debug)]
    pub struct FileChunk {
        pub file_dir: String,
        pub file_idx: u64,
        pub total_chunk: u64,
        pub file_data: Vec<u8>,
        // is_restart: bool,
        // restart_idx: Option<u64>,
    }

    pub struct FileStatus {
        current_idx: u64,
        current_size: u64,
        total_chunk: u64,
        file_data: BTreeMap<u64, Vec<u8>>,
        last_update: chrono::DateTime<chrono::Utc>,
        // is_restart: bool
    }

    #[derive(Clone)]
    pub struct FileState {
        pub writer: FileSender,
    }

    // async receive file chunk and write to file
    pub async fn file_writer_recv_thread(mut file_chunks: mpsc::Receiver<Req>) {
        let mut chunks_map: HashMap<String, FileStatus> = HashMap::new();
        let mut keys = VecDeque::new();

        while let Some((file_chunk, responder)) = file_chunks.recv().await {
            let file_dir = file_chunk.file_dir.clone();
            match file_writer_handler(&mut chunks_map, &mut keys, file_chunk).await {
                Ok(idx) => {
                    if responder.send(idx as i64).is_err() {
                        tracing::error!("{file_dir} file_writer_thread send back fail");
                    }
                }
                Err(err) => {
                    tracing::error!("{file_dir} {err:#?}");
                    if responder.send(-1).is_err() {
                        tracing::error!("{file_dir} file_writer_thread send back fail");
                    }
                }
            }
        }
        tracing::error!("file_writer receiver channel should not close");
    }

    async fn file_writer_handler(
        chunks_map: &mut HashMap<String, FileStatus>,
        keys: &mut VecDeque<String>,
        file_chunk: FileChunk,
        // responder: oneshot::Sender<i64>,
    ) -> Result<u64, err::ErrorTrace> {
        let mut expired_files = Vec::new();
        chunks_map.iter().for_each(|(k, v)| {
            if (chrono::Utc::now() - v.last_update).num_minutes() > 60 {
                expired_files.push(k.clone());
            }
        });
        expired_files.into_iter().for_each(|k| {
            chunks_map.remove(&k);
            let _idx = keys.iter().position(|x| x == &k).unwrap();
            keys.remove(_idx);
        });

        let file_status = match chunks_map.get_mut(&file_chunk.file_dir) {
            Some(file_status) => {
                file_status
                    .file_data
                    .insert(file_chunk.file_idx, file_chunk.file_data);

                file_status
            }
            None => {
                let mut _tmp_data = BTreeMap::new();
                _tmp_data.insert(file_chunk.file_idx, file_chunk.file_data);

                let file_status = FileStatus {
                    current_idx: 0,
                    current_size: 0,
                    total_chunk: file_chunk.total_chunk,
                    file_data: _tmp_data,
                    last_update: chrono::Utc::now(),
                };
                chunks_map.insert(file_chunk.file_dir.clone(), file_status);
                keys.push_back(file_chunk.file_dir.clone());

                let f_name = PathBuf::from(file_chunk.file_dir.clone());
                tokio::fs::create_dir_all(&f_name.parent().unwrap_or(&PathBuf::from("/"))).await?;
                tokio::fs::File::create(&f_name).await?;
                chunks_map.get_mut(&file_chunk.file_dir).unwrap()
            }
        };

        let file_path = std::path::Path::new(&file_chunk.file_dir);
        if let Some(parent_dir) = file_path.parent() {
            tokio::fs::create_dir_all(parent_dir)
                .await
                .map_err(|err| ErrorTrace::new(&format!("{:?} {err}", parent_dir)))?;
        }

        let mut file = tokio::fs::OpenOptions::new()
            .append(true)
            .create(false)
            .open(&file_chunk.file_dir)
            .await?;

        while let Some(data) = file_status.file_data.remove(&file_status.current_idx) {
            file.write_all(&data).await?;
            file_status.current_idx += 1;
            file_status.current_size += data.len() as u64;
            file_status.last_update = chrono::Utc::now();
        }

        // if let Err(idx) = responder.send(file_status.current_idx as i64) {
        //     let err = format!("send idx {idx} back fail");
        //     return Err(err::ErrorTrace::new(&err));
        // }

        let ret = file_status.current_idx;
        if file_status.current_idx == file_status.total_chunk {
            // dbg!("file write finished");
            chunks_map.remove(&file_chunk.file_dir);
            let _idx = keys.iter().position(|x| x == &file_chunk.file_dir).unwrap();
            keys.remove(_idx);
        }
        Ok(ret)
    }
}
