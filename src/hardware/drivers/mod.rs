use alloc::string::String;
use alloc::vec::Vec;
use core::marker::PhantomData;
use x86_64::structures::idt::{HandlerFunc, HandlerFuncWithErrCode, Idt, PageFaultHandlerFunc};

pub mod keyboard;

pub struct DriverManager {
    pub drivers: Vec<Driver>,
}

impl DriverManager {
    pub fn new() -> DriverManager {
        let keyboard_driver = keyboard::driver();
        DriverManager {
            drivers: [keyboard_driver].to_vec(),
        }
    }
}

#[derive(Clone)]
pub struct Driver {
    pub interrupts: Vec<DriverInterrupt>,
    pub name: String,
}

impl Driver {
    pub fn new(interrupts: Vec<DriverInterrupt>, name: String) -> Driver {
        Driver { interrupts, name }
    }
}

#[derive(Clone, Copy)]
pub struct DriverInterrupt {
    pub idx: usize,
    pub handler: HandlerFunc,
}
