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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartHpOptReq {
    pub db_type: Option<String>,
    pub db_name: Option<String>,
    pub project_id: ProjectId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StopHpOptReq {
    pub db_name: String,
    pub project_id: ProjectId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasourceListReq {
    pub project_id: ProjectId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasourceNewReq {
    pub project_id: ProjectId,
    pub db_name: String,
}

type StudyId = i64;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudyDetailReq {
    pub study_id: StudyId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteStudyReq {
    pub study_id: StudyId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudyObjectiveCodeReq {
    pub study_id: StudyId,
    pub project_id: ProjectId,
    pub db_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StudyNewReq {
    pub objective_content: String,
    pub project_id: ProjectId,
    pub study_name: String,
    pub db_name: String,
    pub directions: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectiveContentReq {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditStudyCodeReq {
    pub content: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptRunReq {
    pub project_id: ProjectId,
    pub study_id: StudyId,
    pub study_name: String,
    pub db_name: String,
    pub n_trials: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptStateReq {
    pub opt_state_key: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatLogReq {
    pub project_id: ProjectId,
    pub db_name: String,
    pub study_id: StudyId,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasourceResp {
    pub name: String,
    pub status: String,
    pub port: Option<u16>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StudyObjectiveCodeResp {
    pub objective_content: String,
    pub full_file_path: String,
}
