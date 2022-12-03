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

pub const SUCCESS_CODE: u32 = 21_000_000;

pub const API_FAIL_CODE: u32 = 51_000_500;
pub const API_FAIL_MSG: &str = "fail";

pub const NB_STORE_ERROR_CODE: u32 = 51000018;
pub const NB_STORE_ERROR_MSG: &str = "Store new notebook file failed.";

pub const NB_RENAME_ERROR_CODE: u32 = 51001003;
pub const NB_RENAME_ERROR_MSG: &str = "Notebook rename failed.";

pub const NB_NO_MORE_CONTENT_ERROR_CODE: u32 = 41001006;
pub const NB_NO_MORE_CONTENT_ERROR_MSG: &str = "no more";

pub const INVALID_FILETYPE_ERROR_CODE: u32 = 51_002_001;
pub const INVALID_FILETYPE_ERROR_MSG: &str = "File type not a notebook!";

pub const PREVIEW_ERROR_CODE: u32 = 51001001;
pub const PREVIEW_ERROR_MSG: &str = "preview still get notebook";
pub const LAST_MODIFIED_ERROR_CODE: u32 = 51001002;
pub const LAST_MODIFIED_ERROR_MSG: &str = "get last modified failed";

pub const PROJECT_NAME_UNIQ_CREATE_FAIL_CODE: u32 = 41000003;
pub const PROJECT_NAME_UNIQ_CREATE_FAIL_MSG: &str = "project name exist";

pub const PROJECT_NAME_UNIQ_CREATE_FAIL_CODE_RESOURCE_API: u32 = 41134014;

pub const PROJECT_GET_PROJECT_ID_FAIL_CODE: u32 = 51000007;
pub const PROJECT_GET_PROJECT_ID_FAIL_MSG: &str = "get project_id fail";

pub const PROJECT_CREATE_FINAL_FAIL_CODE: u32 = 51000008;
pub const PROJECT_CREATE_FINAL_FAIL_MSG: &str = "create project fail";

pub const PROJECT_NOT_FOUND_FAIL_CODE: u32 = 41000404;
pub const PROJECT_NOT_FOUND_FAIL_MSG: &str = "project not found";

pub const UPLOAD_MODEL_ERROR_CODE: u32 = 41_000_100;
pub const UPLOAD_MODEL_ERROR_MSG: &str = "upload model file failed.";

// hpopt
pub const HPOPT_CREATE_DB_EXISTS_CODE: u32 = 131500;
pub const HPOPT_CREATE_DB_EXISTS_MSG: &str = "db file name already exist";

pub const HPOPT_CREATE_DB_TIMEOUT_CODE: u32 = 131501;
pub const HPOPT_CREATE_DB_TIMEOUT_MSG: &str = "db file create timeout";

pub const HPOPT_RUN_START_FAIL_CODE: u32 = 131502;
pub const HPOPT_RUN_START_FAIL_MSG: &str = "hp optimize run process start fail.";
