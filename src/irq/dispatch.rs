//! IRQ dispatch module. This is an internal storage module for the
//! system-internal Rust re-implementation of interrupt dispatch.

#[no_mangle]
pub extern fn rust_interrupt_handler(intnr: usize) {

    println!("Handled interrupt {}!", intnr);
}

#[no_mangle]
pub extern fn rust_exception_handler() {
    println!("Handled exception!");
}
