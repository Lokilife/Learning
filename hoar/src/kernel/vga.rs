const VGA_BUFFER: *mut u16 = 0xb8000 as *mut u16;

pub struct VGA {
    cursor: usize,
    width: usize,
    height: usize,
    buffer: &'static mut [u16]
}

impl VGA {
    pub fn new(width: usize, height: usize) -> Self {
        unsafe {
            let buffer = VGA::get_buffer(width, height);

            Self {
                cursor: 0,
                width,
                height,
                buffer,
            }
        }
    }

    unsafe fn get_buffer(width: usize, height: usize) -> &'static mut [u16] {
        core::slice::from_raw_parts_mut(VGA_BUFFER, width * height)
    }

    pub fn print_string(&mut self, string: &str) {
        for ch in string.chars() {
            self.print_char(ch as u8);
        }
    }

    pub fn clear(&mut self) {
        self.cursor = 0;

        for i in 0..self.width * self.height {
            self.buffer[i] = (0x07 << 8) | (b' ' as u16);
        }
    }

    pub fn get_cursor(&self) -> usize {
        self.cursor
    }

    pub fn set_cursor(&mut self, pos: usize) {
        self.cursor = pos;
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn print_char(&mut self, ch: u8) {
        if ch == b'\n' {
            let offset = self.width - (self.cursor % self.width);
            if self.cursor + offset > self.width * self.height {
                // TODO: error handling
                self.cursor = 0;
            }
            else {
                self.cursor += offset;
            }
        }

        self.buffer[self.cursor] = (0x07 << 8) | (ch as u16);
        self.cursor += 1;
    }
}

static mut VGA: Option<VGA> = None;

pub fn get_vga() -> &'static mut VGA {
    unsafe {
        // note: VGA should be initialized right at the kernel start, so it should be safe
        VGA.as_mut().unwrap()
    }
}

pub fn initialize_vga(width: usize, height: usize) -> &'static mut VGA {
    unsafe {
        VGA = Some(VGA::new(width, height));
        VGA.as_mut().unwrap()
    }
}
