#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]


// A struct representing the descriptor, it has to be packed C-style
#[repr(C)]
pub struct RSDPdesc {
    signature:  [u8; 8],
    checksum:   u8,
    oemid:      [u8; 6],
    revision:   u8,
    rsdt_addr:  u32,
}


/// Find RSD Pointer
/// #Safety
/// Read-only from memory addresses
pub fn find_rsdp() -> usize {
    // Start by getting the ebda start address which is located at 0x40e
    // The address is a segmented 2-byte pointer
    let ebda_start: usize;

    // Since the address technically is a 20-bit integer, u16 is too small!
    let mut ebda_segment: u32;
    let mut ebda_offset: u32;

    // Get the pointer into one of the variables
    unsafe {
        asm!( "mov rax, [0x40e]"
            : "={rax}"(ebda_offset) :
            : "{rax}"
            : "intel" );
    }
    // Place the shifted version to the other
    ebda_segment = ebda_offset << 0x08;

    // Mask the unneccesary bits
    ebda_segment = ebda_segment & 0x000FFFF0;
    ebda_offset  = ebda_offset  & 0x00000001;

    // Now we simply add the two addresses using bitwise OR
    ebda_start = (ebda_segment | ebda_offset) as usize;

    // EBDA is max 1 KB  long, so we can get the end address
    let ebda_end = ebda_start + 0x400;


    // Iterate over possible EBDA area
    // Note that the identifier is on a 16-byte boundary
    let mut current: usize = ebda_start;
    while current <= ebda_end {
        if unsafe {verify_rsdp(current)} {
            // YES FOUND IT WOOOO
            return current;
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
        if unsafe{verify_rsdp(current)} {
            return current;
        } else {
            current = current + 0x10;
        }
    }

    return 0x0;
}



unsafe fn verify_rsdp(rsdpp: usize) -> bool {
    // We need an ASCII string containing "RSD PTR "
    let str: [u8; 8] = [82, 83, 68, 32, 80, 84, 82, 32];
    //let str: &[u8; 8] = b"RSD PTR ";

    // THIS IS HOW YOU RECAST SH... stuff
    // Phil Opp's multiboot2 code uses similar syntax
    let rsdpd = &*(rsdpp as *const RSDPdesc);

    // Check identifier string
    // Got this line, more or less, from
    // http://stackoverflow.com/questions/23148737/compare-definite-length-arrays
    if !str.iter().zip(rsdpd.signature.iter()).all(|(a,b)| a == b) {
        return false;
    }


    // Let's verify the checksum!
    // This must be done for every _byte_ in the struct
    let mut chksum: u32 = 0;

    // Sum signature
    // Should be constant, and already verified. Could skip this.
    for i in rsdpd.signature.iter() {
        chksum = chksum + (*i & 0xFF) as u32;
    }

    // Sum checksum
    chksum = chksum + (rsdpd.checksum & 0xFF) as u32;

    // Sum OEMID
    for i in rsdpd.oemid.iter() {
        //println!("OEMID {}", *i as char);
        chksum = chksum + (*i & 0xFF) as u32;
    }

    // Sum revision
    chksum = chksum + (rsdpd.revision & 0xFF) as u32;

    // Sum address, this is ugly since we need to add all four bytes
    chksum = chksum + ((rsdpd.rsdt_addr >> 0x18) & 0xFF);
    chksum = chksum + ((rsdpd.rsdt_addr >> 0x10) & 0xFF);
    chksum = chksum + ((rsdpd.rsdt_addr >> 0x08) & 0xFF);
    chksum = chksum + (rsdpd.rsdt_addr & 0xFF);


    // Mask all bytes above this
    chksum = chksum & 0xFF;



    // Debug lines for your pleasure!
    println!("RSDP checksum 0x{:x}", rsdpd.checksum);
    println!("ACPI version {}", rsdpd.revision);
    println!("RSDT address 0x{:x}", rsdpd.rsdt_addr);
    println!("Calculated checksum 0x{:x}", chksum);


    if chksum != 0 {
        return false;
    }

    if rsdpd.revision == 0 {
        return true;
    } else {
        //TODO: implement ACPI 2.0 support
        return false;
    }
}
