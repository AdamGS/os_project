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

    pub fn ack(&self) {
        self.control_port.read();
    }

    pub fn read(&self) -> Option<u8> {
        let status = self.control_port.read();

        let keycode = self.data_port.read();

        if (status.trailing_zeros() == 0 && keycode < 52) {
            let keycode_map = "  1234567890-= \tqwertyuiop[]\n asdfghjkl;\'` \\zxcvbnm,./ *                                                           ";
            let letter = keycode_map.as_bytes()[keycode as usize];
            Port::new(0x020).write(0x20 as u8);
            return Some(letter);
        }

        Port::new(0x020).write(0x20 as u8);
        None
    }
}
