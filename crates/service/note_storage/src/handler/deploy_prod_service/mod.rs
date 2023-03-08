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

use err::ErrorTrace;

use crate::api_model::deploy_service::*;
/// http://idp-k8s-service-svc:8084/api/command/k8s
pub const K8S_SERVICE_API_BASE_URL: &str = "http://idp-k8s-service-svc:8084/api/command/k8s";
pub const K8S_SERVICE_API_V2_BASE_URL: &str = "http://idp-k8s-service-svc:8084/api/v2/command/k8s";
pub const ADMIN_API_BASE_URL: &str = "http://idp-admin-rs-svc:9092/api/v1/admin-rs";

pub mod aperation_log;
pub mod build_image;
pub mod check_model_service;
pub mod deploy_model;
pub mod destroy_model;
pub mod get_equipment;
pub mod get_resource;
pub mod get_services;
pub mod modify_service;

pub use aperation_log::*;
pub use build_image::build_image;
pub use check_model_service::check_model_batch_service_handler;
pub use check_model_service::check_model_service;
pub use deploy_model::deploy_model;
pub use destroy_model::destroy_model;
pub use get_equipment::*;
pub use get_resource::get_resource_handler;
pub use get_services::*;
pub use modify_service::*;
