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
use std::net::SocketAddr;
use std::ops::Add;
use std::ops::Sub;
use std::process;
use std::time::Duration;
use std::time::SystemTime;

use err::ErrorTrace;
use futures_util::stream::SplitSink;
use futures_util::stream::SplitStream;
use futures_util::SinkExt;
use futures_util::StreamExt;
use lsp_types::notification;
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::process::Child;
use tokio::process::ChildStdin;
use tokio::process::ChildStdout;
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use tokio_tungstenite::accept_hdr_async;
use tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use tracing::*;

use crate::helper::complete_u8;
use crate::helper::env_or_default;
use crate::helper::get_python_site;
use crate::helper::handle_handshake;

mod helper;
mod test;

const GIT_VERSION: &str = env!("VERSION");

type Responder<T> = oneshot::Sender<T>;
const ORDER_ERROR_NOTIFICATION: &str = "REOPEN";
const OPENED_NOTIFICATION: &str = "OPENED";
// const PYTHON_BASE_DIR: &str = "/opt/miniconda3";
const PYRIGHT_HELLO_HEADER: &str = r#"{"jsonrpc":"2.0","method":"window/logMessage","params":{"type":3,"message":"Server root directory: "#;
const FAK_PID: &str = "88985";
const RESET_CMD: &str = "RESET";
const CLOSE_CMD: &str = "CLOSE";
const RESET_FRAME: Message = Message::Close(Some(CloseFrame {
    code: CloseCode::Unsupported,
    reason: std::borrow::Cow::Borrowed(r#"reset yourself"#),
}));
const CLOSE_FRAME: Message = Message::Close(Some(CloseFrame {
    code: CloseCode::Normal,
    reason: std::borrow::Cow::Borrowed(""),
}));

const PING: &str = "ping";
const PONG: &str = "pong";

pub fn thousands_days_later() -> SystemTime {
    SystemTime::now().add(chrono::Duration::days(1000).to_std().expect("msg"))
}

#[derive(Debug)]
enum ConfigCommand {
    GetLsp { resp: Responder<Lsp> },
    RefreshVisitTime { time: SystemTime },
    // SetNeedReset {
    //     time: SystemTime,
    // },
    Visit,
    SetInited,
    ResetEnv { new_env: String },
}

type CommandWithId = (String, String, ConfigCommand);

#[derive(Debug, Clone)]
struct Lsp {
    initd: bool,
    python: String,
    env_name: String,
    check_mode: String,
    pyright: String,
    node: String,
    visited: bool,
    last_visit_time: SystemTime,
    need_update_time: SystemTime,
    project_id: String,
    team_id: String,
}

impl Lsp {
    pub fn new(team: String, project: String) -> Lsp {
        let team_id = team.parse().unwrap();
        let project_id = project.parse().unwrap();
        let conda_env_name = business::path_tool::project_conda_env(team_id, project_id);
        let python_path =
            business::path_tool::get_conda_env_python_path(team_id, conda_env_name.clone());

        Lsp {
            initd: false,
            pyright: env_or_default("PY_RIGHT_SERVER_JS", "/opt/lsp/pyright/server.js"),
            check_mode: String::from("off"),
            env_name: conda_env_name,
            python: python_path,
            node: env_or_default("NODE_BIN", "node"),
            visited: true,
            last_visit_time: SystemTime::now(),
            need_update_time: thousands_days_later(),
            team_id: team,
            project_id: project,
        }
    }
    pub(crate) fn log_self(&self, prefix: &str) {
        trace!("LSP {}: {} {}", prefix, self.initd, self.python);
    }
}

async fn handle_lsp_message(
    lsp_msg: String,
    lsp_sender: Sender<String>,
    conf_sender: Sender<CommandWithId>,
    team_id: String,
    project_id: String,
) -> bool {
    conf_sender
        .send((team_id.clone(), project_id.clone(), ConfigCommand::Visit))
        .await
        .expect("update lsp visit status failed");
    let init_re = Regex::new(r#".*"method":\s*"initialize"\s*,.*"#).expect("init re error");
    let inited_re = Regex::new(r#".*"method":\s*"initialized"\s*,.*"#).expect("inited re error");

    if init_re.is_match(lsp_msg.as_str()) {
        let lsp = get_lsp_conf(conf_sender, team_id.clone(), project_id.clone()).await;
        if lsp.initd {
            // already initd, ignore init message
            true
        } else {
            // init message need change process id to real pid
            let init_message_with_right_pid =
                lsp_msg.replace(FAK_PID, current_pid().to_string().as_str());
            match lsp_sender.send(init_message_with_right_pid).await {
                Ok(_) => true,
                Err(err) => {
                    warn!("{err}");
                    false
                }
            }
        }
    } else if inited_re.is_match(lsp_msg.as_str()) {
        let lsp = get_lsp_conf(conf_sender.clone(), team_id.clone(), project_id.clone()).await;
        if lsp.initd {
            debug!("already inited, skip inited message");
            true
        } else {
            conf_sender
                .send((
                    team_id.clone(),
                    project_id.clone(),
                    ConfigCommand::SetInited,
                ))
                .await
                .expect("set inited failed");
            if (lsp_sender.send(lsp_msg).await).is_err() {
                return false;
            }

            let config_after_inited = format!(
                r#"{{"jsonrpc":"2.0","method":"workspace/didChangeConfiguration","params":{{"settings":{{"python":{{"analysis":{{"autoImportCompletions":true,"autoSearchPaths":true,"extraPaths":[],"stubPath":"typings","diagnosticMode":"openFilesOnly","diagnosticSeverityOverrides":{{}},"logLevel":"Information","typeCheckingMode":"{check_mode}","typeshedPaths":[],"useLibraryCodeForTypes":false}},"pythonPath":"python","venvPath":"","defaultInterpreterPath":"{python_bin}"}},"pyright":{{"disableLanguageServices":false,"disableOrganizeImports":false}}}}}}"#,
                check_mode = lsp.check_mode,
                python_bin = lsp.python
            );
            debug!("will change python path to {}", lsp.python);
            lsp_sender.send(config_after_inited).await.is_ok()
        }
    } else {
        // not init related lsp message
        lsp_sender.send(lsp_msg).await.is_ok()
    }
}

async fn message_need_forward(
    lsp_msg: &str,
    lsp_sender: Sender<String>,
    conf_sender: Sender<CommandWithId>,
    team_id: String,
    project_id: String,
) -> bool {
    not_client_register(lsp_msg, lsp_sender.clone()).await
        && not_python_hello(
            lsp_msg,
            lsp_sender.clone(),
            conf_sender.clone(),
            team_id.clone(),
            project_id.clone(),
        )
        .await
        && not_auto_init(lsp_msg, lsp_sender.clone()).await
        && not_workspace_config(
            lsp_msg,
            lsp_sender.clone(),
            conf_sender,
            team_id,
            project_id,
        )
        .await
}

async fn not_python_hello(
    lsp_msg: &str,
    lsp_sender: Sender<String>,
    conf_sender: Sender<CommandWithId>,
    team_id: String,
    project_id: String,
) -> bool {
    if lsp_msg.starts_with(PYRIGHT_HELLO_HEADER) {
        let init_msg = r#"{"jsonrpc":"2.0","id":0,"method":"initialize","params":{"processId":88985,"rootPath":null,"rootUri":null,"capabilities":{"workspace":{"applyEdit":true,"workspaceEdit":{"documentChanges":true,"resourceOperations":["create","rename","delete"],"failureHandling":"textOnlyTransactional"},"didChangeConfiguration":{"dynamicRegistration":true},"didChangeWatchedFiles":{"dynamicRegistration":true},"symbol":{"dynamicRegistration":true,"symbolKind":{"valueSet":[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26]},"tagSupport":{"valueSet":[1]}},"codeLens":{"refreshSupport":true},"executeCommand":{"dynamicRegistration":true},"configuration":true,"semanticTokens":{"refreshSupport":true},"fileOperations":{"dynamicRegistration":true,"didCreate":true,"didRename":true,"didDelete":true,"willCreate":true,"willRename":true,"willDelete":true},"workspaceFolders":true},"textDocument":{"publishDiagnostics":{"relatedInformation":true,"versionSupport":false,"tagSupport":{"valueSet":[1,2]}},"synchronization":{"dynamicRegistration":true,"willSave":true,"willSaveWaitUntil":true,"didSave":true},"completion":{"dynamicRegistration":true,"contextSupport":true,"completionItem":{"snippetSupport":true,"commitCharactersSupport":true,"documentationFormat":["markdown","plaintext"],"deprecatedSupport":true,"preselectSupport":true,"insertReplaceSupport":true,"tagSupport":{"valueSet":[1]},"resolveSupport":{"properties":["documentation","detail","additionalTextEdits"]},"insertTextModeSupport":{"valueSet":[1,2]}},"completionItemKind":{"valueSet":[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25]}},"hover":{"dynamicRegistration":true,"contentFormat":["markdown","plaintext"]},"signatureHelp":{"dynamicRegistration":true,"contextSupport":true,"signatureInformation":{"documentationFormat":["markdown","plaintext"],"activeParameterSupport":false,"parameterInformation":{"labelOffsetSupport":true}}},"definition":{"dynamicRegistration":true},"references":{"dynamicRegistration":true},"documentHighlight":{"dynamicRegistration":true},"documentSymbol":{"dynamicRegistration":true,"symbolKind":{"valueSet":[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26]},"hierarchicalDocumentSymbolSupport":true,"tagSupport":{"valueSet":[1]}},"codeAction":{"dynamicRegistration":true,"isPreferredSupport":true,"disabledSupport":true,"dataSupport":true,"honorsChangeAnnotations":false,"resolveSupport":{"properties":["edit"]},"codeActionLiteralSupport":{"codeActionKind":{"valueSet":["","quickfix","refactor","refactor.extract","refactor.inline","refactor.rewrite","source","source.organizeImports"]}}},"codeLens":{"dynamicRegistration":true},"formatting":{"dynamicRegistration":true},"rangeFormatting":{"dynamicRegistration":true},"onTypeFormatting":{"dynamicRegistration":true},"rename":{"dynamicRegistration":true,"prepareSupport":true},"documentLink":{"dynamicRegistration":true,"tooltipSupport":true},"typeDefinition":{"dynamicRegistration":true},"implementation":{"dynamicRegistration":true},"declaration":{"dynamicRegistration":true},"colorProvider":{"dynamicRegistration":true},"foldingRange":{"dynamicRegistration":true,"rangeLimit":5000,"lineFoldingOnly":true},"selectionRange":{"dynamicRegistration":true},"callHierarchy":{"dynamicRegistration":true},"semanticTokens":{"dynamicRegistration":true,"tokenTypes":["namespace","type","class","enum","interface","struct","typeParameter","parameter","variable","property","enumMember","event","function","method","macro","keyword","modifier","comment","string","number","regexp","operator"],"tokenModifiers":["declaration","definition","readonly","static","deprecated","abstract","async","modification","documentation","defaultLibrary"],"formats":["relative"],"requests":{"range":true,"full":{"delta":true}},"multilineTokenSupport":false,"overlappingTokenSupport":false},"linkedEditingRange":{"dynamicRegistration":true}},"window":{"showMessage":{"messageActionItem":{"additionalPropertiesSupport":false}},"showDocument":{"support":false},"workDoneProgress":true},"general":{"regularExpressions":{"engine":"ECMAScript","version":"ES2020"},"markdown":{"parser":"marked","version":"1.1.0"}}},"trace":"on","workspaceFolders":null,"locale":"zh_CN","clientInfo":{"name":"coc.nvim","version":"0.0.80"},"workDoneToken":"41c586b8-6b12-4f53-a3ed-888fd592a547"}}"#;
        handle_lsp_message(
            String::from(init_msg),
            lsp_sender,
            conf_sender,
            team_id.clone(),
            project_id,
        )
        .await;
        false
    } else {
        true
    }
}

async fn not_client_register(lsp_msg: &str, lsp_sender: Sender<String>) -> bool {
    let p = Regex::new(r#".*"id":(\d+),"method":"client/registerCapability".*"#)
        .expect("client register re error");
    if let Some(cap) = p.captures(lsp_msg) {
        let id = &cap[1];
        let response = format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":{req_id},\"result\":{{}}}}",
            req_id = id
        );
        debug!("== got auto request =={}", lsp_msg);
        debug!("= auto == pyright ==>{}", response);
        if let Err(msg) = lsp_sender.send(response).await {
            error!("client register send to lsp failed: {}", msg);
        }
        false
    } else {
        true
    }
}
async fn not_auto_init(lsp_msg: &str, lsp_sender: Sender<String>) -> bool {
    let part = "\"id\":0,\"result\":{\"capabilities\":";
    if lsp_msg.contains(part) {
        let initd_message =
            String::from("{\"jsonrpc\": \"2.0\", \"method\": \"initialized\", \"params\": {}}\n");
        debug!("will auto initialized");
        if let Err(msg) = lsp_sender.send(initd_message).await {
            error!("send auto initialized to lsp failed: {}", msg);
        }
        false
    } else {
        true
    }
}
async fn not_workspace_config(
    lsp_msg: &str,
    lsp_sender: Sender<String>,
    conf_sender: Sender<CommandWithId>,
    team_id: String,
    project_id: String,
) -> bool {
    let p = Regex::new(r#".*"id":(\d+),"method":"workspace/configuration".*"#)
        .expect("worksapce config re error");
    if let Some(cap) = p.captures(lsp_msg) {
        let id = &cap[1];

        let lsp = get_lsp_conf(conf_sender, team_id, project_id).await;
        debug!("not_workspace_config {:#?}", lsp);

        lsp.log_self("get fresh");
        match get_python_site(lsp.python.as_str()) {
            Ok(python_site) => {
                let response = format!(
                    "{{\"jsonrpc\":\"2.0\",\"id\":{},\"result\":[{{\"autoImportCompletions\":true,\"autoSearchPaths\":true,\"extraPaths\":[\"{}\"],\"stubPath\":\"typings\",\"diagnosticMode\":\"openFilesOnly\",\"diagnosticSeverityOverrides\":{{}},\"logLevel\":\"Information\",\"typeCheckingMode\":\"{}\",\"typeshedPaths\":[\"/opt/lsp/typeshed\"],\"useLibraryCodeForTypes\":false}}]}}",
                    id, python_site, lsp.check_mode
                );
                debug!("== got auto request 2 == {}", lsp_msg);
                debug!("= auto == pyright ==> {}", response);
                debug!("current python: {}", lsp.python);
                if let Err(msg) = lsp_sender.send(response).await {
                    error!(
                        "auto check_mode, send workspace config to lsp failed: {}",
                        msg
                    );
                }
                false
            }
            Err(msg) => {
                error!("{}", msg);
                false
            }
        }
    } else {
        true
    }
}

async fn get_lsp_conf(
    conf_sender: Sender<CommandWithId>,
    team_id: String,
    project_id: String,
) -> Lsp {
    let (resp_tx, resp_rx) = oneshot::channel();
    let cmd = ConfigCommand::GetLsp { resp: resp_tx };

    conf_sender
        .send((team_id, project_id, cmd))
        .await
        .expect("get lsp conf failed");
    let lsp: Lsp = resp_rx.await.expect("unwrap lsp conf failed");
    lsp
}

async fn accept_connection(
    peer: SocketAddr,
    stream: TcpStream,
    conf_sender: Sender<CommandWithId>,
) {
    debug!("here comes new connection {}", peer);
    let (ws_stream, team_id_underscore_project_id) =
        match get_project_and_stream(peer, stream).await {
            Ok(x) => x,
            Err(err) => {
                error!("{err}");
                return;
            }
        };
    let team_id_underscore_project_id = team_id_underscore_project_id
        .strip_prefix('/')
        .unwrap_or(&team_id_underscore_project_id);
    let team_id_underscore_project_id = team_id_underscore_project_id
        .strip_prefix("executor")
        .unwrap_or(team_id_underscore_project_id);

    // info!("get project with slash #{}#", project_id);

    let (team_id, project_id) = match team_id_underscore_project_id.split_once('_') {
        Some(x) => x,
        None => {
            error!("invalid team_id_underscore_project_id {team_id_underscore_project_id}");
            return;
        }
    };
    if team_id.parse::<u64>().is_err() {
        error!("invalid team_id {team_id}");
        return;
    }
    if project_id.parse::<u64>().is_err() {
        error!("invalid project_id {project_id}");
        return;
    }
    let team_id = team_id.to_string();
    let project_id = project_id.to_string();

    /* todo: need handle projectId here
       so using different lsp:
       1, if exist, using it.
       2, if not exist, create it.
       then using the correct stream to run sender and receiver.
    */

    let lsp = get_lsp_conf(conf_sender.clone(), team_id.clone(), project_id.clone()).await;
    lsp.log_self("new connection");

    let mut cmd = Command::new(&lsp.node);
    cmd.arg(&lsp.pyright)
        .arg("--stdio")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .kill_on_drop(true);
    info!("cmd = {cmd:?}");
    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(err) => {
            panic!("{cmd:?} {err}");
        }
    };
    let stdout = child.stdout.take().expect("get command stdout failed");
    let stdin = child.stdin.take().expect("get command stdin failed");

    let (ws_out, ws_in) = ws_stream.split();

    let (ws_sender, ws_receiver) = mpsc::channel(32);
    let (lsp_sender, lsp_receiver) = mpsc::channel(32);

    let t1 = send_to_lsp(stdin, ws_sender.clone(), lsp_receiver);
    let t2 = send_to_ws(ws_out, ws_receiver);
    let t3 = lsp2ws(
        stdout,
        lsp_sender.clone(),
        conf_sender.clone(),
        ws_sender.clone(),
        team_id.clone(),
        project_id.clone(),
    );
    let t4 = ws2lsp(
        ws_in,
        lsp_sender.clone(),
        conf_sender.clone(),
        ws_sender.clone(),
        child,
        team_id.clone(),
        project_id.clone(),
    );
    let t5 = check_idle_or_need_reset(
        ws_sender,
        conf_sender.clone(),
        lsp_sender,
        chrono::Duration::minutes(30)
            .to_std()
            .expect("time idle failed"),
        team_id.clone(),
        project_id.clone(),
    );
    // TODO: on nfs, inotify can't watch the file, so need improve
    // let t6 = watch_packages_change(conf_sender.clone(), team_id.clone(), project_id.clone());
    //tokio::join!(t1, t2, t3, t4, t5);

    tokio::spawn(t3);
    tokio::spawn(t4);
    tokio::spawn(t5);
    // tokio::spawn(t6);
    tokio::spawn(t1);
    tokio::spawn(t2);
}

async fn get_project_and_stream(
    peer: SocketAddr,
    stream: TcpStream,
) -> Result<(WebSocketStream<TcpStream>, String), ErrorTrace> {
    let (header_tx, mut header_rx) = oneshot::channel();
    let ws_stream = accept_hdr_async(stream, handle_handshake(header_tx, peer)).await?;

    let uri = header_rx.try_recv()?;

    info!("get new connect uri: {}", uri);
    let project_id_with_slash = String::from(uri.path());
    // info!("uri path: {}", project_id_with_slash);
    Ok((ws_stream, project_id_with_slash))
}

async fn check_idle_or_need_reset(
    ws_sender: Sender<String>,
    conf_sender: Sender<CommandWithId>,
    lsp_sender: Sender<String>,
    idle_time: Duration,
    team_id: String,
    project_id: String,
) {
    let mut interval_timer = tokio::time::interval(
        chrono::Duration::seconds(2)
            .to_std()
            .expect("time period failed"),
    );
    loop {
        interval_timer.tick().await;
        let now = SystemTime::now();
        let need_update_time_limit = now.sub(chrono::Duration::seconds(2).to_std().expect("msg"));
        let mut lsp = get_lsp_conf(conf_sender.clone(), team_id.clone(), project_id.clone()).await;
        if lsp.visited {
            trace!("set last visit time to {:?}", now);
            if let Err(msg) = conf_sender
                .send((
                    team_id.clone(),
                    project_id.clone(),
                    ConfigCommand::RefreshVisitTime { time: now },
                ))
                .await
            {
                error!("send refresh visit time failed: {}", msg);
            }
        } else {
            trace!("not visit from last check");
            // if not visit， check if whether need reset or not

            if need_update_time_limit.gt(&lsp.need_update_time) {
                debug!("need update env to {}", lsp.env_name);
                lsp.need_update_time = thousands_days_later();
                if let Err(msg) = conf_sender
                    .send((
                        team_id.clone(),
                        project_id.clone(),
                        ConfigCommand::ResetEnv {
                            new_env: lsp.env_name,
                        },
                    ))
                    .await
                {
                    error!("send reset env failed: {}", msg);
                }
                if (lsp_sender.send(RESET_CMD.to_string()).await).is_err() {
                    warn!("send reset message failed");
                }

                if (ws_sender.send(CLOSE_CMD.to_string()).await).is_err() {
                    warn!("send reset message to ws sender failed");
                }
            } else {
                trace!("{} not update", lsp.env_name);
            }

            if now.sub(idle_time).gt(&lsp.last_visit_time) {
                warn!(
                    "idle more than {:?}, will close lsp {}:{}",
                    idle_time, lsp.team_id, lsp.project_id
                );
                if (ws_sender.send(RESET_CMD.to_string()).await).is_err() {
                    warn!("send close to ws failed");
                }
                break;
            }
        }
    }
}

async fn config_server(mut lsps: HashMap<String, Lsp>, mut conf_receiver: Receiver<CommandWithId>) {
    while let Some((team, project, cmd)) = conf_receiver.recv().await {
        if !lsps.contains_key(project.as_str()) {
            lsps.insert(project.clone(), Lsp::new(team.clone(), project.clone()));
        }
        // info!("lsps now has size {}", size_of_val(&lsps));  // its size is always 48
        // now this project and config;
        let lsp = lsps
            .get_mut(project.as_str())
            .unwrap_or_else(|| panic!("not get lsp for {}", project));
        match cmd {
            ConfigCommand::GetLsp { resp } => {
                let _ = resp.send(lsp.clone());
                lsp.log_self("request conf");
            }
            ConfigCommand::ResetEnv { new_env } => {
                debug!("remove {} to reset env to {}", project.clone(), new_env);
                lsps.remove_entry(&project.clone());
            }
            ConfigCommand::SetInited => {
                debug!("lsp initd");
                lsp.initd = true;
            }
            ConfigCommand::RefreshVisitTime { time } => {
                lsp.last_visit_time = time;
                lsp.visited = false;
            }
            ConfigCommand::Visit => {
                lsp.visited = true;
            } // ConfigCommand::SetNeedReset { time } => {
              //     debug!("handle set need reset");
              //     lsp.need_update_time = time;
              // }
        }
    }
}

pub fn build_response<T>(value: T::Params) -> String
where
    T: notification::Notification,
    T::Params: serde::Serialize,
{
    format!(
        r#"{{"jsonrpc": "2.0", "method": "{}", "params": {} }}"#,
        T::METHOD,
        serde_json::to_value(value).unwrap()
    )
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct TelemetryEventParams {
    /// The text document's URI.
    pub uri: String,

    /// The actual message
    pub message: String,

    /// Opened files
    pub opened: Vec<String>,
}

async fn send_to_lsp(
    mut stdin: ChildStdin,
    ws_sender: Sender<String>,
    mut lsp_receiver: Receiver<String>,
) {
    debug!("entry: send_to_lsp");
    let mut registered_file: Vec<String> = Vec::new();
    let init_re =
        Regex::new(r#".*"method":\s*"initialize"\s*,.*"#).expect("init initialize re error");
    // init_re.is_match(lsp_msg.as_str())
    while let Some(msg) = lsp_receiver.recv().await {
        // if msg.contains("textDocument") && !msg.contains("initialize") {
        if msg.contains("textDocument") && !init_re.is_match(msg.as_str()) {
            let _uri = msg
                .split("\"uri\":\"")
                .nth(1)
                .unwrap()
                .split('\"')
                .next()
                .unwrap();

            if !registered_file.contains(&_uri.to_string()) {
                if msg.contains("textDocument/didOpen") {
                    registered_file.push(_uri.to_string());
                    warn!("after open file{}: {:?}", _uri.to_string(), registered_file);

                    let params = TelemetryEventParams {
                        // uri: Url::parse(_uri).unwrap(),
                        uri: _uri.to_string(),
                        message: String::from(OPENED_NOTIFICATION),
                        opened: registered_file.clone(),
                    };

                    let response = build_response::<notification::TelemetryEvent>(
                        serde_json::to_value(params).unwrap(),
                    );

                    debug!("Opened notification: {}", OPENED_NOTIFICATION);
                    if let Err(msg) = ws_sender.send(response).await {
                        error!("send opened notification failed: {}", msg);
                        break;
                    }
                    debug!("ws_cap {} rest", ws_sender.capacity());
                } else {
                    let params = TelemetryEventParams {
                        // uri: Url::parse(_uri).unwrap(),
                        uri: _uri.to_string(),
                        message: String::from(ORDER_ERROR_NOTIFICATION),
                        opened: registered_file.clone(),
                    };

                    let response = build_response::<notification::TelemetryEvent>(
                        serde_json::to_value(params).unwrap(),
                    );

                    debug!("Reopen notification: {}", ORDER_ERROR_NOTIFICATION);
                    ws_sender
                        .send(response)
                        .await
                        .expect("REOPEN send to ws failed");
                    debug!("ws_cap {} rest", ws_sender.capacity());

                    continue;
                };
            }
        }

        if RESET_CMD.eq(msg.as_str()) {
            debug!("receive reset, close all");
            drop(stdin);
            drop(lsp_receiver);
            break;
        } else if PING.eq(msg.as_str()) {
            trace!("response ping->pong");
            if (ws_sender.send(PONG.to_string()).await).is_err() {
                warn!("send pong failed");
                break;
            }
        } else {
            let headed_msg = format!("Content-Length: {}\r\n\r\n{}\n", msg.len() + 1, msg);
            debug!("real->lsp {}", msg);
            // DO NOT USE `writeln!` here !!!
            // because lsp will count char error, blame that "must have a Content-Length header"
            // writeln!(stdin, "{}", headed_msg).expect("base write msg into lsp failed"); -- for std::process::ChildStdin
            if let Err(msg) = stdin.write(headed_msg.as_bytes()).await {
                error!(headed_msg, "write msg into lsp failed: {}", msg);
                break;
            }
        }
    }

    debug!("exit: send_to_lsp");
}

#[cfg(not)]
async fn watch_packages_change(
    conf_sender: Sender<CommandWithId>,
    team_id: String,
    project_id: String,
) {
    use std::path::Path;

    use notify::Error;
    use notify::Event;
    use notify::RecommendedWatcher;
    use notify::RecursiveMode;
    use notify::Watcher;
    let py_env = project2env(&team_id, &project_id).unwrap_or_else(|| "python39".to_string());
    match get_python_site(
        env2python_bin(&team2base_dir(&team_id), &py_env)
            .unwrap()
            .as_str(),
    ) {
        Ok(python_site_dir) => {
            let (tx, mut rx) = mpsc::channel(100);

            let mut watcher =
                RecommendedWatcher::new(move |result: std::result::Result<Event, Error>| {
                    tx.blocking_send(result).expect("Failed to send event");
                })
                .unwrap();
            debug!("start to monitor python site: {}", python_site_dir);

            watcher
                .watch(
                    Path::new(python_site_dir.as_str()),
                    RecursiveMode::Recursive,
                )
                .unwrap();

            // This is a simple loop, but you may want to use more complex logic here,
            // for example to handle I/O.
            while let Some(res) = rx.recv().await {
                debug!("monitor get change to {:?}", res);
                let cmd = (
                    team_id.clone(),
                    project_id.clone(),
                    ConfigCommand::SetNeedReset {
                        time: SystemTime::now(),
                    },
                );
                conf_sender
                    .send(cmd)
                    .await
                    .expect("send need reest message failed");
            }
        }
        Err(e) => {
            error!("get python site failed: {}", e);
        }
    }
}

async fn send_to_ws(
    mut ws_out: SplitSink<WebSocketStream<TcpStream>, Message>,
    mut ws_receiver: Receiver<String>,
) {
    debug!("entry: send_to_ws");
    while let Some(msg) = ws_receiver.recv().await {
        trace!("real->ws {}", msg);

        if msg.eq(RESET_CMD) {
            warn!("will send reset frame");
            if let Err(msg) = ws_out.send(RESET_FRAME).await {
                error!("send reset frame failed: {}", msg);
            }
            break;
        } else if msg.eq(CLOSE_CMD) {
            warn!("will send close frame");
            if let Err(msg) = ws_out.send(CLOSE_FRAME).await {
                error!("send close frame failed: {}", msg);
            }
        } else if let Err(msg) = ws_out.send(Message::from(msg)).await {
            error!("send to ws failed: {}", msg);
            break;
        }
    }

    debug!("exit: send_to_ws");
}

fn current_pid() -> u32 {
    process::id()
}

async fn ws2lsp(
    mut ws_in: SplitStream<WebSocketStream<TcpStream>>,
    lsp_sender: Sender<String>,
    conf_sender: Sender<CommandWithId>,
    ws_sender: Sender<String>,
    mut child: Child,
    team_id: String,
    project_id: String,
) {
    debug!("entry: ws2lsp");
    while let Some(msg1) = ws_in.next().await {
        if let Ok(msg) = msg1 {
            trace!("ws->lsp {}", msg);
            if msg.is_text() || msg.is_binary() {
                trace!("ws->txt {}", msg);
                let reset_pattern =
                    Regex::new(r"^BAIHAI_RESET_ENV\s*(.*)").expect("reset re error");
                if let Some(cap) = reset_pattern.captures(&msg.to_string()) {
                    conf_sender
                        .send((
                            team_id.clone(),
                            project_id.clone(),
                            ConfigCommand::ResetEnv {
                                new_env: String::from(&cap[1]),
                            },
                        ))
                        .await
                        .expect("send reset env failed");
                    ws_sender
                        .send(RESET_CMD.to_string())
                        .await
                        .expect("send close command failed");
                } else {
                    // not a reset command
                    if !handle_lsp_message(
                        msg.to_string(),
                        lsp_sender.clone(),
                        conf_sender.clone(),
                        team_id.clone(),
                        project_id.clone(),
                    )
                    .await
                    {
                        warn!("send into lsp failed, will close");
                        break;
                    }
                }
            }
        } else {
            debug!("ws-> unwrap msg failed.");
        }
    }
    // come here, client closed, so close all sender
    debug!("exit: ws2lsp");
    if let Err(msg) = child.kill().await {
        error!("kill child failed: {}", msg);
    }
}
async fn lsp2ws(
    stdout: ChildStdout,
    lsp_sender: Sender<String>,
    conf_sender: Sender<CommandWithId>,
    ws_sender: Sender<String>,
    team_id: String,
    project_id: String,
) {
    debug!("entry: lsp2ws");
    let mut reader = BufReader::new(stdout);
    let mut lines = Vec::new();
    loop {
        let mut buffer = Vec::with_capacity(81920);
        // using while let usize = reader.read_buf(&mut buffer).await.expect() is more effective, but the following
        // buffer.clear() will blame: cant using buffer both as mutable and immutable.
        let usize = reader
            .read_buf(&mut buffer)
            .await
            .expect("read to end failed");
        if usize == 0 {
            break;
        }
        lines.push(buffer);
        let msg_u8 = lines.concat();

        if complete_u8(&msg_u8) {
            let msg_stuf = String::from_utf8(msg_u8).expect("from u8 to String failed");
            debug!("lines {}", lines.len());
            debug!("---------------------------------");
            lines.clear();
            // buffer.clear();  // todo: can't clear here, but need clear
            let header_pattern = Regex::new(r"Content-Length:\s+\d+\s*").expect("header re error");
            let parts: Vec<_> = header_pattern.split(msg_stuf.as_str()).collect();
            debug!("{} contains {} message", msg_stuf, parts.len());
            for msg_with_new_line in parts {
                let msg = msg_with_new_line.trim();
                if !msg.is_empty() {
                    debug!("lsp->ws: {}", msg);
                    let json_message = String::from(msg);
                    if complete_u8(json_message.as_bytes()) {
                        if message_need_forward(
                            json_message.as_str(),
                            lsp_sender.clone(),
                            conf_sender.clone(),
                            team_id.clone(),
                            project_id.clone(),
                        )
                        .await
                        {
                            //let new_msg = reduce_completion_number(&json_message);
                            debug!("->ws #{}#", json_message);
                            if let Err(err) = ws_sender.send(json_message).await {
                                error!("send msg to ws {err}");
                            }
                            debug!("ws_cap {} rest", ws_sender.capacity());
                        }
                    } else {
                        debug!("skip invalidate msg {}", json_message.clone());
                    }
                } else {
                    debug!("empty message");
                }
            }
        } else {
            // message partly
        }
    }
    debug!("exit: lsp2ws");
}

pub async fn main_(args: Vec<String>) {
    if let Some(val) = args.get(1) {
        if val == "--version" {
            println!("{}", GIT_VERSION);
            return;
        }
    }
    let mut port = 7777;
    if let Some(arg1) = args.get(1) {
        if arg1 == "--port" {
            if let Some(arg2) = args.get(2) {
                port = arg2.parse().unwrap();
            }
        }
    }
    logger::init_logger();
    let listener = TcpListener::bind(&(std::net::Ipv4Addr::UNSPECIFIED, port))
        .await
        .unwrap();

    let lsps: HashMap<String, Lsp> = HashMap::new();
    let (conf_sender, conf_receiver) = mpsc::channel(32);
    tokio::spawn(config_server(lsps, conf_receiver));
    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        debug!("Peer address: {}", peer);
        tokio::spawn(accept_connection(peer, stream, conf_sender.clone()));
    }
}
