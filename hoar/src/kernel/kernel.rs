#![no_std]
#![no_main]
#![feature(lang_items)]

mod mem;

use core::arch::asm;
use core::panic::PanicInfo;

mod io;
mod serial;
mod vga;

mod gdt;
mod idt;
mod tss;
mod interrupts;

static mut IDT: Option<idt::Idt> = None;
static mut PIC_MASTER: interrupts::Pic = interrupts::Pic::new(0x20);
static mut PIC_SLAVE: interrupts::Pic = interrupts::Pic::new(0xA0);

static VGA_WIDTH: usize = 80;
static VGA_HEIGHT: usize = 25;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    serial::init_serial();

    let vga = vga::initialize_vga(VGA_WIDTH, VGA_HEIGHT);

    vga.clear();
    vga.print_string("Hello, Hoar!");

    gdt::init_gdt();
    serial::print_string("GDB initiated!\n");

    // unsafe {
    //     let mut idt = idt::Idt::new();

    //     for i in 0..32 {
    //         let handler_addr = interrupts::get_isr_address(i);
    //         idt.set_entry(
    //             i,
    //             handler_addr,
    //             gdt::KERNEL_CODE_SELECTOR,
    //             idt::IDT_PRESENT | idt::IDT_RING0 | idt::IDT_INTERRUPT_GATE,
    //         );
    //     }

    //     for i in 0..16 {
    //         let handler_addr = interrupts::get_irq_address(i);
    //         idt.set_entry(
    //             32 + i,
    //             handler_addr,
    //             gdt::KERNEL_CODE_SELECTOR,
    //             idt::IDT_PRESENT | idt::IDT_RING0 | idt::IDT_INTERRUPT_GATE,
    //         );
    //     }

    //     idt.load();
    //     IDT = Some(idt);
    //     serial::print_string("IDT initiated!\n");
    // }

    // unsafe {
    //     PIC_MASTER.remap(0x20);
    //     PIC_SLAVE.remap(0x28);

    //     PIC_MASTER.enable_irq(0);
    //     PIC_MASTER.enable_irq(1);
    // }
    // serial::print_string("PIC initiated!\n");

    unsafe { asm!("sti"); }

    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let msg = _info.message().as_str().unwrap_or("");
    let vga = vga::get_vga();
    vga.clear();
    vga.print_string("\nPanic!\n");
    vga.print_string(msg);

    loop {
        core::hint::spin_loop();
    }
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
