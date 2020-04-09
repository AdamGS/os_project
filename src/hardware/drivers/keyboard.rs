use hardware::drivers::{Driver, DriverInterrupt};
use hardware::Port;
use x86_64::structures::idt::ExceptionStackFrame;

pub fn driver() -> Driver {
    let press_key = DriverInterrupt {
        idx: 33,
        handler: keyboard_handler,
    };
    Driver::new([press_key].to_vec(), "Keyboard".into())
}

extern "x86-interrupt" fn keyboard_handler(stack_frame: &mut ExceptionStackFrame) {
    let keyboard = Keyboard::new();
    let keycode = keyboard.read();
}

struct Keyboard {
    control_port: Port<u8>,
    data_port: Port<u8>,
}

impl Keyboard {
    fn new() -> Keyboard {
        Keyboard {
            control_port: Port::new(0x64),
            data_port: Port::new(0x60),
        }
    }

    fn ack(&self) {
        let ack_port = Port::new(0x20);
        ack_port.write(0x20 as u8);
    }

    fn read(&self) -> Option<u8> {
        let status = self.control_port.read();
        let keycode = self.data_port.read();

        self.ack();

        if status.trailing_zeros() == 0 {
            Some(keycode)
        } else {
            None
        }
    }
}
