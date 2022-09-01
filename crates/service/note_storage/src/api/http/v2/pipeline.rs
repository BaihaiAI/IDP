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
 * @Author: zqk@baihai.ai
 * @Date: 2022-04-21 09:21:44
 * @LastEditTime: 2022-04-22 11:26:22
 * @LastEditors: whitelilis
 * @Description:  pipeline
 * All rights reserved
 */
use std::fs;
use std::path::Path;

use axum::extract::Query;
use business::path_tool;
use business::path_tool::escape_path_as_string;
use business::path_tool::get_nbconvert_by_team_id;
use business::path_tool::get_pipeline_output_path;
use business::path_tool::get_store_full_path;
use common_model::entity::cell::CellType;
use common_model::enums::mime::Mimetype;
use common_model::service::rsp::Rsp;
use common_tools::cookies_tools::get_cookie_value_by_team_id;
use common_tools::file_tool;
use err::ErrorTrace;
use serde_json::json;
use tower_cookies::Cookies;
use tracing::debug;
use tracing::error;
use tracing::info;

// use crate::status_code::NB_RENAME_ERROR_CODE;
use crate::api_model::pipeline::PipelineCatResult;
use crate::api_model::pipeline::PipelineResultDto;
use crate::api_model::pipeline::PipelineResultRequest;
use crate::business_::path_tool::name_convert;
use crate::common::error::IdpGlobalError;
use crate::handler::content::cat::CatRsp;
use crate::handler::content::cat::CatRspBody;
use crate::handler::pipeline_handler;
use crate::status_code::LAST_MODIFIED_ERROR_CODE;
use crate::status_code::LAST_MODIFIED_ERROR_MSG;
use crate::status_code::PREVIEW_ERROR_CODE;
use crate::status_code::PREVIEW_ERROR_MSG;
use crate::status_code::REMOVE_ERROR_CODE;
use crate::status_code::REMOVE_ERROR_MSG;

fn path_exists(path: String) -> bool {
    std::fs::metadata(path).is_ok()
}

pub async fn cat_result(
    cookies: Cookies,
    Query(payload): Query<PipelineCatResult>,
) -> Result<Rsp<PipelineResultDto>, IdpGlobalError> {
    info!("access cat pipeline_result api.");

    let team_id = get_cookie_value_by_team_id(cookies);
    pipeline_handler::cat_result(
        team_id,
        payload.project_id,
        payload.path,
        payload.job_id,
        payload.job_instance_id,
        payload.task_instance_id,
        payload.start,
        payload.limit,
    )
    .await
}

//accountId=1520684197767442432-60-85-849646
#[cfg(not)]
pub async fn task_state(
    Query(qs): Query<HashMap<String, String>>,
    Extension(app_context): Extension<AppContext>,
) -> Result<Rsp<PipelineStatusDto>, IdpGlobalError> {
    let empty = String::from("1000000000000000000-00-00-000000");
    let account_id = qs.get("accountId").unwrap_or(&empty).to_string();
    tracing::info!("pipeline task_state account_id={:?}", account_id);
    let pipeline_key_tail = generate_state_key_tail(account_id);
    tracing::info!(
        "pipeline task_state pipeline_key_tail={:?}",
        pipeline_key_tail
    );
    pipeline_handler::task_state(app_context.redis_cache, pipeline_key_tail).await
}

#[cfg(not)]
pub fn generate_state_key_tail(account_id: String) -> String {
    let account_split = account_id.split('-');
    let x: Vec<&str> = account_split.collect();
    let mut key = x[1].to_string();
    let s_split = "-".to_string();
    let s2 = x[2].to_string();
    key += &s_split;
    key += &s2;
    key
}

pub fn path_to_use(template_path: String, task_path: String) -> String {
    if path_exists(task_path.clone()) {
        tracing::info!("##path_exists task_path={}", task_path);
        if let Some(true) = first_is_newer(task_path.clone(), template_path.clone()) {
            tracing::info!("## first_is_newer true,  task_path us newer");
            task_path
        } else {
            tracing::info!("## first_is_newer false, template_path us newer");
            template_path
        }
    } else {
        tracing::info!("## NOT path_exists task_path={}", task_path);
        template_path
    }
}

pub async fn task_result(
    cookies: Cookies,
    Query(payload): Query<PipelineResultRequest>,
) -> Result<Rsp<CatRsp>, ErrorTrace> {
    let project_id = payload.project_id;
    let browser_path = payload.path;
    let job_id = payload.job_id;
    let task_instance_id = payload.task_instance_id;
    let job_instance_id = payload.job_instance_id;

    let team_id = get_cookie_value_by_team_id(cookies);

    let is_idpnb = browser_path.ends_with(".ipynb") || browser_path.ends_with(".idpnb");

    let mut job_id_u64 = 0u64;
    if let Ok(p) = job_id.parse::<u64>() {
        job_id_u64 = p;
    }
    let mut job_instance_id_u64 = 0u64;
    if let Ok(p) = job_instance_id.parse::<u64>() {
        job_instance_id_u64 = p;
    }
    let mut task_instance_id_u64 = 0u64;
    if let Ok(p) = task_instance_id.parse::<u64>() {
        task_instance_id_u64 = p;
    }

    let task_file_path = get_pipeline_output_path(
        team_id,
        project_id,
        &browser_path,
        job_id_u64,
        job_instance_id_u64,
        task_instance_id_u64,
        is_idpnb,
    );

    tracing::info!(
        "##task_result get_pipeline_output_path task_file_path={:?}",
        task_file_path
    );

    debug!(
        "get pipeline output path:  {} {} {} {} {} {}",
        team_id, project_id, browser_path, job_id, task_instance_id, job_instance_id
    );

    let nbconvert_path = get_nbconvert_by_team_id(team_id.to_string());
    let template_file_path = get_store_full_path(team_id, project_id, &browser_path);

    tracing::info!("##task_result template_file_path={:?}", template_file_path);
    tracing::info!("##task_result task_file_path    ={:?}", task_file_path);

    debug!(
        "template_path:  {:?}; task_file_path: {:?};",
        template_file_path, task_file_path
    );

    let real_path_str = if is_idpnb {
        let real_path = path_to_use(
            get_job_pipeline_store_path(team_id, project_id, job_id, browser_path.clone()),
            task_file_path.to_str().unwrap().to_string(),
        );
        tracing::info!("##task_result real_path={:?}", real_path);
        real_path
    } else {
        //not ipynb file , cat the job/xx.sh, or job/xx.py
        get_job_pipeline_store_path(team_id, project_id, job_id, browser_path.clone())
    };

    tracing::info!("##task_result _ipynb_flag    ={is_idpnb}");
    tracing::info!("##task_result _real_path_str ={}", real_path_str);

    if is_idpnb {
        debug!("ipynb file {} need convert to html", real_path_str);
        let prefixed_file_name = format!(
            "preview_{}",
            path_tool::escape_path_as_string(browser_path.clone())
        );
        let html_file_name =
            crate::business_::path_tool::name_convert(prefixed_file_name, "html".to_string());
        let dst = path_tool::get_full_tmp_path(team_id, project_id, html_file_name);

        ipynb2html(
            nbconvert_path,
            real_path_str,
            dst.to_str().unwrap().to_string(),
        )
        .await
    } else {
        debug!(
            "not ipynb file, using preview_by_realpath {}",
            real_path_str
        );
        preview_by_realpath(real_path_str).await
    }
}

pub async fn ipynb2html(
    nbconvert_path: String,
    real_path: String,
    dst: String,
) -> Result<Rsp<CatRsp>, ErrorTrace> {
    let html_file_exist = std::path::Path::new(&dst).exists();
    if html_file_exist {
        debug!("html file exists {}", dst);
        if let Some(true) = first_is_newer(dst.clone(), real_path.clone()) {
            debug!("{} is newer than {}", dst, real_path);
            preview_by_realpath(dst).await
        } else {
            debug!("{} is older than {}, generate html", dst, real_path);
            export_as_helper(
                nbconvert_path,
                real_path.clone(),
                "html".to_string(),
                dst.clone(),
            )
            .await?;
            preview_by_realpath(dst.clone()).await
        }
    } else {
        export_as_helper(nbconvert_path, real_path, "html".to_string(), dst.clone()).await?;
        preview_by_realpath(dst).await
    }
}

async fn nbconvert(
    nbconvert_bin_path: &String,
    convert_type: &String,
    nb_path: &String,
) -> Result<(), std::io::Error> {
    let python_bin = nbconvert_bin_path.replace("jupyter-nbconvert", "python3");
    let mut cmd = tokio::process::Command::new(python_bin);
    cmd.arg(nbconvert_bin_path)
        .arg("--to")
        .arg(convert_type)
        .arg(nb_path);
    tracing::info!("nbconvert: cmd = {cmd:?}");
    let start = std::time::Instant::now();
    let output = cmd.output().await?;
    tracing::info!("nbconvert time cost {:?}", start.elapsed());
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        tracing::error!("nbconvert error!\n{stderr}");
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "nbconvert error",
        ));
    }
    Ok(())
}

pub async fn export_as_helper(
    nbconvert_path: String,
    real_path: String,
    kind: String,
    dst_path: String,
) -> Result<(), ErrorTrace> {
    debug!("will generate {} -> {}", real_path, dst_path);
    let tmp_dir = business::path_tool::store_parent_dir()
        .join("store")
        .join("tmp");
    let validate_ipynb =
        make_validate_ipynb(tmp_dir.to_str().unwrap().to_string(), real_path).await?;
    if path_exists(nbconvert_path.clone()) {
        nbconvert(&nbconvert_path, &kind, &validate_ipynb).await?;
        debug!("generated {} success", validate_ipynb);
        let export_result_file = name_convert(validate_ipynb, "html".to_string());
        std::fs::copy(export_result_file, dst_path.clone())?;
        Ok(())
    } else {
        error!("not found jupyter-nbconvert at {} ", nbconvert_path);
        Err(ErrorTrace::new(REMOVE_ERROR_MSG).code(REMOVE_ERROR_CODE))
    }
}

pub async fn make_validate_ipynb(tmp_dir: String, real_path: String) -> Result<String, ErrorTrace> {
    tracing::info!("make_validate_ipynb: real_path={real_path}");
    let filename_str = escape_path_as_string(real_path.clone());
    let result_file = format!("{}/{}", tmp_dir, filename_str);

    let mut nb = file_tool::read_notebook_from_disk(real_path.clone()).await?;
    nb.base.nbformat = 4;
    nb.base.nbformat_minor = 5;
    for cell in nb.cells.iter_mut() {
        cell.execution_time = None;
        if cell.cell_type == CellType::Markdown {
            cell.execution_count = None;
            cell.outputs = Vec::new();
        } else {
            cell.cell_type = CellType::Code;
        }
        let outputs = &mut cell.outputs;

        for kvs in outputs.iter_mut() {
            // remove useless kv
            if kvs.contains_key("originData") {
                kvs.remove("originData");
            }

            if kvs.contains_key("originText") {
                kvs.remove("originText");
            }

            // change symbol from CamelCase to snake_case
            if kvs.contains_key("outputType") {
                let mm = kvs.remove("outputType").unwrap();
                kvs.insert("output_type".to_string(), mm);
            }

            if kvs.contains_key("data") {
                // special output, need many keys
                //  "execution_count": 12,
                //  "metadata": {},
                //  "output_type": "execute_result"
                if !kvs.contains_key("execution_count") {
                    kvs.insert("execution_count".to_string(), json!(9999_i64));
                }

                if !kvs.contains_key("metadata") {
                    kvs.insert("metadata".to_string(), json!({}));
                }

                if !kvs.contains_key("output_type") {
                    kvs.insert(
                        "output_type".to_string(),
                        json!("execute_result".to_string()),
                    );
                }
            }
        }
    }

    tracing::info!("file_tool::write_notebook_to_disk({result_file}");
    file_tool::write_notebook_to_disk(&result_file, &nb).await?;
    Ok(result_file)
}

pub async fn preview_by_realpath(path: String) -> Result<Rsp<CatRsp>, ErrorTrace> {
    let (mimetype, mimetype_str) =
        crate::handler::content::cat::file_mime_magic::find_mimetype(&path)?;
    info!("preview_by_realpath: {}", path);
    match mimetype {
        Mimetype::Notebook => {
            tracing::error!("for preview, still notebook");
            Err(ErrorTrace::new(PREVIEW_ERROR_MSG).code(PREVIEW_ERROR_CODE))
        }
        _ => {
            if let Some(t) = last_modified_time(&path) {
                debug!("{} last modified at {:?}", path, t);
                let file_content = tokio::fs::read_to_string(path).await?;
                let length = file_content.len();
                Ok(Rsp::success(CatRsp {
                    length,
                    last_modified: format!("{:?}", t),
                    content: CatRspBody::Text(file_content),
                    mime: mimetype_str,
                }))
            } else {
                Err(ErrorTrace::new(LAST_MODIFIED_ERROR_MSG).code(LAST_MODIFIED_ERROR_CODE))
            }
        }
    }
}

pub fn first_is_newer<P: AsRef<Path>>(first: P, second: P) -> Option<bool> {
    let metadata1 = fs::metadata(first);
    let metadata2 = fs::metadata(second);
    if let (Ok(m1), Ok(m2)) = (metadata1, metadata2) {
        if let (Ok(t1), Ok(t2)) = (m1.modified(), m2.modified()) {
            Some(t1 > t2)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn last_modified_time<P: AsRef<Path>>(aim: &P) -> Option<std::time::SystemTime> {
    if let Ok(meta) = fs::metadata(aim) {
        if let Ok(t) = meta.modified() {
            Some(t)
        } else {
            None
        }
    } else {
        None
    }
}

pub fn get_job_pipeline_store_path(
    team_id: u64,
    project_id: u64,
    job_id: String,
    path: String,
) -> String {
    let target_path = path_tool::get_store_path(
        team_id,
        project_id,
        business::business_term::ProjectFolder::PIPELINE,
    );
    let mut target_path_string = "".to_string();
    let file_separator = "/";
    target_path_string += target_path.to_str().unwrap();
    target_path_string += file_separator;
    target_path_string += job_id.as_str();
    target_path_string += path.as_str();
    target_path_string
    // Path::new(&target_path_string).to_path_buf()
}

#[cfg(test)]
#[cfg(target_os = "macos")]
mod tests {
    use super::export_as_helper;

    #[tokio::test]
    async fn test_export_helper() {
        let raw = "/tmp/a.ipynb".to_string();
        export_as_helper(
            "/Users/liuzhe/miniconda3/envs/torch_env/bin/jupyter-nbconvert".to_string(),
            raw,
            "html".to_string(),
            "/tmp/bb.html".to_string(),
        )
        .await
        .unwrap();
    }
}
