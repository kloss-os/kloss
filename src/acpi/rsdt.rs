use super::acpi_header::ACPISDTHeader;
use super::rsdp::RSDPdesc;


/// A struct designating the _Root System Description Table_ (RSDT).
/// This contains pointers to all the other SDT's in the system.
#[repr(C)]
pub struct RSDT {
    /// The header of the RSDT
    header:     ACPISDTHeader,
    /// A list of pointers to other SDTs, the constant here is _bad_ practice
    sdt_ptrs:   [u32;64],
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
        if let Some(rsdt) = unsafe{ load_rsdt_addr(rsdp.rsdt_addr as usize) } {
            return Some(rsdt);
        } else { return None; }
    } else {
        return None;
    }
}
