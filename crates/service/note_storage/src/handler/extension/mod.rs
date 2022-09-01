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

mod detail;
mod install;
mod installed_list;
mod load;
mod models;
mod recommended_list;
mod uninstall;
mod update;

use std::path::Path;

pub use detail::detail;
use err::ErrorTrace;
pub use install::install;
pub use installed_list::installed_list;
pub use load::load;
pub use recommended_list::recommended_list;
pub use uninstall::uninstall;
pub use update::update;

use self::models::ExtensionResp;

pub fn get_extensions_config<P: AsRef<Path>>(
    extension_config_path: P,
) -> Result<Vec<ExtensionResp>, ErrorTrace> {
    let jdata = match std::fs::read_to_string(extension_config_path) {
        Ok(jdata) => jdata,
        Err(err) => {
            tracing::error!("{err}");
            return Err(ErrorTrace::new(""));
        }
    };

    match serde_json::from_str::<Vec<ExtensionResp>>(&jdata) {
        Ok(content) => Ok(content),
        Err(err) => {
            tracing::error!("{err}");
            let empty: Vec<ExtensionResp> = Vec::new();
            Ok(empty)
        }
    }
}
