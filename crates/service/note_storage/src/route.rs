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

use std::collections::HashMap;
use std::sync::Arc;

use axum::routing::on;
use axum::routing::MethodFilter;
use axum::Router;
use common_tools::io_tool::file_writer;
use sqlx::Pool;
use sqlx::Postgres;
use tokio::sync::Mutex;
use tower_cookies::CookieManagerLayer;

use crate::api::http::v2::pipeline;
use crate::api::http::v2::project;
use crate::api::http::v2::workspace;
use crate::handler::content as content_handler;
use crate::handler::content::content_entrance as content;
use crate::handler::environment;
use crate::handler::extension as extension_handler;
use crate::handler::inner;
use crate::handler::note;
use crate::handler::package;
use crate::handler::pipeline_handler;
use crate::handler::snapshot;
use crate::handler::state;
use crate::handler::workspace as workspace_handler;

pub async fn init_router(
    project_info_map: Arc<Mutex<HashMap<String, HashMap<String, String>>>>,
    pg_option: Option<Pool<Postgres>>,
    reload_handle: logger::ReloadLogLevelHandle,
) -> Router {
    let ctx = crate::app_context::AppContext::new(crate::app_context::Config {
        sqlite_path: "/store/idp.db".to_string(),
        redis_url: "redis://127.0.0.1".to_string(),
    })
    .await;
    let (file_writer, writer_receiver) = file_writer::init();
    tokio::spawn(file_writer::file_writer_recv_thread(writer_receiver));
    Router::new()
        .nest(
            "/api/v2/idp-note-rs",
            Router::new()
                .nest(
                    "/environment",
                    Router::new()
                        .route("/list", on(MethodFilter::GET, environment::conda_env_list))
                        .route("/clone", on(MethodFilter::POST, environment::clone))
                        .route(
                            "/clone/state",
                            on(MethodFilter::GET, environment::clone_state),
                        )
                        .route("/current", on(MethodFilter::GET, environment::current_env))
                        .route(
                            "/switch",
                            on(MethodFilter::PUT, environment::switch_environment),
                        ),
                )
                .nest(
                    "/workspace",
                    Router::new()
                        .route("/file", on(MethodFilter::POST, workspace::new_file))
                        .route(
                            "/file",
                            on(MethodFilter::DELETE, workspace_handler::delete_file_or_dir),
                        )
                        .route(
                            "/file/rename",
                            on(MethodFilter::POST, workspace::file_rename),
                        )
                        .route(
                            "/file/download",
                            on(MethodFilter::GET, workspace_handler::download),
                        )
                        .route(
                            "/file/exportAs",
                            on(MethodFilter::GET, workspace::export_as),
                        )
                        .route(
                            "/file/example",
                            on(MethodFilter::POST, workspace::example_project),
                        )
                        .route("/move", on(MethodFilter::POST, workspace::file_dir_move))
                        .route("/copy", on(MethodFilter::POST, workspace::file_dir_copy))
                        .route(
                            "/dir",
                            on(MethodFilter::DELETE, workspace_handler::delete_file_or_dir),
                        )
                        .route("/dir/new", on(MethodFilter::POST, workspace::dir_new))
                        .route(
                            "/dir/export",
                            on(MethodFilter::POST, workspace_handler::dir_export),
                        )
                        .route(
                            "/model/upload",
                            on(MethodFilter::POST, workspace::model_upload),
                        )
                        .route("/dir/zip", on(MethodFilter::POST, workspace::dir_zip))
                        .route(
                            "/dir/browse",
                            on(MethodFilter::POST, workspace::dir_lazy_load),
                        )
                        .route(
                            "/dir/recursive_browse",
                            on(MethodFilter::POST, workspace::dir_recursive_load),
                        )
                        .route("/dir/search", on(MethodFilter::POST, workspace::dir_search))
                        .route(
                            "/dir/global_keyword_search",
                            on(MethodFilter::POST, workspace::global_keyword_search),
                        ),
                )
                .nest("/project", {
                    Router::new()
                        .route("/new", on(MethodFilter::POST, project::new))
                        .route("/delete", on(MethodFilter::POST, project::delete))
                        // .route(
                        //     "/ray-chown-fix",
                        //     on(MethodFilter::POST, project::ray_chown_fix_one_time),
                        // )
                        .route_layer(axum::extract::Extension(file_writer.clone()))
                })
                .nest(
                    "/pipeline",
                    Router::new()
                        .route("/result", on(MethodFilter::GET, pipeline::cat_result))
                        .route("/taskResult", on(MethodFilter::GET, pipeline::task_result))
                        // pipeline/copy API is only used in pipeline-svc pod
                        .route("/copy", on(MethodFilter::POST, pipeline_handler::copy)),
                )
                .nest(
                    "/note",
                    Router::new()
                        .route("/uploadbigfile", on(MethodFilter::POST, note::upload_file))
                        .route_layer(axum::extract::Extension(file_writer)),
                )
                .nest(
                    "/extensions",
                    Router::new()
                        .route("/load/*path", axum::routing::get(extension_handler::load))
                        .route(
                            "/recommendedList",
                            on(MethodFilter::GET, extension_handler::recommended_list),
                        )
                        .route(
                            "/installedList",
                            on(MethodFilter::GET, extension_handler::installed_list),
                        )
                        .route("/update", on(MethodFilter::GET, extension_handler::update))
                        .route(
                            "/install",
                            on(MethodFilter::GET, extension_handler::install),
                        )
                        .route(
                            "/uninstall",
                            on(MethodFilter::GET, extension_handler::uninstall),
                        )
                        .route("/detail", on(MethodFilter::GET, extension_handler::detail)),
                )
                .nest(
                    "/inner",
                    Router::new()
                        .route("/version", on(MethodFilter::GET, inner::version))
                        .route(
                            "/change_log",
                            on(MethodFilter::GET, inner::change_log_level),
                        )
                        .route_layer(axum::extract::Extension(reload_handle)),
                )
                .nest("/snapshot", {
                    // snapshot::sqlite_io::migrate_sqlite_to_redis(ctx.redis_cache.clone()).await;
                    Router::new()
                        .route("/", on(MethodFilter::POST, snapshot::post_snapshot))
                        .route(
                            "/list",
                            on(MethodFilter::POST, snapshot::post_snapshot_list),
                        )
                        .route(
                            "/diff",
                            on(MethodFilter::POST, snapshot::post_snapshot_diff),
                        )
                        .route(
                            "/restore",
                            on(MethodFilter::POST, snapshot::post_snapshot_restore),
                        )
                    // .route(
                    //     "/download",
                    //     get_service(ServeDir::new(db_path)).handle_error(
                    //         |error: std::io::Error| async move {
                    //             (
                    //                 axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    //                 format!("Snapshot download error: {}", error),
                    //             )
                    //         },
                    //     ),
                    // )
                })
                .nest(
                    "/content",
                    Router::new()
                        .route("/cat", on(MethodFilter::GET, content_handler::cat))
                        .route(
                            "/fullPathCat",
                            on(
                                MethodFilter::GET,
                                content_handler::full_path_cat::full_path_cat,
                            ),
                        )
                        .route("/save", on(MethodFilter::POST, content_handler::save))
                        .route("/cell", on(MethodFilter::POST, content::insert_cell))
                        .route("/cell/add", on(MethodFilter::POST, content::add_cell))
                        .route("/cell", on(MethodFilter::PUT, content_handler::put_cell))
                        .route("/cell", on(MethodFilter::DELETE, content::delete_cell))
                        .route("/share", on(MethodFilter::GET, content_handler::share_cell))
                        .route("/loadShared", on(MethodFilter::GET, content::load_shared))
                        .route(
                            "/ipynbPreview",
                            on(MethodFilter::GET, content::ipynb_preview),
                        )
                        .route("/cell/move", on(MethodFilter::PUT, content::move_cell)),
                )
                .nest(
                    "/state",
                    Router::new().route("/health", on(MethodFilter::GET, state::health)),
                )
                .nest(
                    "/package",
                    Router::new()
                        .route("/list", on(MethodFilter::GET, package::pip_list::pip_list))
                        .route("/search", on(MethodFilter::GET, package::search::search))
                        .route(
                            "/install",
                            on(MethodFilter::POST, package::pip_install::pip_install),
                        ),
                )
                .route_layer(axum::extract::Extension(project_info_map))
                .route_layer(axum::extract::Extension(pg_option))
                .nest("/tensorboard", {
                    use std::collections::BTreeMap;

                    use crate::handler::tensorboard;

                    let project_id_tensorboard_port_mapping = Arc::new(Mutex::new(BTreeMap::<
                        tensorboard::ProjectId,
                        tensorboard::TensorboardEntry,
                    >::new(
                    )));
                    Router::new()
                        .route(
                            "/start",
                            on(MethodFilter::POST, tensorboard::start_tensorboard),
                        )
                        .route(
                            "/stop",
                            on(MethodFilter::POST, tensorboard::stop_tensorboard),
                        )
                        .route(
                            "/info",
                            on(MethodFilter::POST, tensorboard::tensorboard_info),
                        )
                        .route_layer(axum::extract::Extension(
                            project_id_tensorboard_port_mapping,
                        ))
                }),
        )
        .route_layer(CookieManagerLayer::new())
        .layer(axum::extract::Extension(ctx))
    // .layer(
    //     tower_http::trace::TraceLayer::new_for_http(),
    //     // .on_request(
    //     // |req: &axum::http::Request<_>, _span: &tracing::Span| {
    //     //     tracing::info!("{} {}", req.method(), req.uri());
    //     // })
    // )
}
