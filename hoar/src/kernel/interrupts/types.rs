#[repr(C)]
pub struct InterruptFrame {
    pub edi: u32,
    pub esi: u32,
    pub ebp: u32,
    pub esp: u32,
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32,
    pub eax: u32,

    pub ds: u32,
    pub es: u32,
    pub fs: u32,
    pub gs: u32,

    pub interrupt_number: u32,
    pub error_code: u32,

    pub eip: u32,
    pub cs: u32,
    pub eflags: u32,
    pub user_esp: u32,
    pub ss: u32,
}
