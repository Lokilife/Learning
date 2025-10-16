use core::sync::atomic::{AtomicPtr, Ordering};

use crate::interrupts::types::InterruptFrame;

pub type InterruptHandler = fn(&InterruptFrame);

static INTERRUPT_HANDLERS: [AtomicPtr<InterruptHandler>; 256] = {
    const INIT: AtomicPtr<InterruptHandler> = AtomicPtr::new(core::ptr::null_mut());
    [INIT; 256]
};

pub fn register_handler(vector: u8, handler: InterruptHandler) -> Result<(), &'static str> {
    let handler_ptr = handler as *const InterruptHandler as *mut InterruptHandler;
    
    let previous = INTERRUPT_HANDLERS[vector as usize]
        .compare_exchange(
            core::ptr::null_mut(),
            handler_ptr,
            Ordering::Acquire,
            Ordering::Relaxed
        );
    
    match previous {
        Ok(_) => Ok(()),
        Err(_) => Err("Handler already registered for this vector")
    }
}

pub fn get_handler(vector: u8) -> Option<&'static InterruptHandler> {
    let ptr = INTERRUPT_HANDLERS[vector as usize].load(Ordering::Relaxed);
    if ptr.is_null() {
        None
    } else {
        unsafe { Some(&*ptr) }
    }
}
