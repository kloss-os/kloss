// This file contains functions related to reading and writing the IDT, that is the global interrupt descriptor table.

/// The number of expected entries in the IDT.
const IDT_NUM_ENTRIES: usize = 256;

/// Bitmask to pick out the lower 16 bits of a 64 bit integer.
const LOWER_16_MASK_64: u64 = 0x000000000000ffff;

/// Bitmask to pick out the middle 16 bits of a 64-bit integer,
/// as counted from the least-significant bit.
const MID_16_MASK_64: u64 = 0x00000000ffff0000;

/*
 ** Here follows bitmasks for various interesting IDT flags: **
 */

/// Magic code for a trap gate type.
pub const FLAG_TYPE_TRAP_GATE: u8 = 0b00001110;

/// Magic code for DPL kernel-level access:
pub const FLAG_DPL_KERNEL_MODE: u8 = 0;

/// Magic code for DPL user mode access:
pub const FLAG_DPL_USER_MODE: u8 = 0b01100000;

/// Magic code for an enabled gate
pub const FLAG_GATE_ENABLED: u8 = 0b10000000;

/// Dummy value for a disabled gate.
pub const FLAG_GATE_DISABLED: u8 = 0;

/// Dummy flag for a disabled interrupt stack table,
/// which enables the legacy mode. See the Intel programmer's
/// Manual for more information.
pub const INTERRUPT_STACK_TABLE_LEGACY :u8 = 0;

/*
 ** End of bitmasks **
 */

/// This represents a null (and disabled) IDT entry: it's
/// simply a blob of zeroes. Used for initialisation.
const NULL_IDT_ENTRY: IdtEntry = IdtEntry {base_low: 0,
                                           selector: 0,
                                           reserved_zero: 0,
                                           flags: 0,
                                           base_high: 0,
                                           reserved_ist: 0,
                                           base_mid: 0};
/*
// This isn't used anymore, but it's left for all the fond memories.
extern {
    fn _load_idt(x: *mut IdtPointer);
}
*/

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
    let service_routine_address = f as u64;

    // Reserved sections: set them to 0
    idt[num].reserved_zero = 0;
    idt[num].reserved_ist = 0;

    // Set selector and flags
    idt[num].selector = selector;
    idt[num].flags = flags;

    // Split the pointer address into three parts:
    // lower (16 bit), middle (16 bit), and upper (32 bit).

    // Right-shift out the 32 upper bits.
    idt[num].base_high = (service_routine_address >> 32) as u32;

    // Pick out the lower 16 bits with a logical AND
    idt[num].base_low = (service_routine_address
        & LOWER_16_MASK_64) as u16;

    // Pick out the middle 16 bits (note that we need to
    // right-shift after AND:ing with the relevant mask).
    idt[num].base_mid = ((service_routine_address
                          & MID_16_MASK_64) >> 16) as u16;

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
    /// Contains, MSB-to-LSB:
    /// - Present? (0b0-0b1)
    /// - Privilege Level (0b00-0b11)
    /// - A zero (0)
    /// - Type: Kind of gate. 0b0000-1111.
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
    // its limit and address, as determined above.
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
