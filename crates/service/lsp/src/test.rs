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

#[test]
fn test_split() {
    let s = String::from("a b c");
    use regex::Regex;
    let p = Regex::new(r"\s+").expect("invalidate regex");
    let parts: Vec<_> = p.split(&s).collect();
    println!("{}", parts.len());
    for p in parts {
        println!("{}", p);
    }
}

#[test]
fn test_many_msg() {
    let m = r#"{"jsonrpc":"2.0","method":"window/logMessage","params":{"type":3,"message":"Pyright language server 1.1.169 starting"}}Content-Length: 161"#;
    use regex::Regex;
    let header_pattern = Regex::new(r"Content-Length:\s+\d+\s*").expect("header re error");
    let _parts: Vec<_> = header_pattern.split(m).collect();
    println!("kkkk");
}

#[test]
fn test_contains_re() {
    use regex::Regex;
    let s1 = r#""method":"initialized","#;
    let s2 = r#""method":   "initialized","#;
    let p = Regex::new(r#".*"method":\s*"initialized"\s*,.*"#).expect("client register re error");
    //assert!(s1.contains(r#""method":\s*"initialized","#))
    assert!(p.is_match(s1));
    assert!(p.is_match(s2));
}

#[tokio::test]
async fn test_mpsc() {
    use tokio::sync::mpsc;
    let (ws_sender, mut ws_reciver) = mpsc::channel(32);

    tokio::spawn(async move {
        while let Some(msg) = ws_reciver.recv().await {
            println!("{}", msg);
        }
    });

    tokio::spawn(async move {
        ws_sender.send("hello").await.unwrap();
        ws_sender.send("world").await.unwrap();
    })
    .await
    .expect("spawn failed");
}

#[test]
fn test_u8_complete() {
    let s = "hello";
    let iter = s.chars();
    let mut count = 0;
    for c in iter {
        count += 1;
        println!("{}", c);
    }
    println!("{}", count);
}
