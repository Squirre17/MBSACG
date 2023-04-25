/* parse argument after -- */

use crate::err;

/// target program argument and @@ position
#[derive(Debug)]
pub struct TargetArgv {
    pub argv : Vec<String>,
    pub atidx : Option<usize>,          /* @ idx */
        name : *mut String,
}

impl TargetArgv {
    pub fn parse(argv : Vec<String>) -> Self {

        let dashidx = argv.iter().position(|arg| arg == "--").unwrap_or_else(||{
            err!("Can't found `--` in argument line");
            unreachable!()
        });

        let argv = &argv[dashidx+1..];

        let mut atidx : Option<usize> = None;
        let mut new_argv = vec![];

        for (idx, arg) in argv.iter().enumerate() {

            new_argv.push(arg.clone());
            
            if arg.contains("@@") {
                atidx = Some(idx); 
            }
        }

        let ptr: *mut String = &mut new_argv[0];
        TargetArgv {
            name : ptr,
            argv : new_argv, 
            atidx
        }
        
    }
    pub fn name(&self) -> &String {
        assert!(!self.name.is_null());
        unsafe { 
            &*(self.name) 
        }
    }
}