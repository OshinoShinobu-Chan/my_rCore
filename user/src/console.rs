use super::{write, read};
use core::fmt::{self, Write};

struct Stdout;

const STDOUT: usize = 1;
const STDIN: usize = 0;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write(STDOUT, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($str: literal $(, $($tail:tt)+)?) => {
        $crate::console::print(format_args!($str $(, $($tail)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($str: literal $(, $($tail:tt)+)?) => {
        $crate::console::print(format_args!(concat!($str, "\n") $(, $($tail)+)?));
    }
}

pub fn getchar() -> u8 {
    let mut c = [0u8; 1];
    read(STDIN, &mut c);
    c[0]
}