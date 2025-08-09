#![no_main]
#![no_std]

use core::arch::asm;

use log::{debug, error, info, warn};
use uefi::{Status, cstr16, entry, proto::console::text::Output};

#[entry]
fn main() -> Status {
    uefi::helpers::init().expect("can not init uefi");

    let st = match uefi::table::system_table_raw() {
        Some(st) => st,
        None => return Status::ABORTED,
    };

    let st = unsafe { st.as_ref() };
    let stdout: *mut Output = st.stdout.cast();
    let stdout = unsafe { &mut *stdout };

    if let Err(err) = stdout.clear() {
        return err.status();
    }

    if let Err(err) = stdout.output_string(cstr16!("Hello world\n")) {
        return err.status();
    }

    info!("Initialized bootloader log.");
    warn!("Warning on logging.");
    error!("Error on logging.");
    debug!("Debuggin on logging.");

    loop {
        unsafe {
            asm!("hlt");
        }
    }

    Status::SUCCESS
}
