// This file contains functions related to reading and writing the IDT, that is the global interrupt descriptor table.

#![feature(asm)]


const IDT_NUM_ENTRIES: usize = 256;

extern {
    fn _load_idt(x: *mut IdtPointer);
}

/// Return a pointer to the Global IDT as given by
/// The `SIDT` instruction, but with the offset data
/// set to zero (e.g. should actually return the start of
/// the IDT).
///
/// It is assumed that the IDT is large enough to cover all
/// 256 gates, and will panic if the limit says otherwise.
///
/// ## Safety
/// This function should work if `IDTR` points to a valid
/// IDT. Otherwise, all bets are off.
pub unsafe fn idt_get_ptr() -> *mut [IdtEntry; 256] {


    let mut idtr_contents = IdtPointer{base: 0, limit: 0};

    asm!("sidt [eax]"
         : // return nothing -- write directly to memory
         : "{eax}"(&mut idtr_contents as *mut IdtPointer)
         : "{eax}"
         : "intel");

    println!("IDTR limit is: 0x{:x}", idtr_contents.limit);
    println!("IDTR base is: 0x{:x}", idtr_contents.base);

    // This is in no way scary.
    return idtr_contents.base as *mut [IdtEntry; 256];
}


const NULL_IDT_ENTRY: IdtEntry = IdtEntry {base_low: 0,
                                           selector: 0,
                                           reserved_zero: 0,
                                           flags: 0,
                                           base_high: 0,
                                           reserved_ist: 0,
                                           base_mid: 0};

// Declare a static IDT.
#[no_mangle]
static mut idt: [IdtEntry; 256] = [NULL_IDT_ENTRY; 256];

// /// Load a new Interrupt Descriptor Table into the CPU.
// ///
// /// # Safety
// /// This function is safe if idt_pointer is the
// /// address of a valid list of IDT entries.
// pub unsafe fn idt_load() {
//     _load_idt(0);
// }

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
    //idt[num].reserved_zero = 0;
    //idt[num].reserved_ist = 0;

    // Set selector and flags
    //idt[num].selector = selector;
    //idt[num].flags = flags;


    // Split the pointer address into three parts: lower (16 bit),
    // middle (16 bit) and upper (32 bit).

    //idt[num].base_high = (base >> 16) as u16;
    //idt[num].base_high = 0;
    //idt[num].base_mid = 0;
    //idt[num].base_low = (base & (1 << 16 - 1)) as u16;
    //idt[num].base_low = 0;
}

#[repr(C, packed)]
struct IdtPointer {
    limit: u16,
    base: u64,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IdtEntry {
    base_low: u16,
    selector: u16,
    reserved_ist: u8, // 0
    flags: u8,
    base_mid: u16,
    base_high: u32,
    reserved_zero: u32,
}
