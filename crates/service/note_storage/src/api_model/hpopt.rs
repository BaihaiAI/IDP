use business::business_term::ProjectId;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartHpOpt {
    pub db_type: Option<String>,
    pub db_name: Option<String>,
    pub project_id: ProjectId,
}