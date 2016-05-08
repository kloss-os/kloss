#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]



/// A struct representing the RSDP descriptor, it has to be packed C-style
/// 48 bytes in total
/// Based on OSDEV C struct
#[repr(C)]
pub struct RSDPdesc {
    /// Signature, is guaranteed to be "RSD PTR "
    signature:  [u8; 8],
    /// Checksum modifier, such that the sum of all bytes & 0xFF == 0x00
    pub checksum:   u8,
    /// OEM ID
    pub oemid:      [u8; 6],
    /// Indicates ACPI version
    pub revision:   u8,
    /// Pointer to RSDT/XSDT
    pub rsdt_addr:  u32,
}
// TODO: make extended table work (ACPIv2 and later)


/// Find RSD Pointer
/// #Safety
/// Read-only from memory addresses, but still terrifying
pub fn load_rsdp() -> Option<&'static RSDPdesc> {
    // Start by getting the ebda start address which is located at 0x40e
    // The address is a segmented 2-byte pointer

    let ebda_start: usize;
    let ebda_ptr: *const u32;

    unsafe {
        ebda_ptr = &*(0x40e as *const u32);

        // Since the address technically is a 20-bit integer, u16 is too small!
        let mut ebda_segment: u32;
        let mut ebda_offset: u32;

        // Mask the unneccesary bits
        ebda_segment = (*ebda_ptr << 0x08) & 0x000FFFF0;
        ebda_offset  = *ebda_ptr & 0x00000001;

        // Now we simply add the two addresses using bitwise OR
        ebda_start = (ebda_segment | ebda_offset) as usize;
    }


    // EBDA is max 1 KB  long, so we can get the end address
    let ebda_end = ebda_start + 0x400;


    // Iterate over possible EBDA area
    // Note that the identifier is on a 16-byte boundary
    let mut current: usize = ebda_start;
    while current <= ebda_end {
        if let Some(rsdp) = unsafe {load_rsdp_addr(current)} {
            // YES FOUND IT WOOOO
            return Some(rsdp);
        } else {
            // Check next address, 16 bytes ahead!
            current = current + 0x10;
        }
    }


    // It can also be located somewhere between the following addresses
    let mbos_start: usize = 0x000E0000;
    let mbos_end:   usize = 0x000FFFFF;
    current = mbos_start;

    // Same as above, but in MBOS
    while current <= mbos_end {
        if let Some(rsdp) = unsafe {load_rsdp_addr(current)} {
            // YES FOUND IT WOOOO
            return Some(rsdp);
        } else {
            // Check next address, 16 bytes ahead!
            current = current + 0x10;
        }
    }

    return None;
}


/// Cast an address as an RSDP descriptor, verify it, return if valid
/// # Safety
/// This _will_ (only) read raw memory data, so don't throw forbidden addresses into it
unsafe fn load_rsdp_addr(rsdp_addr: usize) -> Option<&'static RSDPdesc> {
    // THIS IS HOW YOU RECAST SH... stuff
    // Phil Opp's multiboot2 code uses similar syntax
    if unsafe { verify_rsdp(rsdp_addr) } {
        return Some(&*(rsdp_addr as *const RSDPdesc));
    } else {
        return None;
    }
}



/// Verifies a given RSDP descriptor (looks at ID and checksum)
unsafe fn verify_rsdp(rsdp_addr: usize) -> bool {
    let rsdpd = &*(rsdp_addr as *const RSDPdesc);

    // We need an ASCII string containing "RSD PTR "
    let str: &[u8; 8] = b"RSD PTR ";

    // Check identifier string
    // Got this line, more or less, from
    // http://stackoverflow.com/questions/23148737/compare-definite-length-arrays
    if !str.iter().zip(rsdpd.signature.iter()).all(|(a,b)| a == b) {
        return false;
    }


    // Let's verify the checksum!
    // This must be done for every _byte_ in the struct
    // Due to overflow risk we make it bigger than u8
    let mut chksum: u32 = 0;


    // RSDP descriptor is 48 bytes long
    let start = rsdp_addr as *const u8;
    for i in 0..(0x30) {
        //cur_val: u8 = *start.offset(i as isize);
        chksum = chksum & 0xFF + *start.offset(i as isize) as u32;
    }

    // Return masked sum, we don't care about values above LSByte
    return chksum & 0xFF == 0;
}
