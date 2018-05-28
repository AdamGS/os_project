mod keyboard;

use core::marker::PhantomData;

trait InOut {
    unsafe fn port_in(port: u16) -> Self;
    unsafe fn port_out(port: u16, value: Self);
}

impl InOut for u8 {
    unsafe fn port_in(port: u16) -> u8 {
        inb(port)
    }

    unsafe fn port_out(port: u16, value: u8) {
        outb(port, value);
    }
}
impl InOut for u16 {
    unsafe fn port_in(port: u16) -> u16 {
        inw(port)
    }

    unsafe fn port_out(port: u16, value: u16) {
        outw(port, value);
    }
}
impl InOut for u32 {
    unsafe fn port_in(port: u16) -> u32 {
        inl(port)
    }

    unsafe fn port_out(port: u16, value: u32) {
        outl(port, value);
    }
}

pub struct Port<T> {
    port: u16,
    phantom_data: PhantomData<T>,
}

impl<T: InOut> Port<T> {
    pub fn new(port: u16) -> Port<T> {
        Port {
            port: port,
            phantom_data: PhantomData,
        }
    }
}

unsafe fn inb(port: u16) -> u8 {
    let mut value: u8;
    asm!("inb %al, %dx" : "={al}"(value) :
         "{dx}"(port) ::
         "volatile");

    value
}

unsafe fn outb(port: u16, value: u8) {
    asm!("outb %al, %dx" ::
         "{dx}"(port), "{al}"(value) ::
         "volatile");
}

unsafe fn inw(port: u16) -> u16 {
    let mut value: u16;
    asm!("inw %al, %dx" : "={al}"(value) :
         "{dx}"(port) ::
         "volatile");

    value
}

unsafe fn outw(port: u16, value: u16) {
    asm!("outw %al, %dx" ::
         "{dx}"(port), "{al}"(value) ::
         "volatile");
}

unsafe fn inl(port: u16) -> u32 {
    let mut value: u32;
    asm!("inl %al, %dx" : "={al}"(value) :
         "{dx}"(port) ::
         "volatile");

    value
}

unsafe fn outl(port: u16, value: u32) {
    asm!("outl %al, %dx" ::
         "{dx}"(port), "{al}"(value) ::
         "volatile");
}
