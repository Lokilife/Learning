use crate::io;

const COM1_PORT: u16 = 0x3F8;

pub fn init_serial() {
    unsafe {
        io::outb(COM1_PORT + 3, 0x80);  // Enable DLAB
        io::outb(COM1_PORT, 0x01);  // Set divisor low byte
        io::outb(COM1_PORT + 1, 0x00);  // Set divisor high byte

        // Configure line control (8 bits, no parity, 1 stop bit)
        io::outb(COM1_PORT + 3, 0x03);

        // Enable FIFO
        io::outb(COM1_PORT + 2, 0xC7);

        // Enable interrupts (optional)
        io::outb(COM1_PORT + 4, 0x0B);
    }
}

fn is_transmit_empty() -> bool {
    unsafe {
        io::inb(COM1_PORT + 5) & 0x20 != 0
    }
}

pub fn print_char(ch: u8) {
    while !is_transmit_empty() {}

    unsafe {
        if ch == b'\n' {
            io::outb(COM1_PORT, b'\r');
            while !is_transmit_empty() {}
        }

        io::outb(COM1_PORT, ch);
    }
}

pub fn print_string(str: &str) {
    for ch in str.chars() {
        print_char(ch as u8);
    }
}

pub fn print_hex(num: u32) {
    let hex_digits = b"0123456789ABCDEF";

    print_char(b'0');
    print_char(b'x');

    for i in 0..8 {
        let shift_amount: u32 = 28 - (i * 4);
        let nibble = (num >> shift_amount) & 0xF;
        let hex_char = hex_digits[nibble as usize];
        print_char(hex_char);
    }
}

pub fn print_hex_byte(byte: u8) {
    let hex_digits = b"0123456789ABCDEF";

    print_char(b'0');
    print_char(b'x');

    let high_nibble = (byte >> 4) & 0xF;
    let low_nibble = byte & 0xF;

    print_char(hex_digits[high_nibble as usize]);
    print_char(hex_digits[low_nibble as usize]);
}
