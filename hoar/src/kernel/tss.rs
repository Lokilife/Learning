#[no_mangle]
pub static mut TSS: TaskStateSegment = TaskStateSegment::new();

#[repr(C, align(4))]
#[derive(Debug, Clone, Copy)]
pub struct TaskStateSegment {
    pub prev_tss: u32,        // Previous TSS (for hardware switch)
    pub esp0: u32,            // ESP for ring 0
    pub ss0: u32,             // SS for ring 0
    pub esp1: u32,            // ESP for ring 1
    pub ss1: u32,             // SS for ring 1
    pub esp2: u32,            // ESP for ring 2
    pub ss2: u32,             // SS for ring 2
    pub cr3: u32,             // CR3 (pages table)
    pub eip: u32,             // EIP
    pub eflags: u32,          // EFLAGS
    pub eax: u32,             // EAX
    pub ecx: u32,             // ECX
    pub edx: u32,             // EDX
    pub ebx: u32,             // EBX
    pub esp: u32,             // ESP
    pub ebp: u32,             // EBP
    pub esi: u32,             // ESI
    pub edi: u32,             // EDI
    pub es: u32,              // ES
    pub cs: u32,              // CS
    pub ss: u32,              // SS
    pub ds: u32,              // DS
    pub fs: u32,              // FS
    pub gs: u32,              // GS
    pub ldt: u32,             // LDT selector
    pub trap: u16,            // Trap flag
    pub iomap_base: u16,      // I/O bitmap offset
}

impl TaskStateSegment {
    pub const fn new() -> Self {
        Self {
            prev_tss: 0,
            esp0: 0,
            ss0: 0,
            esp1: 0,
            ss1: 0,
            esp2: 0,
            ss2: 0,
            cr3: 0,
            eip: 0,
            eflags: 0,
            eax: 0,
            ecx: 0,
            edx: 0,
            ebx: 0,
            esp: 0,
            ebp: 0,
            esi: 0,
            edi: 0,
            es: 0,
            cs: 0,
            ss: 0,
            ds: 0,
            fs: 0,
            gs: 0,
            ldt: 0,
            trap: 0,
            iomap_base: 0xFFFF,
        }
    }
}
