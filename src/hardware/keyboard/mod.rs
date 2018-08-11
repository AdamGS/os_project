use super::Port;
use core::ops::BitAnd;

pub struct Keyboard {
    control_port: Port<u8>,
    data_port: Port<u8>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            control_port: Port::new(0x64),
            data_port: Port::new(0x60),
        }
    }

    pub fn ack(&self) {
        let ack_port = Port::new(0x20);
        ack_port.write(0x20 as u8);
    }

    pub fn read(&self) -> Option<u8> {
        let status = self.control_port.read();
        let keycode = self.data_port.read();

        if status.trailing_zeros() == 0 && keycode < 52 {
            self.ack();
            return Some(keycode);
        }

        self.ack();
        None
    }
}
