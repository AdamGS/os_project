use hardware::InOut;
use hardware::Port;

// Represents a port on the x86's IO bus
pub struct Pic {
    pub command_port: Port<u8>,
    pub data_port: Port<u8>,
}

impl Pic {
    fn new(base_port: u16) -> Pic {
        Pic {
            command_port: Port::new(base_port),
            data_port: Port::new(base_port + 1),
        }
    }
}

pub struct PicBoard {
    pub master_pic: Pic,
    pub slave_pic: Pic,
}

impl PicBoard {
    fn new() -> PicBoard {
        PicBoard {
            master_pic: Pic::new(0x20),
            slave_pic: Pic::new(0xA0),
        }
    }
}
pub fn init() -> PicBoard {
    let mut board = PicBoard::new();
    let mut wait_port: Port<u8> = Port::new(0x80);
    let mut wait = || unsafe {
        asm!("nop\nnop\nnop");
        wait_port.write(0);
    };

    let a = board.master_pic.data_port.read();
    let b = board.slave_pic.data_port.read();

    // Start init
    board.master_pic.command_port.write(0x11);
    wait();
    board.slave_pic.command_port.write(0x11);
    wait();

    // Set offsets
    board.master_pic.data_port.write(0x20);
    wait();
    board.slave_pic.data_port.write(0x28);
    wait();

    board.master_pic.data_port.write(0x0);
    board.slave_pic.data_port.write(0x0);

    board.master_pic.data_port.write(0x4);
    wait();
    board.slave_pic.data_port.write(0x2);
    wait();

    board.master_pic.data_port.write(0x1);
    wait();
    board.slave_pic.data_port.write(0x1);
    wait();

    board.master_pic.command_port.write(0x0);
    board.slave_pic.command_port.write(0x0);

    board.master_pic.command_port.write(0x20);
    board.slave_pic.command_port.write(0x20);

    wait();

    board
}
