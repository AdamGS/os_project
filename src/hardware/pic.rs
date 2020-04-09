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
    pub primary_pic: Pic,
    pub secondary_pic: Pic,
}

impl PicBoard {
    fn new() -> PicBoard {
        PicBoard {
            primary_pic: Pic::new(0x20),
            secondary_pic: Pic::new(0xA0),
        }
    }
}
pub fn init() -> PicBoard {
    let board = PicBoard::new();
    let wait_port: Port<u8> = Port::new(0x80);
    let wait = || unsafe {
        asm!("nop\nnop\nnop");
        wait_port.write(0);
    };

    let a = board.primary_pic.data_port.read();
    let b = board.secondary_pic.data_port.read();

    // Start init
    board.primary_pic.command_port.write(0x11);
    wait();
    board.secondary_pic.command_port.write(0x11);
    wait();

    // Set offsets
    board.primary_pic.data_port.write(0x20);
    wait();
    board.secondary_pic.data_port.write(0x28);
    wait();

    board.primary_pic.data_port.write(0x0);
    board.secondary_pic.data_port.write(0x0);

    board.primary_pic.data_port.write(0x4);
    wait();
    board.secondary_pic.data_port.write(0x2);
    wait();

    board.primary_pic.data_port.write(0x1);
    wait();
    board.secondary_pic.data_port.write(0x1);
    wait();

    board.primary_pic.command_port.write(0x0);
    board.secondary_pic.command_port.write(0x0);

    board.primary_pic.command_port.write(0x20);
    board.secondary_pic.command_port.write(0x20);

    wait();

    board
}
