#![no_std]
#![no_main]
#![allow(clippy::println_empty_string)]

extern crate alloc;

#[macro_use]
extern crate user_lib;

const LF: u8 = 0x0au8;
const CR: u8 = 0x0du8;
const BS: u8 = 0x08u8;
const DL: u8 = 0x7fu8;

use alloc::string::String;
use user_lib::console::getchar;
use user_lib::{exec, fork, waitpid, shutdown};

const PROMPT: &str = "\x1b[34m|user >\x1b[32m>\x1b[33m> \x1b[0m";

#[no_mangle]
pub fn main() -> i32 {
    println!("\x1b[31mHello this is user shell!\x1b[0m");
    let mut line: String = String::new();
    print!("{}", PROMPT);
    loop {
        let c = getchar();
        match c {
            LF | CR => {
                println!("");
                if !line.is_empty() {
                    line.push('\0');
                    if line.eq_ignore_ascii_case("quit\0") {
                        // built-in command
                        return 0;
                    }
                    if line.eq_ignore_ascii_case("shutdown\0") {
                        // built-in command
                        shutdown(0)
                    }
                    let pid = fork();
                    if pid == 0 {
                        // child process
                        if exec(line.as_str()) == -1 {
                            println!("{}: command not found", line.as_str());
                            return -4;
                        }
                        unreachable!();
                    } else {
                        let mut exit_code: i32 = 0;
                        let exit_pid = waitpid(pid as usize, &mut exit_code);
                        assert_eq!(pid, exit_pid);
                        println!("Shell: Process {} exited with code {}", pid, exit_code);
                    }
                    line.clear();
                }
                print!("{}", PROMPT);
            }
            BS | DL => {
                // deal with backsapce
                if !line.is_empty() {
                    print!("{}", BS as char);
                    print!(" ");
                    print!("{}", BS as char);
                    line.pop();
                }
            }
            _ => {
                print!("{}", c as char);
                line.push(c as char);
            }
        }
    }
}
