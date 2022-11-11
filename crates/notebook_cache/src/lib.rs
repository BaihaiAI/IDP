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

/*
// wrap with Arc+Mutex
type CacheEntryKey = (project_id, abs_path);
struct Cache {
    notebook_expire_delay_queue: Arc<Mutex<DelayQueue>>,
    /// why we don't use inode: each time write to NFS would create new temp file, when write success then mv temp file to target file
    notebook_cache_entries: HashMap<CacheEntryKey, (Notebook, delay_queue_key)>
}
Cache::new() would spawn a delay_queue consumer coroutine

## private persist_cache_to_fs(key: CacheEntryKey)
*/

type ProjectId = u64;
type NotebookRelativePath = String;
type NotebookCacheKey = (ProjectId, NotebookRelativePath);
type CellId = String;
struct NotebookFsCache {
    expire_delay_queue: tokio_util::time::DelayQueue<NotebookCacheKey>,
    notebooks: std::collections::HashMap<NotebookCacheKey, ()>,
}
