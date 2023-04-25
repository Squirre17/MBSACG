
pub mod inner{
    use ansi_term::Color;

    #[inline]
    pub fn _act(args: std::fmt::Arguments) {
        print!("{}", Color::Blue.bold().paint("[*] "));
        println!("{}", args);
    }

    #[inline]
    pub fn _ok(args: std::fmt::Arguments) {
        print!("{}", Color::Green.bold().paint("[+] "));
        println!("{}", args);
    }

    #[inline]
    pub fn _err(args: std::fmt::Arguments) {
        eprint!("{}", Color::Red.bold().paint("[!] "));
        eprintln!("{}", args);
        // panic!()
    }

    #[inline]
    pub fn _warn(args: std::fmt::Arguments) {
        print!("{}", Color::Yellow.bold().paint("[-] "));
        println!("{}", args);
    }
}
/* macro_export annotation will export it to crate root namespace */
#[macro_export]
macro_rules! act {
    ($($arg:tt)*) => (crate::log::inner::_act(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! ok {
    ($($arg:tt)*) => (crate::log::inner::_ok(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => ({
        crate::log::inner::_err(format_args!($($arg)*));
        panic!();
    });
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => (crate::log::inner::_warn(format_args!($($arg)*)));
}