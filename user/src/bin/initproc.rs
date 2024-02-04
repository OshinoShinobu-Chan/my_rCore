#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{exec, fork, wait, yield_};

#[no_mangle]
fn main() -> i32 {
    let mut shell_pid = fork();
    if shell_pid == 0 {
        exec("user_shell\0");
    } else {
        loop {
            let mut exit_code: i32 = 0;
            let pid = wait(&mut exit_code);
            if pid == -1 {
                yield_();
                continue;
            } else if pid == shell_pid {
                println!("[initproc] User shell exited with code {}", exit_code);
                println!("[initproc] Start another user shell...");
                shell_pid = fork();
                if shell_pid == 0 {
                    exec("user_shell\0");
                }
            } else {
                println!("[initproc] Release a zombie process, pid={}, exit_code={}",
                    pid, exit_code);
            }
        }
    }
    0
}
