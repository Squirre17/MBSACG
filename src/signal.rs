use std::{
    mem::MaybeUninit,
    os::raw::{c_int, c_void},
};
use libc::{memset, sigaction, sighandler_t, sigemptyset};

// extern "C" {
//     fn handle_stop_sig(_: c_int);
//     fn handle_resize(_: c_int);
//     fn handle_skipreq(_: c_int);
//     fn signal_ignore(_ : c_int);
// }
#[no_mangle]
extern "C" fn handle_stop_sig(_: c_int) {
    unimplemented!()
}

#[no_mangle]
extern "C" fn handle_resize(_: c_int) {
    unimplemented!()
}

#[no_mangle]
extern "C" fn handle_skipreq(_: c_int) {
    unimplemented!()
}

#[no_mangle]
extern "C" fn signal_ignore(_ : c_int) {
    unimplemented!()
}



/* TODO: imcomplete */
pub fn setup_signal_handlers() {

    unsafe {

        let mut sa: MaybeUninit<libc::sigaction> = MaybeUninit::uninit();
        
        memset(
            sa.as_mut_ptr() as *mut c_void,
            0,
            std::mem::size_of::<libc::sigaction>(),
        );
        let mut sa = sa.assume_init();

        // Various ways of saying "stop".
        sa.sa_sigaction = handle_stop_sig as *mut c_void as sighandler_t;
        sigemptyset(&mut sa.sa_mask);
        sigaction(libc::SIGHUP, &sa, std::ptr::null_mut());
        sigaction(libc::SIGINT, &sa, std::ptr::null_mut());
        sigaction(libc::SIGTERM, &sa, std::ptr::null_mut());

        // Window resize.
        sa.sa_sigaction = handle_resize as *mut c_void as sighandler_t;
        sigaction(libc::SIGWINCH, &sa, std::ptr::null_mut());

        // SIGUSR1: skip entry.
        sa.sa_sigaction = handle_skipreq as *mut c_void as sighandler_t;
        sigaction(libc::SIGUSR1, &sa, std::ptr::null_mut());

        // Things we don't care about.
        sa.sa_sigaction = signal_ignore as *mut c_void as sighandler_t;
        sigaction(libc::SIGTSTP, &sa, std::ptr::null_mut());
        sigaction(libc::SIGPIPE, &sa, std::ptr::null_mut());
    }
}