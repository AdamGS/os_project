#![feature(lang_items)]
#![feature(const_fn)]
#![feature(allocator_api)]
#![feature(alloc)]
#![no_std]
#![no_main]
#![feature(ptr_internals)]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(panic_handler)]
#![feature(alloc_error_handler)]

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

pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 200 * 1024; // 200 KiB

use core::panic::PanicInfo;
use linked_list_allocator::LockedHeap;
use hardware::pic;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _rust_start() -> ! {

    unsafe {
        //asm!("bits 64");
        // asm!(r"mov ax, 0
        //        mov ss, ax
        //        mov ds, ax
        //        mov es, ax
        //        mov fs, ax
        //        mov gs, ax"::::"volatile", "intel");
    }
    unsafe {
        asm!("mov %ebx, %edi");
        asm!("cmp %edi, 0x36d76289");
    }

    //vga_buffer::clear_screen();

    unsafe {
        asm!("mov eax, 0x2f592f412f4b2f4f"::::"volatile", "intel");
        asm!("mov dword ptr [0xb8000], eax"::::"volatile", "intel");
        //asm!("hlt");
    }

    println!("HELLO WORLD");


    let vga_buffer = 0xb8000 as *mut u8;

    //print `HELLO` to the screen (see
    //https://os.phil-opp.com/minimal-rust-kernel/#printing-to-screen)
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    //println!("Hello i'm in start!");
    loop {}
}

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
    interrupts::init(&mut memory_controller);

    let pic_board = pic::init();

    unsafe {
        HEAP_ALLOCATOR
            .lock()
            .init(HEAP_START, HEAP_START + HEAP_SIZE);
    }

    unsafe {
        x86_64::instructions::interrupts::enable();
    }

    println!("interrupts now enabled");

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
