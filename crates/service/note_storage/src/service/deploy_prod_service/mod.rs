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
use axum::extract::Query;
use axum::Extension;
use axum::TypedHeader;
use common_model::Rsp;
use common_tools::cookies_tools::get_cookie_value_by_key;
use common_tools::cookies_tools::Cookies;
use err::ErrorTrace;

use crate::api_model::deploy_service::*;
use crate::handler::deploy_prod_service::*;

pub mod deploy_prod_service;
pub use deploy_prod_service::*;
pub mod kubeedge_job;
pub mod service_list;
pub mod service_log;
pub mod service_task_history;
pub mod update_service;

pub use kubeedge_job::*;
pub use update_service::*;
