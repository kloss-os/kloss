//! This is a module to handle and set up the (software) side of
//! interrupts. It (and its submodules) deals with setting up the IDT,
//! wrapping assembler magic and so on.
//! See also the x86 crate!
//!
//! # Usage
//! Before doing anything else, you need to perform
//! `irq::idt::install()` to configure the CPU to use the provided
//! interrupt descriptor table (IDT). This will install the module's
//! default ISRs (Interrupt Service Routines) for every exception and
//! interrupt. There is then two ways you can go about adding your own
//! ISRs:
//!
//! 1. Add them using `irq::idt::set_gate()`. This will override the
//!    system routine in the IDT and the provided routine _must be an
//!    assembler routine_ following interrupt handler calling
//!    conventions until Rust supports naked functions.
//! 2. Add a Rust handler using `irq::set_handler()`. Note that this is
//!    _way slower_ and more indirect than the direct CPU dispatch, as it
//!    involves at least one layer of indirection between calls.
//!
//! Also note that you need to define and export the non-mangled function
//! `rust_interrupt_handler` from your main file, and in that function
//! call `irq::entry()`, that is the dispatch entry function.


// Modules and re-exports
pub mod idt;

mod asm_wrappers;

// FIXME: these should NOT BE PUBLIC
pub use self::asm_wrappers::*;

mod dispatch;

// Exception entry point re-export
pub use self::dispatch::entry;

// End modules and re-exports

/// Set the (module-internal) interrupt handler for vector `vec`.
///
/// **Warning**: This will only work if:
///
/// 1. The module's IDT was installed using `irq::install()`, and
/// 2. The ISR for `vec` has not been replaced with
///    `irq::idt::set_gate()`.
/// If you are in doubt, use `set_system_isr!()` to restore the system
/// ISR dispatcher.
///
/// - `vec` can be any integer in the range [0, 255].
/// - `f` must be a handling Rust function taking as its single
///   argument the triggered interrupt. `f` can be `unsafe`.
///
/// # Examples
///
pub fn set_handler(vec: u32,
                   f: unsafe fn(u32)) -> () {

}

/// Install/restore the system ISR for a given vector.
macro_rules! set_system_isr {
    ($x:expr) => (irq::idt::set_gate($x, irq::general_exception_handler,
                                     irq::idt::SELECT_TARGET_PRIV_1,
                                     irq::idt::FLAG_TYPE_TRAP_GATE
                                     | irq::idt::FLAG_DPL_KERNEL_MODE
                                     | irq::idt::FLAG_GATE_ENABLED));
}
