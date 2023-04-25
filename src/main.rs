mod forkserver;
mod checker;
mod config;
mod signal;
mod random;
mod mutate;
mod bitmap;
mod shmem;
mod state;
mod fuzz;
mod log;
mod io;
mod ui;

// crate
use crate::{io::read::reader, state::State};
use crate::forkserver::ForkServer;
use crate::fuzz::executor::Fuzzer;
use crate::io::write::writer;
use crate::config::Config;
use crate::ui::shower;

// std
use std::sync::Mutex;

// external
use clap::{arg, command, value_parser, ArgAction, Command};
use lazy_static::lazy_static;
use libc::SHM_UNLOCK;

fn usage() {

}
fn get_afl_env(var_name: &str) -> Option<bool> {
    match std::env::var(var_name) {
        Ok(val) => Some(val == "1" || val.to_lowercase() == "true"),
        Err(_) => None,
    }
}

// temp
fn register_atexit(){
    unimplemented!()
}
static mut CONF: Option<Config> = None;
lazy_static! {
    static ref GLOBAL_CONF: Mutex<Option<Config>> = Mutex::new(None);
}

fn main() {

    let matches = Command::new("MBSACG")
        .version("1.0")
        .author("Squ17. <ler2sq@gmail.com>")
        .about("Module-based struct-aware coverage-guided fuzz")
        .arg(arg!(-i --in <VALUE> "input directory with test cases").required(true))
        .arg(arg!(-o --out <VALUE> "output directory for fuzzer findings").required(true))
        .arg(arg!(-f --"file-feed" <VALUE> "feed program with file(i.e @@").required(false))
        .get_matches();

    act!("mbsacg by <test@gmail.com>");

    let mut conf = Config::new(
        matches.get_one::<String>("in").expect("required").to_string(),
        matches.get_one::<String>("out").expect("required").to_string()
    );

    if get_afl_env("AFL_DEBUG") == Some(true) {
        conf.set_debug();
    }

    *GLOBAL_CONF.lock().unwrap() = Some(conf);

    let state = State{};
    
    random::rand_set_seed();
    signal::setup_signal_handlers();
    checker::check_asan_opts();
    checker::check_crash_handling();
    checker::check_cpu_governor();
    checker::get_core_count();
    checker::check_binary();

    register_atexit();
    act!("start to work");

    reader::read_testcases(todo!(), todo!());
    writer::pivot_inputs();

    reader::setup_stdio_file();

    shmem::setup_testcase_shmem();

    let fuzz = Fuzzer{};
    fuzz.perform_dry_run();
    fuzz.cull_queue();
    /* seed validation veirfy */

    shower::show_init_stats()






    
}
