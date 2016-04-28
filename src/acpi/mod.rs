use self::acpi_header::ACPISDTHeader;

mod acpi_header;
mod rsdp;



/// A struct designating the _Root System Description Table_ (RSDT).
/// This contains pointers to all the other SDT's in the system.
/// Signature is "RSDT"
#[repr(C)]
pub struct RSDT {
    /// The header of the RSDT
    header:     ACPISDTHeader,
    /// A list of pointers to other SDTs, the constant here is _bad_ practice
    /// Might do for MVP. Should be easier to fix with more rust-skills
    sdt_ptrs:   [u32;4],
}





/// The _Multiple APIC Description Table_ (MADT) describes all interrupt controllers
/// Signature is "APIC"
#[repr(C)]
pub struct MADT {
    /// ACPI SDT Header
    header:     ACPISDTHeader,
    /// Local controller address
    local_ctrl: u32,
    /// Flags
    flags:      u32,

    // After this, we have a variable length sequence
    // of variable length records
}

/// Universal header for interrupt controller description
#[repr(C)]
pub struct IntCtrlHeader {
    /// Describes type of interrupt controller
    entry_type:     u8,
    /// Length of its table
    record_length:  u8,
}

/// _Processor Local APIC_ (LAPIC), indicated by entry type 0
/// Represents a single physical processor and its local interrupt controller
#[repr(C)]
pub struct LAPIC {
    /// Universal header
    header:     IntCtrlHeader,
    /// ACPI Processor ID
    acpi_proc_id: u8,
    /// APIC ID
    apic_id:    u8,
    /// Flags, if 1 it is enabled
    flags:      u32,
}

/// _I/O APIC_ (IOAPIC), indicated by entry type 1
#[repr(C)]
pub struct IOAPIC {
    /// Universal header
    header:   IntCtrlHeader,
    /// I/O APIC's ID
    id:       u8,
    /// Reserved, value is 0 (_padding?_)
    reserved: u8,
    /// I/O APIC's Address
    address:  u32,
}

/// _Interrupt Source Override_, indicated by entry type 2
#[repr(C)]
pub struct ISO {
    /// Universal header
    header:     IntCtrlHeader,
    /// Bus Source
    bus_source: u8,
    /// IRQ Source
    irq_source: u8,
    /// Global System Interrupt
    global_sys_int: u32,
    /// Flags
    flags:      u16,
}





/// Cast a pointer to RSDT
/// #Safety
/// Risk of reading forbidden memory. Be careful to use this on the right address!
unsafe fn load_rsdt_addr(rsdt_addr: usize) -> Option<&'static RSDT> {
    if acpi_header::verify_struct(rsdt_addr) {
        return Some(&*(rsdt_addr as *const RSDT));
    } else {
        return None;
    }
}

/// Loads an RSDT, scary stuff!
pub fn load_rsdt() -> Option<&'static RSDT> {
    if let Some(rsdp) = rsdp::load_rsdp() {
        if let Some(rsdt_root) = unsafe{load_rsdt_addr(rsdp.rsdt_addr as usize)} {
            return Some(rsdt_root);
        } else { return None; }
    } else {
        return None;
    }
}


/// Finds a table of a certain type in an rsdt
pub unsafe fn find_madt(rsdt: &'static RSDT) -> Option<&'static MADT> {
    for sdth in rsdt.sdt_ptrs.iter() {
        if  acpi_header::verify_struct(*sdth as usize) &&
            acpi_header::find_type(
                acpi_header::load_acpisdt_header(
                    *sdth as usize))
            == acpi_header::SDTtype::MADT
            {
                return Some(&*(*sdth as *const MADT));
            }
    }

    return None;
}



/// This is mainly for testing, should delete once we're sure of things
pub fn get_rsdt() -> u8 {

    if let Some(rsdt) = load_rsdt() {
        println!("Loaded rsdt, length is 0x{:x}!", rsdt.header.length);

        if let Some(madt) = unsafe { find_madt(rsdt) } {
            println!("Loaded madt!");
            println!("Length: {:x}", madt.header.length);
        } else {
            println!("Not loaded madt!");
        }

        return 0x1;
    } else {
        println!("Didn't load rsdt!");
        return 0x0;
    }
}
