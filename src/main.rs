#![no_std]
#![no_main]

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Something went wrong");
    loop {}
}

use core::panic::PanicInfo;

/// この関数はパニック時に呼ばれる
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
