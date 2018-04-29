#![feature(lang_items)]
#![feature(const_fn)]
#![no_std]
#![feature(unique)]
#![feature(ptr_internals)]

#[macro_use]
extern crate bitflags;
extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;
extern crate x86_64;

#[macro_use]
mod vga_buffers;
mod memory;

use memory::FrameAllocator;

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

    let kernel_start = elf_sections.sections().map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections
        .sections()
        .map(|s| s.addr + s.size)
        .max()
        .unwrap();

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("The kernel sections are:");
    for section in elf_sections.sections() {
        println!(
            "addr:0x{:x}, size: 0x{:x}, flags:0x{:x}",
            section.addr, section.size, section.flags
        );
    }

    println!(
        "kernel_start: 0x{:x}, kernel_end: 0x{:x}",
        kernel_start, kernel_end
    );
    println!(
        "multiboot_start: 0x{:x}, multiboot_end: 0x{:x}",
        multiboot_start, multiboot_end
    );

    let mut allocator = memory::AreaFrameAllocator::new(
        kernel_start as usize,
        kernel_end as usize,
        multiboot_start as usize,
        multiboot_end as usize,
        memory_map_tag.memory_areas(),
    );

    for i in 0.. {
        if let None = allocator.allocate_frame() {
            println!("allocated {} frames", i);
            break;
        }
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
