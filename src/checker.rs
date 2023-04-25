use std::io::Read;
use std::path::Path;
use std::fs;

use crate::{err, act, warn};
use crate::state::State;

fn getenv(var_name: &str) -> Option<String> {
    match std::env::var(var_name) {
        Ok(val) => Some(val),
        Err(_) => None,
    }
}

pub fn check_asan_opts() {

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

    let mut fd = match std::fs::File::open("/proc/sys/kernel/core_pattern") {
        Ok(file) => file,
        Err(_) => return,
    };
    
    act!("Checking core_pattern...");

    let mut fchar = [0u8; 1].to_vec();

    if fd.read_to_end(&mut fchar)
         .map(|bytes| bytes == 1 && fchar[0] == b'|')
         .unwrap_or(false) 
    {

        err!("maybe run : echo core >/proc/sys/kernel/core_pattern")
    }
}

pub fn check_cpu_governor() {
    warn!("not impl check_cpu_governor yet");
} 

pub fn get_core_count() {
    warn!("not impl get_core_count yet");
}

pub fn check_binary(fpath : &Path) {
    /* ELF valication + AFL instrument evidence */
    act!("Validating target binary...");
    
    if !fpath.exists() {
        err!("Error: File does not exist.");
    }

    use fs::File;
    let mut file : File = match fs::File::open(fpath) {
        Err(e) => {
            err!("Error opening file: {}", e);
            unreachable!()
        }
        Ok(f) => f,
    };

    let mut header = [0; 16];

    // read first 16 bytes
    match file.read_exact(&mut header) {
        Err(e) => {
            err!("Error reading file: {}", e);
            unreachable!()
        }
        Ok(_) => (),
    }

    // 检查比较所有 16 个字节是否符合规范
    if header != [0x7F, b'E', b'L', b'F', 2, 1, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0] {
        err!("Error: Invalid ELF header.");
    }
}