use nix::unistd;
use core::ffi::CStr;
use std::{f32::consts, ffi::c_char};


use crate::{act, err};

pub struct ForkServer {
    target_path : String,
    argv        : Vec<String>,
}
impl ForkServer {
    fn argv_c_vec(&self) -> Vec<&CStr> {
        self.argv.iter()
                 .map(|it| unsafe {
                    let ptr = it.as_str().as_ptr() as *const c_char ;
                    CStr::from_ptr(ptr)
                 })
                 .collect::<Vec<&CStr>>()
    }
}

impl ForkServer {
    pub fn exec_child(&self) {

        act!("execute child : {}", self.target_path);

        let path : &CStr;
        unsafe {
            let ptr = self.target_path.as_str().as_ptr() as *const c_char;
            path = CStr::from_ptr(ptr);
        }

        let argv = self.argv_c_vec();

        unistd::execv(path, argv.as_slice()).unwrap_or_else(|err|{
            err!("execv failed with : {}", err);
        });
    }
}