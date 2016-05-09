//! This is a module to handle and set up the (software) side of
//! interrupts. It (and its submodules) deals with setting up the IDT,
//! wrapping assembler magic and so on.
//! See also the x86 crate!
//!
//! # Usage
//! Before doing anything else, you need to perform
//! `irq::idt::install()` to configure the CPU to use the provided
//! IDT. This will install the module's default ISR:s for every
//! exception and interrupt. There is then two ways you can go about
//! adding your own ISRs:
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

// End modules and re-exports
