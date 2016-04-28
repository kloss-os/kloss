use self::acpi_header::ACPISDTHeader;

mod acpi_header;
mod rsdp;


/// A struct designating the _Root System Description Table_ (RSDT).
/// This contains pointers to all the other SDT's in the system.
pub struct RSDT {
    /// The header of the RSDT
    header:     ACPISDTHeader,
    /// A list of pointers to other SDTs, the constant here is _bad_ practice
    /// Might do for MVP. Should be easier to fix with more rust-skills
    sdt_ptrs:   [u32;64],
}


pub struct MADT {
    /// ACPI SDT Header
    header:     ACPISDTHeader,
    /// Local controller address
    local_ctrl: u32,
    /// Flags
    flags:      u32,
    /// Entries, need to research this
    entry:      [u32; 64]
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












/// This is mainly for testing, should delete once we're sure of things
pub fn get_rsdt() -> u8 {
    if let Some(rsdt) = load_rsdt() {
        println!("Loaded rsdt!");
        return 0x1;
    } else {
        println!("Didn't load rsdt!");
        return 0x0;
    }
}
