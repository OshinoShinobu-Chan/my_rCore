#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::yield_;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;

#[no_mangle]
fn main() -> i32 {
    unsafe {
        let ptr = 0x80000000 as *mut u8;
        println!("{}", ptr.read_volatile() as char);
    }
    for i in 0..HEIGHT {
        for _ in 0..WIDTH {
            print!("C");
        }
        println!(" [{}/{}]", i + 1, HEIGHT);
        yield_();
    }
    println!("Test write_c OK!");
    // a delibrate illegal instruction
    0
}
