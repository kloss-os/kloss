//! This is a module to handle and set up timers. It also provides a
//! very weak callback infrastructure to enable timer hooks.

/// The interrupt vector at which IRQ0 arrives from the PIT.
const IRQ0_VEC : usize = 32;

/// The tolerance for number of sleep "ticks" to accept as 0, in Hz.
const SLEEP_TOLERANCE_TICKS : u16 = 100;

use irq::set_handler;

pub mod pit;

mod apict;

use io::{send_LAPIC_EOI};

/// A tick counter
static mut TICK_COUNTER : usize = 0;

/// Initialise the timer infrastructure. Please note that you must have
/// previously set up the IDT using `irq::install()`!
pub fn init(apic_addr: usize) {

    // Init the PIT
    //self::pit::init();

    set_handler(IRQ0_VEC, self::handle_timeout);

    apict::init(apic_addr, 4_000_000);
}


/// Busy sleep for ms milliseconds.
pub fn busy_sleep(ms : usize) {
    let start_time = get_ticks();

    while (get_ticks() - start_time) < ms {
        // do nothing
    }

}

/// Function to call when the timer times out.
/// Argument is ignored.
unsafe fn handle_timeout(_iv : usize) {
    println!("Timer reset! Now at {}", TICK_COUNTER);

    //set_timer(RATE_MAX);
    unsafe {

        // Nope, no race conditions here!
        TICK_COUNTER += 1;
    }

    // Send the End-of-Interrupt (EOI) signal to LAPIC:
    send_LAPIC_EOI();
}

/// Get the global tick count since the timer was started.
pub fn get_ticks() -> usize {
    unsafe {TICK_COUNTER}
}
