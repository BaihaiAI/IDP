use business::business_term::ProjectId;
use serde::Deserialize;

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
pub struct DatasourceListReq{
    pub project_id: ProjectId,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasourceNewReq{
    pub project_id: ProjectId,
    pub db_name: String,
}

