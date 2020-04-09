#![feature(lang_items)]
#![feature(const_fn)]
#![feature(allocator_api)]
#![no_std]
#![feature(ptr_internals)]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(alloc_error_handler)]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;
extern crate bit_field;
extern crate linked_list_allocator;
extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;
extern crate x86_64;

#[macro_use]
mod vga_buffer;
mod hardware;
mod interrupts;
mod memory;

#[macro_use]
extern crate once;

pub const HEAP_START: usize = 0o0_000_010_000_000_000;
pub const HEAP_SIZE: usize = 200 * 1024; // 200 KiB

use core::panic::PanicInfo;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    // ATTENTION: we have a very small stack and no guard page
    vga_buffer::clear_screen();
    println!("Hello World!");

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    enable_nxe_bit();
    enable_write_protect_bit();

    // set up guard page and map the heap pages
    let mut memory_controller = memory::init(boot_info);

    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_START, HEAP_START + HEAP_SIZE);
    }

    hardware::pic::init();
    interrupts::init(&mut memory_controller);

    unsafe {
        x86_64::instructions::interrupts::enable();
    }

    loop {
        unsafe {
            x86_64::instructions::halt();
        }
    }
}

#[panic_handler]
#[lang = "eh_personality"]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    if let Some(location) = _info.location() {
        println!(
            "\n\nPANIC in {} at line {}:",
            location.file(),
            location.line()
        );
        println!("General panic :(");
    }
    loop {}
}

use alloc::alloc::Layout;

#[lang = "oom"]
#[alloc_error_handler]
#[no_mangle]
pub fn oom_panic(layout: Layout) -> ! {
    loop {}
}

fn enable_nxe_bit() {
    use x86_64::registers::msr::{rdmsr, wrmsr, IA32_EFER};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}
