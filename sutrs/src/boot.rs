#![no_main]
#![no_std]

use core::{arch::asm, ptr::NonNull};

use elf::{
    endian::AnyEndian,
    file::{Class, Elf64_Ehdr, FileHeader},
};
use log::{debug, info};
use uefi::{
    Status,
    boot::MemoryType,
    cstr16, entry,
    proto::{
        console::text::Output,
        media::file::{File, FileAttribute, FileMode},
    },
};

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

    let handle = uefi::boot::image_handle();
    let mut fs = uefi::boot::get_image_file_system(handle)
        .expect("Failed retrieve image file system protocol");

    info!("Located file system protocol");

    let mut root_dir = fs.open_volume().expect("Failed to open volume");
    info!("Open file system volume");

    let file = root_dir
        .open(cstr16!("ymirs.elf"), FileMode::Read, FileAttribute::empty())
        .expect("Fail to open kernel elf file");
    let mut file = file.into_regular_file().unwrap();

    info!("Open kernel file");

    let header_size = core::mem::size_of::<Elf64_Ehdr>();
    let header_ptr = uefi::boot::allocate_pool(MemoryType::LOADER_DATA, header_size)
        .expect("Failed to allocate memory for kernel ELF header");

    // TODO: must convert back to non null to free it.
    let header_ptr = header_ptr.as_ptr();
    let header_buffer = unsafe { core::slice::from_raw_parts_mut(header_ptr, header_size) };

    file.read(header_buffer)
        .expect("Failed to read kernel ELF header");
    let ident = elf::file::parse_ident::<AnyEndian>(&header_buffer[0..elf::abi::EI_NIDENT])
        .expect("Failed to parse ELF ident");
    let tail_start = elf::abi::EI_NIDENT;
    let tail_end = tail_start + elf::file::ELF64_EHDR_TAILSIZE;
    let ehdr = FileHeader::parse_tail(ident, &header_buffer[tail_start..tail_end])
        .expect("Failed to parse kernel ELF header");

    info!("Parsed ELF kernel header");
    debug!(
        r#"
        Kernel ELF information:
            Entry Point         : 0x{:X}
            Is 64-bit           : {}
            # of Program Headers: {}
            # of Section Headers: {}"#,
        ehdr.e_entry,
        ehdr.class == Class::ELF64,
        ehdr.e_phnum,
        ehdr.e_shnum,
    );

    let free_ptr = header_buffer.as_mut_ptr();
    let free_ptr = unsafe { NonNull::new_unchecked(free_ptr) };
    unsafe {
        uefi::boot::free_pool(free_ptr).expect("Failed to free memory");
    };

    loop {
        unsafe {
            asm!("hlt");
        }
    }

    Status::SUCCESS
}
