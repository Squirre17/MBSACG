
pub mod inner{
    use ansi_term::Color;

    pub fn _act(args: std::fmt::Arguments) {
        print!("{}", Color::Blue.bold().paint("[*] "));
        println!("{}", args);
    }

    pub fn _ok(args: std::fmt::Arguments) {
        print!("{}", Color::Green.bold().paint("[+] "));
        println!("{}", args);
    }

    pub fn _err(args: std::fmt::Arguments) {
        print!("{}", Color::Red.bold().paint("[-] "));
        println!("{}", args);
    }
}

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
    ($($arg:tt)*) => (crate::log::inner::_err(format_args!($($arg)*)));
}