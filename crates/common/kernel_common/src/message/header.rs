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

use business::business_term::ProjectId;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    // pub msg_id: String,
    #[serde(serialize_with = "serde_helper::ser_u64_to_string")]
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub project_id: ProjectId,
    #[serde(serialize_with = "serde_helper::ser_u64_to_string")]
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub team_id: u64,
    pub cell_id: String,
    /// running/idle kernel can't rename, move and delete
    pub path: String,

    pub pipeline_opt: Option<Pipeline>,
}

pub fn kernel_header_hash(project_id: u64, path: &str, task_instance_id: i64) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    // std::hash::Hash::hash(&team_id, &mut hasher);
    std::hash::Hash::hash(&project_id, &mut hasher);
    std::hash::Hash::hash(&path, &mut hasher);
    std::hash::Hash::hash(&task_instance_id, &mut hasher);
    std::hash::Hasher::finish(&hasher)
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct Pipeline {
    #[serde(serialize_with = "serde_helper::ser_u64_to_string")]
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub job_id: u64,
    /// one pipeline job has multi task_instance
    #[serde(serialize_with = "serde_helper::ser_u64_to_string")]
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub job_instance_id: u64,
    #[serde(serialize_with = "serde_helper::ser_u64_to_string")]
    #[serde(deserialize_with = "serde_helper::de_u64_from_str")]
    pub task_instance_id: u64,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            path: "".to_string(),
            project_id: 0,
            team_id: 0,
            cell_id: "".to_string(),
            pipeline_opt: None,
        }
    }
}

impl Header {
    fn hash_to_u64(&self) -> u64 {
        let task_instance_id = match self.pipeline_opt {
            Some(ref pipeline) => pipeline.task_instance_id as i64,
            None => -1,
        };
        kernel_header_hash(self.project_id, &self.path, task_instance_id)
    }

    pub fn ipynb_abs_path(&self) -> std::path::PathBuf {
        let pipeline = match self.pipeline_opt {
            Some(ref pipeline) => pipeline,
            None => {
                return business::path_tool::get_store_full_path(
                    self.team_id,
                    self.project_id,
                    &self.path,
                );
            }
        };
        tracing::info!("ipynb_abs_path: is pipeline, header={self:?}");
        if self.project_id > 1000000 {
            tracing::error!("invalid project_id {}", self.project_id);
        }
        let dst_path = business::path_tool::get_pipeline_output_path(
            self.team_id,
            self.project_id,
            &self.path,
            pipeline.job_id,
            pipeline.job_instance_id,
            pipeline.task_instance_id,
            true,
        )
        .unwrap();
        tracing::info!("dst_path = {dst_path:?}");
        if !dst_path.exists() {
            let dir = dst_path.parent().unwrap();
            if let Err(err) = std::fs::create_dir_all(dir) {
                tracing::error!("{err}");
            }
            // copy from pipeline input dir
            let src_path = get_pipeline_run_input_path(
                self.team_id,
                self.project_id,
                &self.path,
                pipeline.job_id,
                pipeline.job_instance_id,
                pipeline.task_instance_id,
            );
            tracing::info!("src_path = {src_path:?}");
            if let Err(err) = std::fs::copy(&src_path, &dst_path) {
                tracing::error!("src_path={src_path:?} dst_path={dst_path:?} {err}");
            }
        }
        dst_path
    }
    pub fn inode(&self) -> u64 {
        if self.pipeline_opt.is_some() {
            self.ipynb_abs_path();
            self.hash_to_u64()
        } else {
            self.hash_to_u64()
        }
    }
}

fn get_pipeline_run_input_path(
    team_id: u64,
    project_id: u64,
    relative_path: &str,
    job_id: u64,
    job_instance_id: u64,
    task_instance_id: u64,
) -> std::path::PathBuf {
    let pipeline_root_path = business::path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::PIPELINE,
    );
    // "/store/1519596781073219584/projects/104/pipeline/80-400-770/1.ipynb"

    //using job_id,instance_id,task_id splice the source file directory path
    let source_directory_name = format!("{}-{}-{}", job_id, job_instance_id, task_instance_id);
    pipeline_root_path
        .join(source_directory_name)
        .join(relative_path.strip_prefix('/').unwrap())
}

#[cfg(FAlSE)]
fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: serde::de::Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

/// jupyter message request parent_header must empty
/// jupyter message response parent_header must some
#[cfg(FALSE)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ParentHeader {
    Some(Header),
    EmptyObject {},
}

#[cfg(FALSE)]
#[test]
fn test_deserialize_parent_header() {
    serde_json::from_str::<ParentHeader>("{}").unwrap();
}
