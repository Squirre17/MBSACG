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

const MAX_FILE : u64 = 1 * 1024 * 1024;


pub mod reader {
    
    use std::io::{BufRead, BufReader};
    use std::path::Path;
    use std::fs;
    
    use crate::mutcase::{MutCases, MutCase};
    use crate::state::State;
    use crate::err;

    pub fn read_testcases(afl : &State, que : &mut MutCases ,directory : &Path) {
        /* open all raw files in directory which is original */

        if let Ok(files) = fs::read_dir(directory) {
    
            files.filter_map(Result::ok) // filter invaild files
                    .map(|file| (file.path(), file.file_name().to_owned()))
                    .filter_map(|(path, filename)| {
    
                        let size = fs::metadata(&path).ok()?.len();
                        if size > super::MAX_FILE {
                            err!("file {:?} pass over file limit", filename);
                        }
    
                        Some((path.into_os_string().into_string().unwrap(), size as usize))
                        
                    }).for_each(|(fullpath, size)| {
                        que.add(fullpath, size);
                    });
        }
    }
        
    pub fn setup_stdio_file() {
        /* Setup the output file for fuzzed data, if not using -f(in aflpp). */
        /*
            use file as fd or stdin
         */
        unimplemented!()
    }
}