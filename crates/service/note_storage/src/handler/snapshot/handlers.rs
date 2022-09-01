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

use std::path::PathBuf;

use cache_io::snapshot_key;
use cache_io::CacheService;
use cache_io::SnapshotRedisListItem;
use common_model::entity::notebook::Notebook;
use err::ErrorTrace;

use super::diff::diff_notebook1_notebook2;
use super::models::SnapshotDiffRes;
use super::models::SnapshotListItem;

/////////// /snapshot ///////////
pub async fn snapshot(
    label: &str,
    file_full_path: &PathBuf,
    redis_cache: &CacheService,
    project_id: u64,
) -> Result<Vec<SnapshotListItem>, ErrorTrace> {
    let path = file_full_path.to_str().unwrap();
    let project_id_u64 = project_id;
    let notebook = redis_cache
        .read_notebook(&file_full_path, project_id_u64)
        .await?;
    let notebook = serde_json::to_string(&notebook)?;
    let path_string = file_full_path.to_str().unwrap();

    snapshot_insert(path_string, label, &notebook, redis_cache, project_id_u64).await?;
    snapshot_list(path, redis_cache, project_id_u64).await
}

pub async fn snapshot_insert(
    path: &str,
    label: &str,
    content: &str,
    redis_cache: &CacheService,
    project_id: u64,
) -> Result<(), ErrorTrace> {
    let key = snapshot_key(path, project_id);
    let now = chrono::Local::now();
    let timestamp = now.timestamp();
    let snap = SnapshotRedisListItem {
        id: timestamp as u64,
        label: label.to_string(),
        time: now.naive_local().to_string(),
        content: content.to_string(),
    };
    redis_cache.snapshot_insert(&key, snap).await?;
    Ok(())
}

// wanted format `2022-06-18 07:34:28.150690440`
#[test]
fn test_dt_format() {
    let now = chrono::Utc::now();
    println!("{}", now.naive_local());
}

/////////// /snapshot/list ///////////
pub async fn snapshot_list(
    path: &str,
    redis_cache: &CacheService,
    project_id: u64,
) -> Result<Vec<SnapshotListItem>, ErrorTrace> {
    let key = snapshot_key(path, project_id);
    Ok(redis_cache
        .snapshot_list(&key)
        .await?
        .into_iter()
        .map(|snap| SnapshotListItem {
            id: snap.id.to_string(),
            label: snap.label,
            // time: snap.time.naive_local().to_string(),
            time: snap.time,
        })
        .collect())
}

/////////// /snapshot/restore ///////////
pub async fn snapshot_restore(
    path: &str,
    id: u64,
    redis_cache: &CacheService,
    project_id: u64,
) -> Result<super::models::SnapshotRestoreRes, ErrorTrace> {
    let key = snapshot_key(path, project_id);
    let list = redis_cache.snapshot_list(&key).await?;

    match list.into_iter().find(|item| item.id == id) {
        Some(snap) => {
            let notebook = serde_json::from_str::<Notebook>(&snap.content)?;
            #[cfg(feature = "redis")]
            redis_cache
                .update_notebook(path, notebook, project_id)
                .await?;
            #[cfg(not(feature = "redis"))]
            redis_cache.update_notebook(path, notebook).await?;
            Ok(super::models::SnapshotRestoreRes {
                id: id.to_string(),
                label: snap.label,
                path: path.to_string(),
            })
        }
        None => Err(ErrorTrace::new("Snapshot id not found")),
    }
}

/////////// /snapshot/diff ///////////
pub async fn snapshot_diff(
    id1: u64,
    id2: u64,
    path: String,
    redis_cache: &CacheService,
    project_id: u64,
) -> err::Result<SnapshotDiffRes> {
    tracing::info!("snapshot_diff: {} {} {}", id1, id2, path);
    if id1 == id2 {
        let err_msg = format!("id1==id2=={id1}, please select two different snapshots version");
        return Err(ErrorTrace::new(&err_msg));
    }
    let key = snapshot_key(&path, project_id);
    let start = std::time::Instant::now();
    let list = redis_cache.snapshot_list(&key).await?;
    tracing::info!(
        "snapshot_diff: after snapshot list time cost {:?}, key = {key}, list.len() = {}",
        start.elapsed(),
        list.len()
    );
    let snap1 = match list.iter().find(|snap| snap.id == id1) {
        Some(snap1) => snap1,
        None => {
            return Err(ErrorTrace::new("id1 not found"));
        }
    };
    let notebook1 = serde_json::from_str::<Notebook>(&snap1.content)?;
    let snap2 = match list.iter().find(|snap| snap.id == id2) {
        Some(snap1) => snap1,
        None => {
            return Err(ErrorTrace::new("id2 not found"));
        }
    };
    let notebook2 = serde_json::from_str::<Notebook>(&snap2.content)?;
    let handle = tokio::task::spawn_blocking(|| diff_notebook1_notebook2(notebook1, notebook2));

    match handle.await {
        Ok(res) => Ok(res?),
        Err(_) => {
            tracing::warn!("snapshot {key}");
            Err(ErrorTrace::new("diff err"))
        }
    }
    // Ok(diff_notebook1_notebook2(notebook1, notebook2)?)
}
