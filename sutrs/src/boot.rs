#![no_main]
#![no_std]

use core::arch::asm;

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(export_name = "efi_main")]
pub extern "C" fn main(_h: *mut core::ffi::c_void, _st: *mut core::ffi::c_void) -> usize {
    loop {
        unsafe {
            asm!("hlt");
        }
    }

    0
}
