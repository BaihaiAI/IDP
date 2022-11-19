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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StudyObjectiveCodeResp {
    pub objective_content: String,
    pub full_file_path: String,
}
