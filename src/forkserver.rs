use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::{setsid, dup2, close};
use nix::sys::signal::SIGTERM;
use nix::unistd::Pid;
use nix::unistd;

use core::ffi::CStr;


use libc::{SIGPIPE, sigaction, c_void};
use libc::c_int;

use std::os::unix::io::IntoRawFd;
use std::ffi::c_char;

use crate::{act, err};

pub struct ForkServer {
    target_path : String,
    argv        : Vec<String>,
    trace_bits  : u32,
    fsrv_ctl_fd : i32,
    fsrv_st_fd  : i32,
    init_tmout  : u32,
    fsrv_pid    : i32
}

const FORKSRV_FD : i32 = 198;
const EXEC_FAIL_SIG : u32 =  0xfee1dead;
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
fn temp_handle_signal() {
    panic!("SIGPIPE detect")
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

    pub fn init_child_func(&self) {
        unimplemented!()
    }

    pub fn start(&mut self) {

        let (st_pipe_read, st_pipe_write) = nix::unistd::pipe().unwrap();
        let (ctl_pipe_read, ctl_pipe_write) = nix::unistd::pipe().unwrap();
    
        // fsrv.last_run_timed_out = false;
    
        let fsrv_pid = unsafe { libc::fork() };
        match fsrv_pid {
            -1 => panic!("fork() failed"),
            0 => { /*  CHILD PROCESS */
    
                // Create a SigAction instance
                // more check here
                let mut sig_act: sigaction = unsafe { std::mem::zeroed() };

                // Set handler function
                sig_act.sa_sigaction = temp_handle_signal as usize;

                // Clear and add signals to block
                unsafe {
                    // Clear the signal mask
                    libc::sigemptyset(&mut sig_act.sa_mask as *mut libc::sigset_t);
                    
                    // Add SIGTERM signal to mask
                    libc::sigaddset(&mut sig_act.sa_mask as *mut libc::sigset_t, SIGTERM as c_int);
                }
                unsafe { sigaction(SIGPIPE, &sig_act, std::ptr::null_mut()) };
    
                /* Dumping cores is slow and can lead to anomalies if SIGKILL is delivered
                    before the dump is complete. */
    
                /* Isolate the process and configure standard descriptors. If out_file is
                    specified, stdin is /dev/null; otherwise, out_fd is cloned instead. */
    
                setsid().unwrap();// TODO:
    
                if false /*  debug_child_output */{
    
                    let dev_null_fd = std::fs::File::open("/dev/null").unwrap();
                    let dev_null_fd_raw = dev_null_fd.into_raw_fd();
                    dup2(dev_null_fd_raw, 1).unwrap();
                    dup2(dev_null_fd_raw, 2).unwrap();
    
                }
    
                // if !fsrv.use_stdin {
                // if false {
    
                //     let dev_null_fd = std::fs::File::open("/dev/null").unwrap();
                //     let dev_null_fd_raw = dev_null_fd.into_raw_fd();
                //     unsafe { dup2(dev_null_fd_raw, 0) };
    
                // } else {
    
                //     let out_fd_raw = fsrv.out_fd;
                //     unsafe { dup2(out_fd_raw, 0) };
                //     close(fsrv.out_fd).unwrap();
    
                // }
    
                /* Set up control and status pipes, close the unneeded original fds. */
    
                dup2(ctl_pipe_read, FORKSRV_FD).unwrap();
                dup2(st_pipe_write, FORKSRV_FD + 1).unwrap();
    
                close(ctl_pipe_read).unwrap();
                close(ctl_pipe_write).unwrap();
                close(st_pipe_read).unwrap();
                close(st_pipe_write).unwrap();
    
                // close(fsrv.out_dir_fd).unwrap();
                // close(fsrv.dev_null_fd).unwrap();
                // close(fsrv.dev_urandom_fd).unwrap();
    
                // if fsrv.plot_file.is_some() {
    
                //     let mut plot_file = fsrv.plot_file.take().unwrap();
                //     std::mem::drop(plot_file);
    
                // }
    
                /* Set sane defaults for sanitizers */
                // set_sanitizer_defaults();
    
                self.init_child_func();
    
                /* Use a distinctive bitmap signature to tell the parent about execv()
                    falling through. */
    
                // let exec_fail_sig: u32 = EXEC_FAIL_SIG;
                // unsafe { *(fsrv.trace_bits as *mut u32) = exec_fail_sig };
                self.trace_bits = EXEC_FAIL_SIG;
                panic!("Error: execv to target failed\n");
    
            }
            _ => { /* PARENT PROCESS */
    
                let pid_buf = fsrv_pid.to_string();
                // if fsrv.cmplog_binary {
                //     std::env::set_var("__AFL_TARGET_PID2", pid_buf);
                // } else {
                //     std::env::set_var("__AFL_TARGET_PID1", pid_buf);
                // }
    
                close(ctl_pipe_read).unwrap();
                close(st_pipe_write).unwrap();
    
                self.fsrv_ctl_fd = ctl_pipe_write;
                self.fsrv_st_fd = st_pipe_read;
    
                /* Wait for the fork server to come up, but don't wait too long. */
    
                let init_timeout = self.init_tmout;// TODO

                if init_timeout > 0 {
                    let start_time = std::time::Instant::now();
                    loop {
                        let mut buf = [0u8; 4];
                        let num = unsafe{
                            libc::read(self.fsrv_ctl_fd, buf.as_mut_ptr() as *mut c_void, buf.len())
                        };

                        match num {
                            4 => return,
                            _ => {
                                err!("imcomplete read")
                            },
                        }
                        // if start_time.elapsed().as_secs() > init_timeout as u64 {
                        //     break;
                        // } else {
                        //     thread::sleep(time::Duration::from_millis(50));
                        // }
                    }
                }
    
                /* If we have a four-byte "hello" message from the server, we're all set.
                    Otherwise, try to figure out what went wrong. */
    
                let mut buf = [0u8; 4];
                let num = unsafe{
                    libc::read(self.fsrv_ctl_fd, buf.as_mut_ptr() as *mut c_void, buf.len())
                };
                
                if num == 4 {
                    // All good
                }else {
                    panic!()
                }
                
                let pid : Pid = Pid::from_raw(self.fsrv_ctl_fd);
                match waitpid(pid, None) {
                    Ok(WaitStatus::Exited(_, _)) => {
                        panic!("Fork server handshake failed");
                    }
                    Ok(WaitStatus::Signaled(_, _, _)) => {
                        panic!("Fork server handshake failed");
                    }
                    Err(e) => {
                        panic!("waitpid() failed with {}" ,e);
                    }
                    _ => {unimplemented!()}
                }
            }
        }

    }
}