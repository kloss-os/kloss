//! #I/O Module
//!
//! Contains functions to read/write IOAPIC registers and disabling legacy PIC.
//!
//! Note that the default IOAPIC address is 0xFEC00000, it can be found using ACPI module.


use core::intrinsics::{volatile_load, volatile_store};

// Masks for reserved bits
const IOWIN_RESERVED_LO : u32 = 0b0101 << 24;
const IOWIN_RESERVED_HI : u32 = 0x00FF_FFFF;

// Conventional registry offsets, write to IOREGSEL
const KBD_IOWIN_LO : u32 = 0x12;
const KBD_IOWIN_HI : u32 = 0x13;



/// Read contents of an IOWIN
///
/// # Parameters
/// + **ioapicaddr:** Address for accessing IOREGSEL and IOWIN
/// + **reg:** Register to be selected and read
///
/// # Safety
/// + Only so many registers can exist
/// + Given address must be valid and R/W
unsafe fn read_ioapic(ioapicaddr: *mut u32, reg: u32) -> u32 {
    // Set IOWIN address
    let iowin = (ioapicaddr as *const u32).offset(4);

    // Write the selected register to IOREGSEL
    volatile_store(ioapicaddr, reg & 0xFF);
    volatile_load( iowin )
}

/// Writes given data to the given address
///
/// # Parameters
/// + **ioapicaddr:** Address for accessing IOREGSEL and IOWIN
/// + **reg:** Register to be selected and read
/// + **data:** Data to be written, note that some registers are read-only
///
/// # Safety
/// + Only so many registers can exist
/// + Requires a valid R/W address, note that an offset of 0x10 is used as well
/// + Data _must_ be read and masked with corresponding reserved bytes
unsafe fn write_ioapic(ioapicaddr: *mut u32, reg: u32, data: u32) {
    // Set IOWIN address
    let iowin = (ioapicaddr as *const u32).offset(4) as *mut u32;

    // Write the selected register to IOREGSEL
    volatile_store(ioapicaddr, reg);
    volatile_store(iowin, data);
}


/// Prints the first registers of IOREG
///
/// # Parameters
/// + **ioapicaddr:** Address for accessing IOREGSEL and IOWIN
///
/// # Safety
/// Requires a valid R/W address, note that an offset of 0x10 is used as well
unsafe fn print_ioreg(ioapicaddr: *mut u32) {
    let id  = read_ioapic(ioapicaddr, 0x00);
    let ver = read_ioapic(ioapicaddr, 0x01);
    let arb = read_ioapic(ioapicaddr, 0x02);

    println!("IOAPIC ID: {:x}, VER: {:x}, ARB: {:x}",
             id, ver, arb);

}


/// Generates a redirection table, based on an IOAPIC address
///
/// # Parameters
/// + **ioapicaddr:** Address for accessing IOREGSEL and IOWIN
///
/// # Safety
/// Requires a valid R/W address, note that an offset of 0x10 is used as well
pub unsafe fn gen_ioredtable(ioapicaddr: *mut u32) {
    let read_kbd_hi = read_ioapic(ioapicaddr, KBD_IOWIN_HI) & IOWIN_RESERVED_HI;
    let read_kbd_lo = read_ioapic(ioapicaddr, KBD_IOWIN_LO) & IOWIN_RESERVED_LO;

    let (kbd_hi, kbd_lo) =
        gen_irq(0, 0, 0, 0, 0, 0b000, 0x80);

    write_ioapic(ioapicaddr, KBD_IOWIN_HI, kbd_hi | read_kbd_hi);
    write_ioapic(ioapicaddr, KBD_IOWIN_LO, kbd_lo | read_kbd_lo);


    let res_kbd_hi = read_ioapic(ioapicaddr, KBD_IOWIN_HI);
    let res_kbd_lo = read_ioapic(ioapicaddr, KBD_IOWIN_LO);

    println!("KBD INT HI: {:x}, KBD INT LO: {:x}",
             res_kbd_hi, res_kbd_lo);
}



/// Generates two 32-bit registers to be written in a redirection table
///
/// # Parameters
/// + **dest:** _destination field_, specifies which LAPIC (by ID) interrupt is sent to
/// + **dest_mod:** _destination mode_, 1-bit, specifies whether (1) dest is a range, or (0) dest is a single address
/// + **mask:** _interrupt mask_, 1-bit, (1) disables the IRQ, but trouble might occur if an interrupt has already been accepted by the LAPIC
/// + **trig:** _trigger mode_, 1-bit, makes the signal (1) level sensitive or (0) edge sensitive
/// + **pol:** _interrupt pin polarity_, 1-bit, specifies whether (0) high active or (1) low active
/// + **del_mod:** _delivery mode_, is a _3-bit_ field specifying which specified LAPIC's to send
/// interrupt to: e g `000` sends to all, while `001` sends to processor with lowest priority execution
/// + **int_vec:** _interrupt vector_, sets interrupt vector for this IRQ, range is 0x10-0xFE
fn gen_irq(dest: u8, dest_mod: u8, mask: u8, trig: u8, pol: u8, del_mod: u8, int_vec: u8) -> (u32, u32) {
    let hi: u32 = (dest as u32) << 24;
    let lo: u32 =   (((mask as u32) & 0b1) << 16)
                  | (((trig as u32) & 0b1) << 15)
                  | (((pol as u32) & 0b1) << 13)
                  | (((dest_mod as u32) & 0b1) << 11)
                  | (((del_mod as u32) & 0b111) << 8)
                  | int_vec as u32;

    (hi, lo)
}


/// Masks the PIC IRQ's such the signals are ignored
///
/// # Safety
/// + Will start playing around with addresses 0x20, 0x21, 0xA0, 0xA1
unsafe fn mask_pic_irq() {
    let pic1_data: u16 = 0x21;
    let pic2_data: u16 = 0xA1;

    for i in 0..8 {
        let current_pic = if i > 8 { pic1_data } else { pic2_data };
        let current_irq = i % 8;
        let value: u8;
        asm!("in al, dx"
             : "={al}"(value)
             : "{dx}"(current_pic)
             : "{al}","{dx}"
             : "intel" );

        let masked = value | (1 << current_irq);
        asm!("out dx, al"
             :
             : "{dx}"(current_pic),"{al}"(value)
             : "{al}","{dx}"
             : "intel" );
    }
}


/// Remaps PIC IRQ's such that vector offsets are 32:39 and 40:47 for PIC1 and PIC2 respectively
///
/// # Safety
/// Will start playing around with addresses 0x20, 0x21, 0xA0, 0xA1
unsafe fn remap_pic() {
    // https://en.wikibooks.org/wiki/X86_Assembly/Programmable_Interrupt_Controller#Remapping

    asm!("mov al, 0x11\n\t\
          out 0x20, al\n\t\
          out 0xA0, al\n\t\
          mov al, 0x20\n\t\
          out 0x21, al\n\t\
          mov al, 0x28\n\t\
          out 0xA1, al\n\t\
          mov al, 0x04\n\t\
          out 0x21, al\n\t\
          mov al, 0x02\n\t\
          out 0xA1, al\n\t\
          mov al, 0x01\n\t\
          out 0x21, al\n\t\
          out 0xA1, al"
        :
        :
        : "{al}"
        : "intel"
        )
}


/// Disables PIC, should work as long as PIC is conventional
pub fn disable_pic() {
    unsafe {
        mask_pic_irq();
        remap_pic();
    }
}
