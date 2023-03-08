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

use axum::routing::delete;
use axum::routing::get;
use axum::routing::on;
use axum::routing::post;
use axum::routing::MethodFilter;
use axum::Extension;
use axum::Router;
use common_tools::io_tool::file_writer;
use handler::content as content_handler;
use handler::content::content_entrance as content;
use handler::extension as extension_handler;
use handler::inner;
use handler::note;
use handler::pipeline_handler;
use handler::snapshot;
use handler::state;
use handler::team_handler;
use handler::workspace as workspace_;
use tokio::sync::Mutex;
use workspace_::decompress::unzip;

use crate::api::http::v2::hpopt;
use crate::api::http::v2::pipeline;
use crate::api::http::v2::project;
use crate::api::http::v2::workspace;
use crate::app_context::AppContext;
use crate::app_context::DB_POOL;
use crate::handler;
use crate::handler::deploy::equipment;
use crate::service::component::component_lib_service as component_svc;
use crate::service::deploy_prod_service;
use crate::service::deploy_prod_service::service_list;
use crate::service::deploy_prod_service::service_log;
use crate::service::deploy_prod_service::service_task_history;

/// 10 GB
const MAX_UPLOAD_SIZE: usize = 1024 * 1024 * 1024 * 10;

pub async fn init_router(reload_handle: logger::ReloadLogLevelHandle) -> Router {
    Router::new()
        .nest("/api/v2/idp-note-rs", public_api_route(reload_handle).await)
        .nest(
            "/api/private",
            Router::new()
                .route("/closeTeam", post(team_handler::close_team_handler))
                .route("/closeUser", post(team_handler::close_user_handler)),
        )
}

async fn public_api_route(reload_handle: logger::ReloadLogLevelHandle) -> Router {
    let ctx = AppContext::new().await;
    let (file_writer, writer_receiver) = file_writer::init();
    tokio::spawn(file_writer::file_writer_recv_thread(writer_receiver));

    let router = Router::new()
        .nest(
            "/environment",
            handler::environment::environment_apis_route(ctx.clone()),
        )
        .nest(
            "/workspace",
            Router::new()
                .route("/move", post(workspace::file_dir_move))
                .route("/copy", post(workspace::file_dir_copy))
                .route("/file", post(workspace::new_file))
                .route("/file", delete(workspace_::delete_file_or_dir))
                .route("/file/rename", post(workspace::file_rename))
                .route("/file/download", get(workspace_::download))
                .route("/file/upload_from_url", post(workspace_::upload_from_url))
                .route("/file/exportAs", get(workspace::export_as))
                .route("/file/convertTo", get(workspace::convert_to))
                .route("/file/example", post(workspace::example_project))
                .route("/file/decompress", post(unzip))
                .route("/dir", delete(workspace_::delete_file_or_dir))
                .route("/dir/new", post(workspace::dir_new))
                .route("/dir/export", post(workspace_::dir_export))
                .route("/dir/zip", post(workspace_::compress::dir_zip))
                .route("/dir/browse", post(workspace::dir_lazy_load))
                .route("/dir/recursive_browse", post(workspace::dir_recursive_load))
                .route("/dir/search", post(workspace::dir_search))
                .route("/dir/pre-search", post(workspace::keyword_search))
                .route("/model/upload", post(workspace::model_upload))
                .route("/model/export", post(workspace::model_export))
                .route("/model/export_dir", post(workspace::model_export_dir))
                .route(
                    "/dir/global_keyword_search",
                    post(workspace::global_keyword_search),
                )
                .route(
                    "/dir/global_keyword_search_dir_file",
                    post(workspace::global_keyword_search_dir_file),
                )
                .with_state(ctx.clone()),
        )
        .nest("/hpopt", {
            Router::new()
                .route("/datasource/list", get(hpopt::datasource_list))
                .route("/datasource/new", post(hpopt::datasource_new))
                .route("/backend/start", get(hpopt::start_hpopt_backend))
                .route("/backend/stop", get(hpopt::stop_hpopt_backend))
                .route("/backend/state", get(hpopt::backend_state))
                .route("/study", delete(hpopt::delete_study))
                .route("/study/list", get(hpopt::list_study))
                .route("/study/detail", get(hpopt::study_detail))
                .route("/study/new", post(hpopt::study_new))
                .route("/study/objective-code", get(hpopt::study_objective_code))
                .route(
                    "/study/objective-code",
                    post(hpopt::edit_study_objective_code),
                )
                .route(
                    "/optimize/example-names",
                    get(hpopt::objective_example_names),
                )
                .route("/optimize/example-code", get(hpopt::objective_code_content))
                .route("/optimize/run", post(hpopt::study_optimize_run))
                .route("/optimize/state", get(hpopt::optimize_state))
                .route("/optimize/log", get(hpopt::optimize_log))
                .route("/optimize/list", get(hpopt::study_optimize_run_list))
                .with_state(ctx.clone())
        })
        .nest("/project", {
            Router::new()
                .route("/new", post(project::new))
                .route("/new_v2", post(project::new_v2))
                .route("/delete", post(project::delete))
                .with_state(file_writer.clone())
        })
        .nest(
            "/pipeline",
            Router::new()
                .route("/result", get(pipeline_handler::cat_result))
                .route("/taskResult", get(pipeline::task_result))
                // pipeline/copy API is only used in pipeline-svc pod
                .route("/copy", post(pipeline_handler::copy)),
        )
        .nest(
            "/note",
            Router::new()
                .route("/uploadbigfile", post(note::upload_file))
                .with_state(file_writer),
        )
        // .nest("/team", Router::new().route("/init", post(team_handler::init_team)))
        .nest(
            "/extensions",
            Router::new()
                .route(
                    "/load/*path",
                    axum::routing::get(extension_handler::load)
                        .layer(tower_http::compression::CompressionLayer::new()),
                )
                .route("/recommendedList", get(extension_handler::recommended_list))
                .route("/installedList", get(extension_handler::installed_list))
                .route("/update", post(extension_handler::update))
                .route("/install", get(extension_handler::install))
                .route("/uninstall", get(extension_handler::uninstall))
                .route("/initInstall", post(extension_handler::init_install))
                .route("/detail", get(extension_handler::detail)),
        )
        .nest(
            "/inner",
            Router::new()
                .route("/version", get(inner::version))
                .route("/change_log", get(inner::change_log_level))
                .with_state(reload_handle),
        )
        .nest(
            "/publish_model",
            Router::new().route(
                "/",
                post(handler::publish_third_party_model_platform::publish_model),
            ),
        )
        .nest("/runtime", handler::runtime_pod::routes())
        .nest("/snapshot", {
            Router::new()
                .route("/", post(snapshot::post_snapshot))
                .route("/list", post(snapshot::post_snapshot_list))
                .route("/diff", post(snapshot::post_snapshot_diff))
                .route("/restore", post(snapshot::post_snapshot_restore))
                .with_state(ctx.clone())
        })
        .nest(
            "/content",
            Router::new()
                .route("/cat", get(content_handler::cat))
                .route(
                    "/load",
                    get(content_handler::load::load)
                        .layer(tower_http::compression::CompressionLayer::new()),
                )
                .route(
                    "/fullPathCat",
                    get(content_handler::full_path_cat::full_path_cat),
                )
                .route("/save", post(content_handler::save))
                .route("/cell", post(content::insert_cell))
                .route("/cell/add", post(content::add_cell))
                .route("/cell", on(MethodFilter::PUT, content_handler::put_cell))
                .route("/cell", delete(content::delete_cell))
                .route("/share", get(content_handler::share_cell))
                .route("/loadShared", get(content::load_shared))
                .route("/ipynbPreview", get(content::ipynb_preview))
                .route("/cell/move", on(MethodFilter::PUT, content::move_cell))
                .with_state(ctx),
        )
        .nest("/state", Router::new().route("/health", get(state::health)))
        .nest("/package", handler::package::routes())
        .nest("/tensorboard", {
            use std::collections::BTreeMap;

            use crate::handler::tensorboard;
            let project_id_tensorboard_port_mapping = std::sync::Arc::new(Mutex::new(BTreeMap::<
                tensorboard::ProjectId,
                tensorboard::TensorboardEntry,
            >::new(
            )));
            Router::new()
                .route("/start", post(tensorboard::start_tensorboard))
                .route("/stop", post(tensorboard::stop_tensorboard))
                .route("/info", post(tensorboard::tensorboard_info))
                .with_state(project_id_tensorboard_port_mapping)
        })
        .nest(
            "/deploy-prod-model",
            Router::new()
                .route("/list", get(deploy_prod_service::list))
                .route("/detail", get(deploy_prod_service::service_detail))
                .route("/operation-log", get(deploy_prod_service::operation_list))
                .route("/delete", post(deploy_prod_service::delete))
                .route("/start", post(deploy_prod_service::start))
                .route("/stop", post(deploy_prod_service::stop))
                .route("/new", post(deploy_prod_service::deploy))
                .route("/run", post(deploy_prod_service::run_once))
                .route(
                    "/history-task-list",
                    get(service_task_history::service_task_history_list_handler),
                )
                .route(
                    "/history-task-insert",
                    post(service_task_history::insert_service_task_history),
                )
                .route(
                    "/history-task-update-status",
                    post(service_task_history::update_service_task_history_handler),
                )
                .route("/total-resource", get(deploy_prod_service::get_resource))
                .route(
                    "/update-resource",
                    post(deploy_prod_service::update_resource),
                )
                .route("/update", post(deploy_prod_service::update_all))
                .route("/update-cron", post(deploy_prod_service::update_cron))
                .route("/update-info", post(deploy_prod_service::update_basic_info))
                .route(
                    "/kubeedge-job/update",
                    post(deploy_prod_service::update_kubeedge_job),
                )
                .route(
                    "/kubeedge-job/delete",
                    post(deploy_prod_service::delete_kubeedge_job),
                )
                .route(
                    "/kubeedge-job/add",
                    post(deploy_prod_service::add_kubeedge_job),
                )
                .route(
                    "/kubeedge-job/rollback",
                    post(deploy_prod_service::rollback_kubeedge_job),
                )
                .route(
                    "/equipments",
                    get(deploy_prod_service::get_service_equipment),
                )
                .route("/service/container-list", post(service_log::container_list))
                .route(
                    "/service/container-resource",
                    post(service_log::resource_usage),
                )
                .route("/service/log-list", get(service_log::log_list))
                .route("/service/log", get(service_log::get_log))
                .route("/service/list-by-equip", get(service_list::list_by_equip))
                .route("/equipment/list", get(equipment::get_equipment_list))
                .route("/equipment/csv", get(equipment::equipment_csv))
                .route("/equipment/delete", post(equipment::delete_equipment))
                .route("/equipment/new", post(equipment::insert_equipment))
                .route("/equipment/activate", post(equipment::activate_equipment))
                .route("/equipment/detail", get(equipment::equipment_detail))
                .route("/equipment/update", post(equipment::equipment_update)),
        )
        .nest("/visual-modeling", handler::visual_modeling::routes())
        .nest(
            "/component-lib",
            Router::new()
                .route("/lib/browse", get(component_svc::get_directory))
                .route("/lib/detail", get(component_svc::get_component_detail))
                .route("/job/new", post(component_svc::insert_job))
                .route("/job/update", post(component_svc::update_job))
                .route("/job/detail", get(component_svc::get_job_detail_handler))
                .route("/job/schedule", post(component_svc::update_job_schedule))
                .route("/job/list", get(component_svc::get_job_list))
                .route("/job/delete", post(component_svc::delete_job))
                .route(
                    "/job-instance/list",
                    get(component_svc::get_job_instance_list),
                )
                .route(
                    "/job-instance/detail",
                    get(component_svc::get_job_instance_detail),
                )
                .route("/template/new", post(component_svc::insert_template))
                .route("/template/update", post(component_svc::update_template))
                .route("/template/list", get(component_svc::get_template_list))
                .route("/template/delete", post(component_svc::delete_template))
                .route("/template/detail", get(component_svc::get_template_detail))
                .route("/get-schedule-list", get(component_svc::get_time_list)),
        )
        .layer(axum::extract::DefaultBodyLimit::max(MAX_UPLOAD_SIZE));
    if business::kubernetes::is_k8s() {
        router.route_layer(Extension(DB_POOL.clone()))
    } else {
        router
    }
}
