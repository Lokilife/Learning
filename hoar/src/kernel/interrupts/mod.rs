mod types;
mod registry;
mod dispatcher;
mod pic;

pub use registry::*;
pub use pic::*;

use crate::interrupts::types::InterruptFrame;

pub struct InterruptManager;

impl InterruptManager {    
    pub fn register_exception(vector: u8, handler: fn(&InterruptFrame)) {
        dispatcher::register_isr_handler(vector, handler);
    }

    pub fn register_irq(irq: u8, handler: fn(&InterruptFrame)) {
        dispatcher::register_irq_handler(irq, handler);
    }

    pub fn register_timer(handler: fn(&InterruptFrame)) {
        InterruptManager::register_irq(dispatcher::TIMER_VECTOR, handler);
    }

    pub fn register_keyboard(handler: fn(&InterruptFrame)) {
        InterruptManager::register_irq(dispatcher::KEYBOARD_VECTOR, handler);
    }

    // pub fn register_mouse(handler: fn(&InterruptFrame)) {
    //     InterruptManager::register_irq(12, handler);
    // }

    pub fn register_general_protection(handler: fn(&InterruptFrame)) {
        InterruptManager::register_exception(dispatcher::GENERAL_PROTECTION_FAULT_VECTOR, handler);
    }

    pub fn register_page_fault(handler: fn(&InterruptFrame)) {
        InterruptManager::register_exception(dispatcher::PAGE_FAULT_VECTOR, handler);
    }
}
