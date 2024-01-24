#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{get_time, yield_};

#[no_mangle]
fn main() -> i32 {
    let mut counter = 0;
    let current_timer = get_time();
    let wait_for = current_timer + 3000;
    while get_time() < wait_for {
        counter += 1;
        if counter % 100000 == 0 {
            println!("sleep [{}/{}]", counter/1000, wait_for - current_timer);
        }
        yield_();
    }
    println!("Test sleep OK!");
    0
}
