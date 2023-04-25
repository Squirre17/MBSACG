use crate::{err};
use crate::state::State;

fn getenv(var_name: &str) -> Option<String> {
    match std::env::var(var_name) {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}

pub fn check_asan_opts(state : State) {

    let asan = getenv("ASAN_OPTIONS");

    if let Some(asan_inner) = asan {
        if !asan_inner.contains("abort_on_error=1") {
            err!("Custom ASAN_OPTIONS set without abort_on_error=1 - please fix!");
        }
    }

    let msan = getenv("MSAN_OPTIONS");

    if let Some(msan_inner) = msan {
        if !msan_inner.contains("exit_code=") {
            err!("Custom MSAN_OPTIONS set without exit_code=MSAN_ERROR - please fix!");
        }
    }

    // NOT SUPPORT YET
    // let x = get_afl_env("LSAN_OPTIONS");

    // if let Some(x_val) = x {
    //     if !x_val.contains("symbolize=0") {
    //         FATAL("Custom LSAN_OPTIONS set without symbolize=0 - please fix!");
    //     }
    // }
} 

/* check /proc/sys/kernel/core_pattern */
pub fn check_crash_handling() {
    unimplemented!()
}

pub fn check_cpu_governor() {
    unimplemented!()
} 

pub fn get_core_count() {
    unimplemented!()
}

pub fn check_binary() {
    /* ELF valication + AFL instrument evidence */
    unimplemented!()
}