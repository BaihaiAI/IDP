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

/// TODO add tower service/layer like https://github.com/hyperium/hyper/blob/master/examples/tower_server.rs ?
#[tokio::main]
async fn main() {
    tokio::spawn(async {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
        let mut counter = 0;
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    counter += 1;
                    if counter > 5 {

                    }
                }
            }
        }
    });
    kernel_manage::main().await;
}
