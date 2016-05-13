#![cfg(test)]

static mut LAST_IRQ_CAUGHT: usize = 256;

/// Just update the *gulp* global mutable variable
unsafe fn log_interrupt(i: usize) {
    LAST_IRQ_CAUGHT = i;
}

#[test]
fn verify_dispatch_0() {

    super::set_handler(0, log_interrupt);
    super::entry(0); // Fake an interrupt

    unsafe{ assert_eq!(LAST_IRQ_CAUGHT, 0); }
}

#[test]
fn verify_dispatch_255() {
    super::set_handler(255, log_interrupt);
    super::entry(255); // Fake an interrupt

    unsafe{ assert_eq!(LAST_IRQ_CAUGHT, 255); }
}
