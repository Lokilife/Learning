use core::arch::asm;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct IdtEntry {
    offset_low: u16,    // low 16 bit handler offset
    selector: u16,      // code segment selector
    zero: u8,           // zero
    type_attr: u8,      // type and attributes
    offset_high: u16,   // higher 16 bit offset
}

impl IdtEntry {
    pub const fn new(handler: u32, selector: u16, type_attr: u8) -> Self {
        Self {
            offset_low: (handler & 0xFFFF) as u16,
            selector,
            zero: 0,
            type_attr,
            offset_high: ((handler >> 16) & 0xFFFF) as u16,
        }
    }
}

// consts for type_attr
pub const IDT_PRESENT: u8 = 0x80;
pub const IDT_INTERRUPT_GATE: u8 = 0x0E;
pub const IDT_TRAP_GATE: u8 = 0x0F;
pub const IDT_RING0: u8 = 0x00;
pub const IDT_RING3: u8 = 0x60;

#[repr(C, packed)]
pub struct IdtPointer {
    limit: u16,
    base: u32,
}

#[repr(C, packed)]
pub struct Idt {
    entries: [IdtEntry; 256],
}

impl Idt {
    pub fn new() -> Self {
        Self {
            entries: [IdtEntry::new(0, 0, 0); 256]
        }
    }

    pub fn set_entry(&mut self, index: usize, handler: u32, selector: u16, type_attr: u8) {
        self.entries[index] = IdtEntry::new(handler, selector, type_attr);
    }

    pub fn load(&self) {
        let mut pointer = IdtPointer {
            limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
            base: &self.entries as *const _ as u32
        };

        unsafe {
            asm!("lidt [{}]", in(reg) &mut pointer);
        }
    }
}
