//! This module interfaces the APIC timer.
//! See also http://wiki.osdev.org/APIC_timer

use core::intrinsics::{volatile_store};

const APIC_LVT_TMR     : u32 = 0x320;
const APIC_TMRDIV      : u32 = 0x3E0;
const APIC_TMRINITCNT  : u32 = 0x380;
const TMR_PERIODIC     : u32 = 0x20000;
const TARGET_IV        : u32 = 32;


/// Set up the APIC timer with a counter of `rate` between interrupts.
///
/// # Safety
/// Requires a valid APIC address.
pub fn init(apic_addr: usize, rate: u32) {

    let apic_timer_irq = (apic_addr as u32 + APIC_LVT_TMR) as *mut u32;

    let apic_timer_divider = (apic_addr as u32 + APIC_TMRDIV) as *mut u32;

    let apic_timer_init_count = (apic_addr as u32 + APIC_TMRINITCNT) as *mut u32;

    unsafe {

        volatile_store(apic_timer_init_count, rate);

        volatile_store(apic_timer_irq, TARGET_IV | TMR_PERIODIC);

        volatile_store(apic_timer_divider, 0x3);
    }
}
