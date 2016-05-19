//! This is a module to handle and set up timers. It also provides a
//! very weak callback infrastructure to enable timer hooks.

/// The interrupt vector at which IRQ0 arrives from the PIT.
const IRQ0_VEC : usize = 32;

/// The tolerance for number of sleep "ticks" to accept as 0, in Hz.
const SLEEP_TOLERANCE_TICKS : u16 = 100;

use irq::set_handler;

pub mod pit;

/// Initialise the timer infrastructure. Please note that you must have
/// previously set up the IDT using `irq::install()`!
pub fn init() {

    // Init the PIT
    self::pit::init();

     set_handler(IRQ0_VEC, self::pit::handle_timeout);
}


/// Busy sleep for ms milliseconds.
pub fn busy_sleep(ms : usize) {
    let start_time = pit::get_ticks();
    let rate = pit::ms_per_tick();

    while (pit::get_ticks() - start_time) < ms {
        println!("Sleeping, current no ticks: {}", pit::get_ticks());
        // do nothing
    }

}
