#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

use bootloader::{entry_point, BootInfo};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory;
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("Hello World{}", "!");
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { rust_os::memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    #[cfg(test)]
    test_main();
    rust_os::hlt_loop();
}

/// この関数はパニック時に呼ばれる。
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}
