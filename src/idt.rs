// This file contains functions related to reading and writing the IDT, that is the global interrupt descriptor table.

/// The number of expected entries in the IDT.
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
pub unsafe fn idt_get_ptr()
                          -> *mut [IdtEntry; IDT_NUM_ENTRIES] {

    // This represents (or rather: will represent) the IDTR
    // special-purpose register, holding the base address and limit (max
    // length) of the IDT.
    // Note the stupid but unlikely default values!
    let mut idtr = IdtPointer{base: 17, limit: 17};

    // ASM Magic: Store the IDT address + limit at the memory pointed to
    // by the contents of EAX
    asm!("sidt [eax]"
         : // return nothing -- write directly to the IdtPointer struct!
         : "{eax}"(&mut idtr as *mut IdtPointer)
         : "{eax}" // don't clobber EAX
         : "intel" // use Intel syntax
    );

    println!("IDTR limit is: 0x{:x}", idtr.limit);
    println!("IDTR base is: 0x{:x}", idtr.base);

    if idtr.limit < 1 {
        // The IDT must have at least one entry!

        panic!("Expected the IDT to be longer!");
    }

    // This is in no way scary. It's type-casting a random blob of
    // memory to a mutable (pointer to an) array of a known size!
    return idtr.base as *mut [IdtEntry; IDT_NUM_ENTRIES];
}


const NULL_IDT_ENTRY: IdtEntry = IdtEntry {base_low: 0,
                                           selector: 0,
                                           reserved_zero: 0,
                                           flags: 0,
                                           base_high: 0,
                                           reserved_ist: 0,
                                           base_mid: 0};

// Declare a static IDT containing `IDT_NUM_ENTRIES`
// null entries.
#[no_mangle]
static mut idt: [IdtEntry; IDT_NUM_ENTRIES] =
    [NULL_IDT_ENTRY; IDT_NUM_ENTRIES];

/* Use this function to set an entry in the IDT. A lot simpler
*  than twiddling with the GDT ;) */
/// Shamelessly stolen from Julia Evans.
/// Set interrupt handler for `num` to run function `f` using selector
/// `selector` and flags `flags`.
#[no_mangle]
pub unsafe fn idt_set_gate(num: usize,
                           f: extern "C" fn(),
                           selector: u16, flags: u8)
{

    // typecast the function pointer to an int
    let service_routine_base = f as u64;

    // Reserved sections: set them to 0
    idt[num].reserved_zero = 0;
    idt[num].reserved_ist = 0;

    // Set selector and flags
    idt[num].selector = selector;
    idt[num].flags = flags;

    // Split the pointer address into three parts: lower (16 bit),
    // middle (16 bit) and upper (32 bit).

    //idt[num].base_high = (base >> 16) as u16;
    //idt[num].base_high = 0;
    //idt[num].base_mid = 0;
    //idt[num].base_low = (base & (1 << 16 - 1)) as u16;
    //idt[num].base_low = 0;
}

/// This represents the contents of an `IDTR` pointer,
/// that is a combination of an address of an IDT and
/// its limit (read: length).
#[repr(C, packed)]
struct IdtPointer {
    /// The limit of the IDT, that is its length.
    /// I don't know its unit, frankly, or if it is inclusive
    /// or exclusive of the table itself.
    limit: u16,
    /// The base address of the IDT, that is the address at which
    /// it starts.
    base: u64,
}

/// This represents an IDT entry. See inline comments for notes
/// on its various components!
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IdtEntry {
    /// The lower bits of the function pointer to the ISR.
    base_low: u16,
    /// FIXME: what does this do???
    selector: u16,
    /// Reserved space: should always be 0.
    reserved_ist: u8,
    /// FIXME: what does this do???
    flags: u8,
    /// The middle 16 bits of the function pointer to the ISR.
    base_mid: u16,
    /// The upper 32 bits of the function pointer to the ISR.
    base_high: u32,
    /// More reserved bits. Should always be 0.
    reserved_zero: u32,
}

/// Install the module's IDT in the kernel.
/// Note that you only need to run this once. It is safe to alter
/// the IDT both before and after installing it.
/// ## Safety
/// Replaces the contents of the `IDTR` special register.
pub unsafe fn idt_install() {

    // This is the length of one IDT entry.
    let idt_entry_size = super::core::mem::size_of::<IdtEntry>();

    // Determine the limit (read: length) of the IDT, for IDTR.
    let idt_limit = (idt_entry_size * IDT_NUM_ENTRIES) - 1;

    // Determine the start of the IDT, for IDTR.
    // Note that this is a _pointer_.
    let idt_base = &idt as *const [IdtEntry; IDT_NUM_ENTRIES];

    // This is the final value for the IDTR: a combination of
    // its limit and length, as determined above.
    let new_idtr = IdtPointer{base: idt_base as u64,
                              limit: idt_limit as u16};


    // Use the built-in assembly instructions to load the
    // new values for `IDTR` from memory. Note how the
    // pointer to `new_idtr` is copied as an immutable value --
    // there should be no overwriting done here, only reading!
    asm!("lidt [eax]"
         : // return nothing
         : "{eax}"(&new_idtr as *const IdtPointer)
         : "{eax}" // don't clobber EAX
         : "intel" // use Intel syntax
    );

}
