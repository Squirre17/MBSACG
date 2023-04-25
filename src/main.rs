mod log;
mod config;
mod forkserver;
mod signal;
mod state;
mod random;
mod checker;
// use crate::log::*;
use config::Config;
use forkserver::ForkServer;
use clap::{arg, command, value_parser, ArgAction, Command};


fn usage() {

}
fn get_afl_env(var_name: &str) -> Option<bool> {
    match std::env::var(var_name) {
        Ok(val) => Some(val == "1" || val.to_lowercase() == "true"),
        Err(_) => None,
    }
}
fn main() {

    let matches = Command::new("MBSACG")
        .version("1.0")
        .author("Squ17. <ler2sq@gmail.com>")
        .about("Module-based struct-aware coverage-guided fuzz")
        .arg(arg!(-i --in <VALUE> "input directory with test cases").required(true))
        .arg(arg!(-o --out <VALUE> "output directory for fuzzer findings").required(true))
        .arg(arg!(-f --file-feed <VALUE> "feed program with file(i.e @@").required(false))
        .get_matches();

    act!("mbsacg by <test@gmail.com>");

    let mut conf = Config::new(
        matches.get_one::<String>("in").expect("required").to_string(),
        matches.get_one::<String>("out").expect("required").to_string()
    );
    
    if get_afl_env("AFL_DEBUG") == Some(true) {
        conf.set_debug();
    }
    
    random::rand_set_seed();
    signal::setup_signal_handlers();
    checker::check_asan_opts();

    act!("start to work");

    





    
}
