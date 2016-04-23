// This file contains functions related to reading and writing the IDT


const IDT_NUM_ENTRIES: usize = 255;

extern {
    fn _load_idt(x: u64);
}

/// Load a new Interrupt Descriptor Table into the CPU.
///
/// # Safety
/// This function is safe if idt_pointer is the
/// address of a valid list of IDT entries.
pub unsafe fn idt_load(idt_pointer: u64) {
    _load_idt(idt_pointer);
}


#[repr(C, packed)]
struct IdtEntry {
    base_low: u16,
    selector: u16,
    reserved_ist: u8, // 0
    flags: u8,
    base_mid: u16,
    base_high: u32,
    reserved_zero: u32,
}
