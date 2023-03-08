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

use kernel_common::Message;
use tracing::debug;

use super::py_stdin::PyStdin;
use super::py_stdout_stderr::PyStdoutStderr;

pub fn init_python(
    iopub_sender: std::sync::mpsc::Sender<Message>,
    input_reply_receiver: std::sync::mpsc::Receiver<String>,
    header: &kernel_common::Header,
    py: pyo3::Python,
) -> PythonDefines {
    let start = std::time::Instant::now();
    #[cfg(unix)]
    super::init_signal_handler::init_signal_handler(py);
    debug!("after init signal, time cost = {:?}", start.elapsed());

    let sys = py.import("sys").unwrap();
    // sys.setattr("kernel_info", format!("{:#?}", kernel_info)).unwrap();
    sys.setattr(
        "stdout",
        pyo3::PyCell::new(py, PyStdoutStderr {
            sender: iopub_sender.clone(),
            stdout_or_stderr: "stdout",
            header: header.clone(),
            buf: String::new(),
            is_busy: true,
        })
        .unwrap(),
    )
    .unwrap();
    sys.setattr(
        "stderr",
        pyo3::PyCell::new(py, PyStdoutStderr {
            sender: iopub_sender.clone(),
            stdout_or_stderr: "stderr",
            header: header.clone(),
            buf: String::new(),
            is_busy: true,
        })
        .unwrap(),
    )
    .unwrap();
    sys.setattr(
        "stdin",
        pyo3::PyCell::new(py, PyStdin {
            input_request_sender: iopub_sender,
            header: header.clone(),
            input_reply_receiver,
        })
        .unwrap(),
    )
    .unwrap();
    debug!("after set stdout/stderr, time cost = {:?}", start.elapsed());
    // ssl._create_default_https_context = ssl._create_unverified_context

    // add current dir to sys.path
    // sys.path.call_method("insert", (0, ""), None).unwrap();
    let sys_path = sys
        .getattr("path")
        .unwrap()
        .downcast::<pyo3::types::PyList>()
        .unwrap();
    sys_path.insert(0, "").unwrap();
    #[cfg(unix)]
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    #[cfg(windows)]
    let home_dir = std::env::var("HOMEPATH").unwrap();
    let custom_packages_dir = format!("{home_dir}/.idp/custom_python_packages");
    if !std::path::Path::new(&custom_packages_dir).exists() {
        std::fs::create_dir_all(&custom_packages_dir).unwrap();
    }
    sys_path.insert(0, custom_packages_dir).unwrap();

    let kernel_helper = pyo3::types::PyModule::from_code(
        py,
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/kernel_helper.py")),
        "kernel_helper.py",
        "kernel_helper",
    )
    .unwrap();
    py.run(
        "import kernel_helper
def get_ipython():
    return kernel_helper.IdpInteractiveShell()
kernel_helper.init_ipython_display()",
        None,
        None,
    )
    .unwrap();

    let startup_folder_path = format!(
        "/store/{}/projects/{}/startup",
        header.team_id, header.project_id
    );
    let startup_folder_path = std::path::Path::new(&startup_folder_path);
    if let Ok(startup_folder) = std::fs::read_dir(startup_folder_path) {
        for file in startup_folder.into_iter().flatten() {
            let abs_path = startup_folder_path.join(file.path());
            if abs_path.is_file() {
                if let Ok(content) = std::fs::read_to_string(&abs_path) {
                    if let Err(err) = py.run(&content, None, None) {
                        tracing::error!("{abs_path:?} {err}");
                    }
                }
            }
        }
    }

    #[cfg(not)]
    if !business::kubernetes::is_k8s() {
        if let Ok(baihai_matplotlib_backend) = pyo3::types::PyModule::from_code(
            py,
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/baihai_matplotlib_backend.py"
            )),
            "baihai_matplotlib_backend.py",
            "baihai_matplotlib_backend",
        ) {
            // No module named 'kernel_helper.baihai_matplotlib_backend'; 'kernel_helper' is not a package
            sys.getattr("modules")
                .unwrap()
                .downcast::<pyo3::types::PyDict>()
                .unwrap()
                .set_item("baihai_matplotlib_backend", baihai_matplotlib_backend)
                .unwrap();
        } else {
            tracing::warn!("matplotlib not install, skip baihai_matplotlib_backend init")
        }
    }

    debug!("end init_python, time cost = {:?}", start.elapsed());
    PythonDefines {
        func_ast_parse: kernel_helper.getattr("func_ast_parse").unwrap().into(),
        // func_cvt_figs_to_graphic_obj: kernel_helper
        //     .getattr("cvt_figs_to_graphic_obj")
        //     .unwrap()
        //     .into(),
        cvt_magic_code: kernel_helper.getattr("cvt_magic_code").unwrap().into(),
        load_or_skip: kernel_helper.getattr("load_or_skip").unwrap().into(),
        after_run: kernel_helper.getattr("after_run").unwrap().into(),
    }
}

#[derive(Clone)]
pub struct PythonDefines {
    pub func_ast_parse: pyo3::Py<pyo3::PyAny>,
    // pub func_cvt_figs_to_graphic_obj: pyo3::Py<pyo3::PyAny>,
    pub cvt_magic_code: pyo3::Py<pyo3::PyAny>,
    pub load_or_skip: pyo3::Py<pyo3::PyAny>,
    pub after_run: pyo3::Py<pyo3::PyAny>,
}
