//! IRQ dispatch module. This is an internal storage module for the
//! system-internal Rust re-implementation of interrupt dispatch.
//!
//! # Safety
//! Are you kidding me?


/// This is a do-nothing default handler for interrupts. It just echoes
/// the interrupt number to screen.
unsafe fn null_handler(vec: usize) {
    println!("Handled interrupt {}!", vec);
}

/// This is a static vector of dispatcher functions. Basically, it's a
/// re-implementation of the IDT on a higher level.
static mut VEC_DISPATCH_FNS: [unsafe fn(usize); super::idt::IDT_NUM_ENTRIES]
    = [null_handler; super::idt::IDT_NUM_ENTRIES];

/// This is the entry point for the dispatcher. It is supposed to be
/// called from whatever lower-level code catches the given interrupt.
pub fn entry(vec: usize) {

    // Call the corresponding handler
    unsafe {VEC_DISPATCH_FNS[vec](vec);}
}

/// Register `f` as the handler for interrupt `vec`
pub fn set_handler(vec: usize,
                   f: unsafe fn(usize) -> ()) {
    unsafe{VEC_DISPATCH_FNS[vec] = f;}

}
