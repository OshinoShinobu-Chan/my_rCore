use core::{panic::PanicInfo};

use crate::exit;

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        error!(
            "", 
            "core panic at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        error!("", "core panic: {}", info.message().unwrap());
    }
    exit(-1);
    loop { }
}
