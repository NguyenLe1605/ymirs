#![no_std]
#![no_main]

use core::arch::asm;

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(export_name = "_start")]
pub fn kernel_entry() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
