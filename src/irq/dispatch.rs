//! IRQ dispatch module. This is an internal storage module for the
//! system-internal Rust re-implementation of interrupt dispatch.


/// This is a do-nothing default handler for interrupts. It just echoes
/// the interrupt number to screen.
fn null_handler(vec: usize) {
    println!("Handled interrupt {}!", vec);
}

/// This is a static vector of dispatcher functions. Basically, it's a
/// re-implementation of the IDT on a higher level.
static mut VEC_DISPATCH_FNS: [fn(usize); super::idt::IDT_NUM_ENTRIES]
    = [null_handler; super::idt::IDT_NUM_ENTRIES];

pub fn entry(vec: usize) {

    // Call the corresponding handler
    unsafe {VEC_DISPATCH_FNS[vec](vec);}
}
