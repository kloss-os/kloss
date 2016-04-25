// This file contains functions related to reading and writing the IDT


const IDT_NUM_ENTRIES: usize = 256;

extern {
    fn _load_idt(x: *mut IdtPointer);
}


// Declare a static IDT.
#[no_mangle]
static mut idt: [IdtEntry; 256] = [IdtEntry
                                   {base_low: 0,
                                    selector: 0,
                                    reserved_zero: 0,
                                    flags: 0,
                                    base_high: 0,
                                    reserved_ist: 0,
                                    base_mid: 0}, ..256];

/// Load a new Interrupt Descriptor Table into the CPU.
///
/// # Safety
/// This function is safe if idt_pointer is the
/// address of a valid list of IDT entries.
pub unsafe fn idt_load(idt_pointer: *mut IdtPointer) {
    _load_idt(idt_pointer);
}

/* Use this function to set an entry in the IDT. A lot simpler
*  than twiddling with the GDT ;) */
/// Shamelessly stolen from Julia Evans.
/// Set interrupt handler for `num` to run function `f` using selector
/// `selector` and flags `flags`.
#[no_mangle]
unsafe fn idt_set_gate(num: u8, f: extern "C" fn(), selector: u16, flags: u8)
{

    // typecast the pointer to an int
    let base = f as u64;

    // Reserved sections: set them to 0
    idt[num].reserved_zero = 0;
    idt[num].reserved_ist = 0;

    // Set selector and flags
    idt[num].selector = selector;
    idt[num].flags = flags;


    // Split the pointer address into three parts: lower (16 bit),
    // middle (16 bit) and upper (32 bit).

    //idt[num].base_high = (base >> 16) as u16;
    idt[num].base_high = 0;
    idt[num].base_mid = 0;
    //idt[num].base_low = (base & (1 << 16 - 1)) as u16;
    idt[num].base_low = 0;
}

#[repr(C, packed)]
struct IdtPointer {
    lower: u16,
    base: u32,
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
