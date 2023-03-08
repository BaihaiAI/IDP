// Copyright 2023 BaihaiAI, Inc.
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

// use std::collections::HashMap;
#[cfg(test)]
use std::error::Error;

use serde::Deserialize;

// use serde::Serialize;
use crate::pojo::component::algorithm::normalization_meta_info::NormalizationMetaInfo;
use crate::pojo::component::algorithm::normalization_para_config::NormalizationParaConfig;
use crate::pojo::component::algorithm::normalization_resource_config::NormalizationResourceConfig;
use crate::pojo::component::comm::comm_info::CommInfo;

/**
 * Copyright @baihai 2021
 * User: Zhang Qike
 * Date: 2023/1/18
 * Time: 15:28
 * To change this template use Preferences | Editor | File and Code Templates | Rust File
 */

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Normalization {
    pub para_config: Option<NormalizationParaConfig>,
    pub resource_config: Option<NormalizationResourceConfig>,
    pub meta_info: Option<NormalizationMetaInfo>,
    pub comm_info: Option<CommInfo>,
}

#[test]
fn string_to_normalization() -> Result<(), Box<dyn Error>> {
    use std::collections::HashMap;
    let normalization_map_str = r#"
        {
            "normalization": {
                "paraConfig": {
                    "method": "MAX",
                    "options": ["MAX", "MIN", "RADOM"],
                    "reserved": false
                },
                "resourceConfig": {
                    "cpu": "1000Core",
                    "mem": "10G"
                },
                "metaInfo": {
                    "inputPath": "xxx.csv",
                    "pointInfo": [{
                            "pointId": 1,
                            "pointCtrlType": "input",
                            "pointDataType": "csv"
                        },
                        {
                            "pointId": 2,
                            "pointCtrlType": "input",
                            "pointDataType": "csv"
                        },
                        {
                            "pointId": 3,
                            "pointCtrlType": "output",
                            "pointDataType": "csv"
                        }
                    ]
                },
                "commInfo": {
                    "exInfo": {
                        "positionX": -220,
                        "positionY": -220
                    },
                    "script": "component1",
                    "taskId": 1674094476605,
                    "taskEdge": [{
                        "toTask": 1674094479326,
                        "fromTask": 1674094476605
                    }],
                    "taskName": "helloworld.idpnb",
                    "taskType": 1
                }
            }
        }
    "#;

    let obj = serde_json::from_str::<serde_json::Value>(normalization_map_str).unwrap();
    println!("obj={:?}", obj);
    let map = obj.as_object().unwrap();
    let mut key = "".to_string();
    for (k, v) in map {
        println!("obj key={} , value={:?}", k, v);
        key = k.to_string();
    }

    if key.eq_ignore_ascii_case("Normalization") {
        let map: HashMap<String, Normalization> = serde_json::from_str(normalization_map_str)?;
        println!("map={:?}", map);

        let obj = map.get("normalization");
        println!("obj value={:?}", obj);

        if let Some(obj) = obj {
            let meta_info = obj.meta_info.as_ref().unwrap();
            println!("meta_info={:?}", meta_info);
            let inputpath = &meta_info.input_path;
            println!("inputpath={:?}", inputpath);
        }
    } else if key.eq_ignore_ascii_case("ReadCsv") {
        type ReadCsv = u32;
        let map: HashMap<String, ReadCsv> = serde_json::from_str(normalization_map_str)?;
        println!("map={:?}", map);

        let obj = map.get("readCsv");
        println!("obj value={:?}", obj);
    }

    Ok(())
}

#[test]
fn string_normalization() -> Result<(), Box<dyn Error>> {
    let _normalization_str = r#"
        {
            "paraConfig": {
                "method": "MAX",
                "options": ["MAX", "MIN", "RADOM"],
                "reserved": false
            },
            "resourceConfig": {
                "cpu": "1000Core",
                "mem": "10G"
            }
        }
    "#;

    // let obj : Normalization = serde_json::from_str::<serde_json::Value>(&normalization_str)?;
    // let obj: Normalization = serde_json::from_str(&normalization_str)?;
    // println!("obj={:?}", obj);

    let _normalization_map_str = r#"
        {
            "normalization": {
                "paraConfig": {
                    "method": "MAX",
                    "options": ["MAX", "MIN", "RADOM"],
                    "reserved": false
                },
                "resourceConfig": {
                    "cpu": "1000Core",
                    "mem": "10G"
                }
            }
        }
    "#;

    //
    // let map: HashMap<String,Normalization> = serde_json::from_str(&normalization_map_str)?;
    // println!("map={:?}", map);
    //
    // for (k, v) in map {
    //     println!("key={} , value={:?}", k, v);
    //
    //     if k.eq_ignore_ascii_case("Normalization") {
    //         println!("Normalization Normalization")
    //     }else if k.eq_ignore_ascii_case("ReadCsv") {
    //         println!("ReadCsv ReadCsv ")
    //     }else{
    //
    //     }
    // }

    // let normalization_para_config_str = r#"
    //   {"method":"MAX","options":["MAX","MIN","RADOM"],"reserved":false}
    // "#;
    // let obj1: NormalizationParaConfig = serde_json::from_str(&normalization_para_config_str)?;
    // println!("obj1={:?}", obj1);

    Ok(())
}

#[test]
fn string_read_csv() -> Result<(), Box<dyn Error>> {
    let normalization_map_str = r#"
        {
            "readCsv": {
                "paraConfig": {
                    "input": "MAX"
                }
            }
        }
    "#;

    let obj = serde_json::from_str::<serde_json::Value>(normalization_map_str).unwrap();
    println!("obj={:?}", obj);
    let map = obj.as_object().unwrap();
    let mut key = "".to_string();
    for (k, _v) in map {
        // println!("obj key={} , value={:?}", k, v);
        key = k.to_string();
    }

    if key.eq_ignore_ascii_case("Normalization") {
        //todo
    } else if key.eq_ignore_ascii_case("ReadCsv") {
        type ReadCsv = u32;
        let map: std::collections::HashMap<String, ReadCsv> =
            serde_json::from_str(normalization_map_str)?;
        println!("map={:?}", map);

        let obj = map.get("readCsv");
        println!("obj value={:?}", obj);

        // for (k, v) in map {
        //     println!("real key={} , value={:?}", k, v);
        //
        //     if k.eq_ignore_ascii_case("Normalization") {
        //         println!("Normalization Normalization")
        //     }else if k.eq_ignore_ascii_case("ReadCsv") {
        //         println!("ReadCsv ReadCsv..........");
        //         println!("{:?}", v.para_config);
        //     }else{
        //
        //     }
        // }
    } else {
    }

    // let map: HashMap<String, Val> = serde_json::from_str(&normalization_map_str)?;
    // println!("map={:?}", map);
    //
    // for (k, v) in map {
    //     println!("key={} , value={:?}", k, v);
    //
    //     if k.eq_ignore_ascii_case("Normalization") {
    //         println!("Normalization Normalization")
    //     }else if k.eq_ignore_ascii_case("ReadCsv") {
    //         println!("ReadCsv ReadCsv ")
    //     }else{
    //
    //     }
    // }

    Ok(())
}
