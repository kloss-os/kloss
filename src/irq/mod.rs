//! This is a module to handle and set up the (software) side of
//! interrupts. It (and its submodules) deals with setting up the IDT,
//! wrapping assembler magic and so on.
//! See also the x86 crate!
//!
//! # Usage
//! Before doing anything else, you need to perform
//! `irq::idt::install()` to configure the CPU to use the provided
//! IDT. This will install the module's default ISRs (Interrupt Service
//! Routines) for every exception and interrupt. There is then two ways
//! you can go about adding your own ISRs:
//!
//! 1. Add them using `irq::idt::set_gate()`. This will override the
//!    system routine and the provided routine _must be an assembler
//!    routine_ until Rust supports naked functions.
//! 2. Add a Rust handler using `irq::set_handler()`. Note that this is
//!    _way slower_ and more indirect than the direct CPU dispatch, as it
//!    involves at least one layer of indirection between calls.


// Modules and re-exports
pub mod idt;

mod asm_wrappers;

// FIXME: these should NOT BE PUBLIC
pub use self::asm_wrappers::*;

mod dispatch;

// End modules and re-exports

/// Set the (module-internal) interrupt handler for vector `vec`.
///
/// **Warning**: This will only work if:
///
/// 1. The module's IDT was installed using `irq::install()`, and
/// 2. The ISR for `vec` has not been replaced with
///    `irq::idt::set_gate()`.
/// If you are in doubt, use `set_system_isr()` to restore the system
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

/// Install/restore the system ISR for vector `vec`.
pub fn set_system_isr(vec: u32) -> () {

}
