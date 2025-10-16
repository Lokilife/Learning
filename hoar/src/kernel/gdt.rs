use core::arch::asm;

use crate::serial;
use crate::tss::TaskStateSegment;
use crate::tss::TSS;

// Access bytes
pub const ACCESS_PRESENT: u8 = 0x80;            // P=1 (segment present)
pub const ACCESS_PRIVILEGE_RING0: u8 = 0x00;    // DPL=0 (core)
pub const ACCESS_PRIVILEGE_RING3: u8 = 0x60;    // DPL=3 (user)
pub const ACCESS_CODE_SEGMENT: u8 = 0x18;       // S=1, E=1 (code)
pub const ACCESS_DATA_SEGMENT: u8 = 0x10;       // S=1, E=0 (data)
pub const ACCESS_CODE_READABLE: u8 = 0x02;      // Readable code segment
pub const ACCESS_DATA_WRITABLE: u8 = 0x02;      // Writable data segment
pub const ACCESS_TSS: u8 = 0x09;                // 32-bit TSS

// Flags
pub const FLAGS_32BIT: u8 = 0x40;           // D/B=1 (32-bit segment)
pub const FLAGS_GRANULARITY_4K: u8 = 0x80;  // G=1

// Selectors
pub const KERNEL_CODE_SELECTOR: u16 = 0x08;  // Index 1, RPL=0
pub const KERNEL_DATA_SELECTOR: u16 = 0x10;  // Index 2, RPL=0  
pub const USER_CODE_SELECTOR: u16 = 0x1B;    // Index 3, RPL=3
pub const USER_DATA_SELECTOR: u16 = 0x23;    // Index 4, RPL=3
pub const TSS_SELECTOR: u16 = 0x28;          // Index 5, RPL=0

extern "C" {
    fn load_gdt(gdt_ptr: *mut GdtPointer, selector: u16);
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct GdtDescriptor {
    limit_low: u16,         // Byte 0-1: Low 16 bit limit
    base_low: u16,          // Byte 2-3: Low 16 bit base address  
    base_middle: u8,        // Byte 4: Middle 8 bit base address
    access: u8,             // Byte 5: Access byte
    flags_limit_high: u8,   // Byte 6: Flags + higher 4 bit limit
    base_high: u8,          // Byte 7: Higher 8 bit base address
}

impl GdtDescriptor {
    pub const fn new(base: u32, limit: u32, access: u8, flags: u8) -> Self {
        Self {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access,
            flags_limit_high: ((limit >> 16) as u8 & 0x0F) | (flags & 0xF0),
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Gdt {
    table: [GdtDescriptor; 7],
}

impl Gdt {
    pub const fn dummy() -> Self {
        Self {
            table: [GdtDescriptor::new(0, 0, 0, 0); 7],
        }
    }

    pub fn new() -> Self {
        Self {
            table: [
                GdtDescriptor::new(0, 0, 0, 0), // 0: Null descriptor
                GdtDescriptor::new(0, 0xFFFFF, 
                    ACCESS_PRESENT | ACCESS_PRIVILEGE_RING0 | ACCESS_CODE_SEGMENT | ACCESS_CODE_READABLE,
                    FLAGS_GRANULARITY_4K | FLAGS_32BIT), // 1: Kernel code
                GdtDescriptor::new(0, 0xFFFFF,
                    ACCESS_PRESENT | ACCESS_PRIVILEGE_RING0 | ACCESS_DATA_SEGMENT | ACCESS_DATA_WRITABLE,
                    FLAGS_GRANULARITY_4K | FLAGS_32BIT), // 2: Kernel data
                GdtDescriptor::new(0, 0xFFFFF,
                    ACCESS_PRESENT | ACCESS_PRIVILEGE_RING3 | ACCESS_CODE_SEGMENT | ACCESS_CODE_READABLE,
                    FLAGS_GRANULARITY_4K | FLAGS_32BIT), // 3: User code
                GdtDescriptor::new(0, 0xFFFFF,
                    ACCESS_PRESENT | ACCESS_PRIVILEGE_RING3 | ACCESS_DATA_SEGMENT | ACCESS_DATA_WRITABLE,
                    FLAGS_GRANULARITY_4K | FLAGS_32BIT), // 4: User data
                GdtDescriptor::new(0, 0, 0, 0), // 5: TSS (will be filled later)
                GdtDescriptor::new(0, 0, 0, 0), // 6: Reserved
            ],
        }
    }

    pub fn load(&'static self) {
        let mut pointer = GdtPointer {
            limit: (core::mem::size_of::<[GdtDescriptor; 7]>() - 1) as u16,
            base: &self.table as *const _ as u32,
        };

        unsafe {
            load_gdt(&mut pointer, KERNEL_CODE_SELECTOR);
        }
    }

    pub fn set_tss_descriptor(&mut self, tss: &TaskStateSegment) {
        let tss_base = tss as *const _ as u32;
        let tss_limit = (core::mem::size_of::<TaskStateSegment>() - 1) as u32;

        self.table[5] = GdtDescriptor::new(
            tss_base,
            tss_limit,
            ACCESS_PRESENT | ACCESS_PRIVILEGE_RING0 | ACCESS_TSS,
            0x00,
        );
    }

    pub fn get_tss_selector() -> u16 {
        TSS_SELECTOR
    }
}

// Structure for LGDT
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct GdtPointer {
    limit: u16,
    base: u32,
}

#[no_mangle]
pub static mut GDT: Gdt = Gdt::dummy();

#[no_mangle]
pub fn init_gdt() {
    unsafe {
        GDT = Gdt::new();

        TSS.ss0 = 0x10;
        TSS.esp0 = 0x9000;
        TSS.iomap_base = 0xFFFF;

        GDT.set_tss_descriptor(&TSS);

        serial::print_string("Loading GDT...\n");

        GDT.load();
        serial::print_string("GDT loaded\n");

        serial::print_string("TSS base: ");
        serial::print_hex(&TSS as *const _ as u32);
        serial::print_string(", selector: ");
        serial::print_hex(Gdt::get_tss_selector() as u32);
        serial::print_string("\n");

        asm!("ltr ax", in("ax") Gdt::get_tss_selector());
        
        serial::print_string("TSS loaded successfully\n");
    }
}
