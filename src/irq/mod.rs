//! This is a module to handle and set up the (software) side of
//! interrupts. It (and its submodules) deals with setting up the IDT,
//! wrapping assembler magic and so on.
//! See also the x86 crate!


pub mod idt;

// fixme: these should NOT BE PUBLIC
extern {
    pub fn general_interrupt_handler();
    pub fn general_exception_handler();
    pub fn null_interrupt_handler();
    pub fn isr_42();
    pub fn isr_255();
    pub fn isr_12();
}
