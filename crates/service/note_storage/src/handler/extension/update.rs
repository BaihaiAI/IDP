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

use axum::extract::Json;
use common_model::Rsp;
use err::ErrorTrace;

use super::install::install_handler;
use super::models::ExtensionReq;
use super::uninstall::uninstall_handler;

pub async fn update(Json(req): Json<ExtensionReq>) -> Result<Rsp<String>, ErrorTrace> {
    uninstall_handler(req.team_id, req.user_id, &req.name).await?;
    install_handler(req.team_id, req.user_id, &req.name, &req.version).await
}
