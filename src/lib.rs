#![feature(lang_items)]
#![feature(const_fn)]
#![no_std]
#![feature(unique)]
#![feature(ptr_internals)]

extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;

#[macro_use]
mod vga_buffers;
mod memory;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    vga_buffers::clear_screen();

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!(
            "    start: 0x{:x}, length: 0x{:x}",
            area.base_addr, area.length
        );
    }

    let elf_sections = boot_info
        .elf_sections_tag()
        .expect("Elf-sections tag required");

    println!("The kernel sections are:");
    for section in elf_sections.sections() {
        println!(
            "addr:0x{:x}, size: 0x{:x}, flags:0x{:x}",
            section.addr, section.size, section.flags
        );
    }

    loop {}
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop {}
}
