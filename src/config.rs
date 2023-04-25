use std::path::{Path, PathBuf};
pub struct Config {
    debug  : bool,
    indir  : PathBuf,
    outdir : PathBuf,
}

impl Config {
    pub fn new(indir : String, outdir : String) -> Self {
        Config {
            debug : false,
            indir : PathBuf::from(indir),
            outdir : PathBuf::from(outdir)
        }
    }
    pub fn set_debug(&mut self) {
        self.debug = true;
    }
    pub fn state_init(&mut self) {
        
    }
}