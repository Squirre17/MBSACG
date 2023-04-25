/* keep afl file hierarchy in mind
├── crashes
│   ├── id:000000,sig:06,src:000000,op:flip1,pos:0
│   ├── id:000001,sig:06,src:000000,op:flip1,pos:4
│   ├── id:000002,sig:06,src:000000,op:flip1,pos:5
│   └── README.txt
├── fuzz_bitmap
├── fuzzer_stats
├── hangs
├── plot_data
└── queue
    ├── id:000000,orig:echo
    ├── id:000001,orig:ls
    ├── id:000002,src:000000,op:flip1,pos:56
    └── id:000003,src:000000,op:flip1,pos:57
 */
pub mod reader {
    use std::path::Path;
    use crate::state::State;

    pub fn read_testcases(afl : State, directory : &Path) {
        /* open all raw files in directory which is original */
        unimplemented!();
    }
    pub fn setup_stdio_file() {
        /* Setup the output file for fuzzed data, if not using -f(in aflpp). */
        /*
            use file as fd or stdin
         */
        unimplemented!()
    }
}