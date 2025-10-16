use crate::serial;
use crate::interrupts::types::InterruptFrame;

// consts
pub const TIMER_VECTOR                      : u8 = 0;
pub const KEYBOARD_VECTOR                   : u8 = 1;
pub const DOUBLE_EXCEPTION_VECTOR           : u8 = 8;
pub const TASK_SWITCH_VECTOR                : u8 = 10;
pub const MISSING_SEGMENT_VECTOR            : u8 = 11;
pub const STACK_ERROR_VECTOR                : u8 = 12;
pub const GENERAL_PROTECTION_FAULT_VECTOR   : u8 = 13;
pub const PAGE_FAULT_VECTOR                 : u8 = 14;

static mut ISR_HANDLERS: [Option<fn(&InterruptFrame)>; 256] = [None; 256];
static mut IRQ_HANDLERS: [Option<fn(&InterruptFrame)>; 16] = [None; 16];

pub fn register_isr_handler(vector: u8, handler: fn(&InterruptFrame)) {
    unsafe {
        ISR_HANDLERS[vector as usize] = Some(handler);
    }
}

pub fn register_irq_handler(irq: u8, handler: fn(&InterruptFrame)) {
    unsafe {
        IRQ_HANDLERS[irq as usize] = Some(handler);
    }
}

#[no_mangle]
pub extern "C" fn isr_dispatch(frame: &InterruptFrame) {
    let vector = frame.interrupt_number as usize;
    
    unsafe {
        if let Some(handler) = ISR_HANDLERS[vector] {
            handler(frame);
        } else {
            default_isr_handler(frame);
        }
    }
}

#[no_mangle]
pub extern "C" fn irq_dispatch(frame: &InterruptFrame) {
    let irq_num = (frame.interrupt_number - 32) as usize;

    unsafe {
        // note: timer vector condition is needed for easier branch prediction for processor
        if irq_num != TIMER_VECTOR as usize {
            if let Some(handler) = IRQ_HANDLERS[irq_num] {
                handler(frame);
            } else {
                default_irq_handler(frame);
            }
        } else {
            let handler = IRQ_HANDLERS[TIMER_VECTOR as usize].unwrap_or(default_irq_handler);
            handler(frame);
        }
    }

    unsafe {
        if irq_num >= 8 {
            crate::PIC_SLAVE.send_eoi();
        }

        crate::PIC_MASTER.send_eoi();
    }
}

fn default_isr_handler(frame: &InterruptFrame) {
    serial::print_string("Unhandled ISR");
    serial::print_hex(frame.interrupt_number);
    serial::print_string(" Error: ");
    serial::print_hex(frame.error_code);
    serial::print_string("\n");

    match frame.interrupt_number as u8 {
        DOUBLE_EXCEPTION_VECTOR | TASK_SWITCH_VECTOR | MISSING_SEGMENT_VECTOR | STACK_ERROR_VECTOR | GENERAL_PROTECTION_FAULT_VECTOR | PAGE_FAULT_VECTOR => {
            serial::print_string("Critical exception - halting\n");
            loop { unsafe { core::arch::asm!("hlt"); } }
        }
        _ => {}
    }
}

fn default_irq_handler(frame: &InterruptFrame) {
    let irq_num = frame.interrupt_number - 32;
    serial::print_string("Unhandled IRQ");
    serial::print_hex(irq_num);
    serial::print_string("\n");
}

pub fn timer_handler(frame: &InterruptFrame) {

}
