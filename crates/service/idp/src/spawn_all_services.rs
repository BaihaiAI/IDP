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

use std::path::Path;

use crate::cli_args::CliArgs;

struct PanicGuard {
    terminal_shutdown_tx: std::sync::mpsc::Sender<()>,
}
impl Drop for PanicGuard {
    fn drop(&mut self) {
        if std::thread::panicking() {
            eprintln!(
                "FATAL: {:?} thread panic! exit process now",
                std::thread::current().name()
            );
            // if thread panic, terminal would not exit
            if let Err(err) = self.terminal_shutdown_tx.send(()) {
                tracing::error!("{err}");
            };
            std::process::exit(1);
        }
    }
}

/*
struct KillOnDrop(std::process::Child);
impl Drop for KillOnDrop {
    fn drop(&mut self) {
        if let Err(err) = self.0.kill() {
            tracing::error!("{err}");
        }
    }
}
*/

pub fn spawn_all_services(args: &CliArgs) {
    args.write_env();
    let gateway_exe_path = std::env::current_exe().unwrap();
    let exe_parent_dir = gateway_exe_path.parent().unwrap();
    let exe_parent_dir_str = exe_parent_dir.to_str().unwrap();
    let mut nodejs_path = None;
    for path in [
        #[cfg(unix)]
        format!("{}/lsp/node/bin/node", exe_parent_dir_str),
        #[cfg(windows)]
        format!(r"{}\lsp\node\node.exe", exe_parent_dir_str),
        "/opt/lsp/node/bin/node".to_string(),
    ] {
        let path = Path::new(&path);
        if path.exists() {
            nodejs_path = Some(path.to_path_buf());
            break;
        }
    }
    let nodejs_path = nodejs_path.unwrap_or_else(|| Path::new("node").to_path_buf());
    tracing::info!("nodejs_path = {nodejs_path:?}");
    // let mut cmd = std::process::Command::new(&nodejs_path);
    // cmd.arg("--version");
    // let output = cmd.output().unwrap_or_else(|_| panic!("err on {cmd:?}"));
    // assert!(output.status.success(), "nodejs --version err");
    // if String::from_utf8_lossy(&output.stdout).trim_end() < "v16.0.0" {
    //     tracing::warn!(
    //         "nodejs version {} < 16",
    //         String::from_utf8_lossy(&output.stdout)
    //     );
    // }
    std::env::set_var("NODE_BIN", &nodejs_path);
    if exe_parent_dir
        .join("lsp")
        .join("pyright")
        .join("server.js")
        .is_file()
    {
        std::env::set_var(
            "PY_RIGHT_SERVER_JS",
            exe_parent_dir.join("lsp").join("pyright").join("server.js"),
        );
    } else if Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../typescript-lsp/packages/vscode-pyright/pyright/server.js"
    ))
    .exists()
    {
        std::env::set_var(
            "PY_RIGHT_SERVER_JS",
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../../typescript-lsp/packages/vscode-pyright/pyright/server.js"
            ),
        );
    }

    let mut terminal_path = None;
    for path in [
        exe_parent_dir
            .join("terminal")
            .join("src")
            .join("server.js"),
        Path::new(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../web/terminal/src/server.js"
        ))
        .to_path_buf(),
        Path::new("/opt/terminal/src/server.js").to_path_buf(),
    ] {
        if path.exists() {
            terminal_path = Some(path.canonicalize().unwrap());
            break;
        }
    }
    let terminal_port = args.terminal_port;
    let (terminal_shutdown_tx, terminal_shutdown_rx) = std::sync::mpsc::channel::<()>();
    let _terminal_handle = std::thread::Builder::new()
        .name("terminal".to_string())
        .spawn(move || {
            let mut cmd = std::process::Command::new(nodejs_path);
            cmd.arg(terminal_path.unwrap())
                .arg("--port")
                .arg(terminal_port.to_string());
            tracing::info!("terminal cmd = {cmd:?}");
            let mut handle = cmd.spawn().unwrap();
            while let Ok(()) = terminal_shutdown_rx.recv() {
                if let Err(err) = handle.kill() {
                    tracing::error!("{err}");
                }
            }
        })
        .unwrap();

    let terminal_shutdown_tx_clone = terminal_shutdown_tx.clone();
    std::thread::Builder::new()
        .name("kernel_manage".to_string())
        .spawn(|| {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let _panic_guard = PanicGuard {
                        terminal_shutdown_tx: terminal_shutdown_tx_clone,
                    };
                    kernel_manage::main().await;
                });
        })
        .unwrap();

    let lsp_port = args.lsp_port;
    std::thread::Builder::new()
        .name("lsp".to_string())
        .spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    // ignore lsp panic
                    // let _panic_guard = PanicGuard {};
                    lsp::main_(vec![
                        "".to_string(),
                        "--port".to_string(),
                        lsp_port.to_string(),
                    ])
                    .await;
                });
        })
        .unwrap();
    let terminal_shutdown_tx_clone = terminal_shutdown_tx.clone();
    std::thread::Builder::new()
        .name("redis_server".to_string())
        .spawn(|| {
            let _panic_guard = PanicGuard {
                terminal_shutdown_tx: terminal_shutdown_tx_clone,
            };
            redis_server::main();
        })
        .unwrap();
    // let terminal_shutdown_tx_clone = terminal_shutdown_tx.clone();
    std::thread::Builder::new()
        .name("note_storage".to_string())
        .spawn(move || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let _panic_guard = PanicGuard {
                        terminal_shutdown_tx: terminal_shutdown_tx.clone(),
                    };
                    note_storage::main().await;
                });
        })
        .unwrap();
}
