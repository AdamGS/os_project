use super::Port;

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
}
