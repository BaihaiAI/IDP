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

// ask linux kernel by prctl syscall to send sigterm if parent process die in development environment
// if in release build when pause kernel pid would change by ptrace, kernel parent pid is 1 after resume
#[cfg(unix)]
pub const INTERRUPT_SIGNAL: libc::c_int = libc::SIGUSR1;
pub fn init_signal_handler(py: pyo3::Python) {
    // let syscall_resp = unsafe { libc::prctl(libc::PR_SET_PDEATHSIG, libc::SIGPWR) };

    // let syscall_resp = unsafe { libc::signal(INTERRUPT_SIGNAL, libc::SIG_IGN) };
    // #[cfg(unix)]
    // if syscall_resp == libc::SIG_ERR {
    //     panic!("{}", std::io::Error::last_os_error());
    // }

    // init_python_signal_handler because we has overwrite process default signal handler,
    // and python would not keep default sigint behavior(raise KeyboardInterrupt)
    py.eval(
        r#"__import__('signal').signal(
        __import__('signal').SIGUSR1,
        lambda signum, frame : (_ for _ in ()).throw(KeyboardInterrupt(frame))
    )"#,
        None,
        None,
    )
    .unwrap();
}

#[cfg(test)]
const N_TIMES: u128 = 10;

#[test]
#[ignore = "signal only works in main thread"]
fn test_init_signal_code_1_eval_performance() {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();

    let mut time_cost_list = Vec::new();
    for _ in 0..N_TIMES {
        let start = std::time::Instant::now();
        py.eval(
            r#"__import__('signal').signal(
            __import__('signal').SIGINT,
            lambda signum, frame : (_ for _ in ()).throw(KeyboardInterrupt(frame))
        )"#,
            None,
            None,
        )
        .unwrap();
        time_cost_list.push(start.elapsed().as_micros());
        dbg!(start.elapsed());
    }
    dbg!(time_cost_list.into_iter().sum::<u128>() / N_TIMES);
}

#[test]
#[ignore = "signal only works in main thread"]
fn test_init_signal_code_1_run_performance() {
    pyo3::prepare_freethreaded_python();
    let gil = pyo3::Python::acquire_gil();
    let py = gil.python();

    let mut time_cost_list = Vec::new();
    for _ in 0..N_TIMES {
        let start = std::time::Instant::now();
        py.eval(
            r#"__import__('signal').signal(
            __import__('signal').SIGINT,
            lambda signum, frame : (_ for _ in ()).throw(KeyboardInterrupt(frame))
        )"#,
            None,
            None,
        )
        .unwrap();
        time_cost_list.push(start.elapsed().as_micros());
        dbg!(start.elapsed());
    }
    dbg!(time_cost_list.into_iter().sum::<u128>() / N_TIMES);
}
