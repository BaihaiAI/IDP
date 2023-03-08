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

const REPO: &str = "https://pypi.tuna.tsinghua.edu.cn/simple";
const DEPLOYMENT_ENTRYPOINT: &str =
    r#"ENTRYPOINT ["supervisord", "-c", "/etc/supervisor/supervisord.conf", "--nodaemon"]"#;
const CRONJOB_ENTRYPOINT: &str = r#"ENTRYPOINT ["/bin/model_execute_services_batch",">","/var/log/model/model_execute_services_api.log"]"#;

use business::path_tool::*;
use tokio::io::AsyncWriteExt;

use super::*;

pub async fn build_image(
    edition: &EditionInfo,
    service_id: i32,
    region: &str,
    pg_pool: &sqlx::PgPool,
    service_type: i32,
    tag: Option<String>,
) -> Result<String, ErrorTrace> {
    tracing::info!("run build_image");
    let mut dockerfile = format!("FROM uhub.service.ucloud.cn/idp-saas/{}\n", edition.image);

    let base_path = if !edition.location.ends_with('/') {
        get_store_model_file_path(edition.team_id, edition.project_id, &edition.location).await
    } else {
        get_store_full_path(
            edition.team_id as u64,
            edition.project_id as u64,
            &edition.location,
        )
    };
    let dest_path = get_store_model_dist_path(service_id).await;
    let dest_path_tmp = get_store_model_dist_path_tmp(service_id).await;

    let model_deploy_path = get_store_deploy_model_file_path(service_id).await;
    let dockerfile_path = model_deploy_path.join("Dockerfile");
    let unzip_path = model_deploy_path.join("model");
    if !std::path::Path::new(&unzip_path).exists() {
        std::fs::create_dir_all(&unzip_path).unwrap();
    }

    let mut is_zip = false;
    if let Some(extension) = base_path.extension() {
        if extension == "zip" {
            is_zip = true;
        }
    }

    if is_zip {
        tracing::info!(
            "is_zip base_path{:?},unzip_path:{:?}",
            base_path,
            unzip_path
        );
        let mut cmd = tokio::process::Command::new("unzip");
        cmd.arg("-q")
            .arg("-o")
            .arg(&base_path)
            .arg("-d")
            .arg(&unzip_path);
        cmd.output().await?;
    } else {
        tracing::info!(
            "not_zip base_path{:?},unzip_path:{:?}",
            base_path,
            unzip_path
        );
        let mut cmd = tokio::process::Command::new("cp");
        cmd.arg("-r").arg("-f").arg(&base_path).arg(&unzip_path);
        cmd.output().await?;
    }

    let cp_command = format!("COPY model {}\nCOPY model {}\n", dest_path, dest_path_tmp);
    dockerfile += &cp_command;

    let requirements_filename = get_deploy_requirement_path(service_id).await;

    let runtime_env = SupportConstant::get_enum(&edition.runtime_env)?;

    if runtime_env.use_python() && requirements_filename.exists() {
        let pip_path = runtime_env.bin_path();
        //pip --no-cache-dir  install -r requirements.txt -i https://pypi.tuna.tsinghua.edu.cn/simple
        let install_requirements_command = format!(
            "COPY model/requirements.txt /tmp/requirements.txt\nRUN {pip_path} --no-cache-dir  install -r /tmp/requirements.txt -i {REPO}\n"
        );
        dockerfile += &install_requirements_command;
    }

    if service_type == 2 {
        dockerfile += CRONJOB_ENTRYPOINT;
    } else {
        dockerfile += DEPLOYMENT_ENTRYPOINT;
    }

    let mut image_name = format!("model_service_{}", service_id);
    let image_tar = format!("model_service_{}.tar", service_id);
    let image_tar_path = unzip_path.join(&image_tar);
    let mut f = tokio::fs::File::create(&dockerfile_path).await?;
    f.write_all(dockerfile.as_bytes()).await?;

    let tag = match tag {
        Some(tag) => tag,
        None => "stable".to_owned(),
    };

    image_name = format!("{image_name}:{tag}");
    post_kaniko_image(
        dockerfile_path.to_str().unwrap(),
        image_tar_path.to_str().unwrap(),
        edition.team_id,
        region,
        &image_name,
    )
    .await?;

    sqlx::query(
        "
    UPDATE idp_model_deploy_prod_service SET image=$1 WHERE id = $2
        ",
    )
    .bind(&image_name)
    .bind(service_id)
    .execute(pg_pool)
    .await?;

    Ok(image_name)
}

async fn post_kaniko_image(
    docker_file_path: &str,
    image_tar_path: &str,
    team_id: i64,
    region: &str,
    image_name: &str,
) -> Result<(), ErrorTrace> {
    let client = reqwest::ClientBuilder::new().build().unwrap();
    let body = KanikoImageBody {
        team_id: team_id.to_string(),
        action: "image",
        service: "kaniko",
        region: region.to_string(),
        docker_file_path: docker_file_path.to_string(),
        image_tar_path: image_tar_path.to_string(),
        image_name: image_name.to_string(),
    };
    tracing::info!("{:?}", body);
    let resp = client
        .post(format!("{K8S_SERVICE_API_BASE_URL}/kaniko-image"))
        .json(&body)
        .send()
        .await?;
    tracing::info!("{K8S_SERVICE_API_BASE_URL}/kaniko-image end");
    let text = resp.text().await?;
    tracing::info!("kaniko image:{text}");
    Ok(())
}
