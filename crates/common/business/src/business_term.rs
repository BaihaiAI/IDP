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

//! business term definitions
pub type TeamId = u64;
pub type ProjectId = u64;
pub type UserId = u64;

#[derive(Debug)]
pub struct ProjectFolder(&'static str);

#[derive(Debug)]
pub struct TeamFolder(&'static str);

/// $team_root=/store/$team_id
impl TeamFolder {
    pub const CONDA: Self = Self("miniconda3");
    pub const PROJECTS: Self = Self("projects");
    pub const EXTENSIONS: Self = Self("extensions");
    #[inline]
    pub(crate) const fn inner(&self) -> &'static str {
        self.0
    }
}

/// $project_root=/store/$team_id/projects/$project_id
impl ProjectFolder {
    pub const ROOT: Self = Self("/");
    pub const JOB: Self = Self("job");
    // pub const KERNEL: &'static str = "kernels";
    pub const PIPELINE: Self = Self("pipeline");
    pub const SNAPSHOT: Self = Self("snapshot");
    pub const TMP: Self = Self("tmp");
    pub const TRASH: Self = Self("trash");
    pub const NOTEBOOKS: Self = Self("notebooks");
    pub const MINICONDA3: Self = Self("miniconda3");
    pub const HPOPT_DATASOURCE: Self = Self("hpopt_datasource");

    #[inline]
    pub const fn inner(&self) -> &'static str {
        self.0
    }
}
