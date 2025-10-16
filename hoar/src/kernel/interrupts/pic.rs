use crate::io;

pub struct Pic {
    pub command: u16,
    pub data: u16,
}

impl Pic {
    const ICW1_INIT: u8 = 0x11;
    const ICW1_ICW4: u8 = 0x01;
    const ICW4_8086: u8 = 0x01;
    const EOI: u8 = 0x20;

    pub const fn new(port: u16) -> Self {
        Self {
            command: port,
            data: port + 1,
        }
    }

    pub unsafe fn remap(&self, offset: u8) {
        let mask = io::inb(self.data);

        io::outb(self.command, Self::ICW1_INIT | Self::ICW1_ICW4);
        io::outb(self.data, offset);
        io::outb(self.data, 0x04);  // Master has slave at IRQ2
        io::outb(self.data, Self::ICW4_8086);

        io::outb(self.data, mask);
    }

    pub unsafe fn enable_irq(&self, irq: u8) {
        let port = self.data;
        let mask = io::inb(port) & !(1 << (irq & 7));
        io::outb(port, mask);
    }

    /// EOI - End Of Interrupt
    pub fn send_eoi(&self) {
        unsafe {
            io::outb(self.command, Self::EOI);
        }
    }
}
