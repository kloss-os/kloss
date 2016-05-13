use core::mem;

/// A struct representing the ACPI SDT header, it has to be packed C-style
/// Based on OSDEV C struct
/// Total size of header is 36 bytes (0x24)
#[repr(C, packed)]
pub struct ACPISDTHeader {
    pub signature:  [u8; 4],
    pub length:     u32,
    pub revision:   u8,
    pub checksum:   u8,
    pub oemid:      [u8; 6],
    pub oem_table_id:   [u8; 8],
    pub oem_rev:     u32,
    pub creator_id:  u32,
    pub creator_rev: u32,
}

#[derive(PartialEq,Clone,Copy)]
pub enum SDTtype {
    RSDT,
    MADT,
}



/// A struct designating the _Root System Description Table_ (RSDT).
/// This contains pointers to all the other SDT's in the system.
/// Signature is "RSDT"
#[repr(C, packed)]
pub struct RSDT {
    /// The header of the RSDT
    pub header:     ACPISDTHeader,
    /// This is the first pointer in an array
    /// You can use the first pointer, along with .offset() method to increment by 32-bit boundary.
    /// The length of the array is given by rsdt.header.length - mem::size_of::<ACPISDTHeader>()
    pub first_ptr:  u32,
}




/// The _Multiple APIC Description Table_ (MADT) describes all interrupt controllers
/// Signature is "APIC"
#[repr(C, packed)]
pub struct MADT {
    /// ACPI SDT Header
    pub header:     ACPISDTHeader,
    /// Local controller address
    pub local_ctrl: u32,
    /// Flags
    pub flags:      u32,

    /// After this, we have a variable length sequence of variable length records.
    /// Using .offset() it will increment the address by 16 bits.
    /// Since _it seems_ the ISO struct is on a 16-bit boundary, this makes sense.
    pub first_intctr: u8,
}



/// Cast a pointer to sdt::RSDT
/// #Safety
/// Risk of reading forbidden memory. Be careful to use this on the right address!
pub unsafe fn load_rsdt(rsdt_addr: usize) -> Option<&'static RSDT> {
    if verify_struct(rsdt_addr) {
        return Some(&*(rsdt_addr as *const RSDT));
    } else {
        return None;
    }
}

/// Finds an MADT table in RSDT
/// Should be totes safe if RSDT is valid and readable
pub fn load_madt(rsdt: &'static RSDT) -> Option<&'static MADT> {
    // Make a raw pointer for the first SDT pointer
    let first: *const u32 = unsafe { &rsdt.first_ptr };

    // Calculate amount of pointers
    let num_ptr = (rsdt.header.length
                   - mem::size_of::<ACPISDTHeader>() as u32)
                    / 4;

    // Step through the list until desired table is found
    for i in 0..num_ptr {
        let current = unsafe { first.offset(i as isize) };
        if  unsafe { verify_struct(*current as usize) } &&
            find_type(
                unsafe { load_acpisdt_header(
                    *current as usize) } )
            == Some(SDTtype::MADT)
        {
            return Some( unsafe { &*(*current as *const MADT) } );
        }
    }

    return None;
}




/// Verifies struct by summing all its bytes and comparing to 0
/// Requires an address as usize
/// # Safety
/// Be _certain_ that starting address is a valid memory space
pub unsafe fn verify_struct(address: usize) -> bool {
    let header = load_acpisdt_header(address);
    return !(header.length < 0xFFFF) || sum_bytes(address, header.length as usize) == 0;
}

/// Casts an acpisdt header from an address
/// # Safety
/// Be _certain_ that starting address is a valid memory space
pub unsafe fn load_acpisdt_header(address: usize) -> &'static ACPISDTHeader {
    &*(address as *const ACPISDTHeader)
}


/// Helper to calculate the sum of an array of bytes.
/// # Safety
/// Be _certain_ that starting address is a valid memory space
unsafe fn sum_bytes(start: usize, len: usize) -> u8 {
    let mut sum: u32 = 0;

    println!("length: {}", len);
    for i in start..(start + len) {
        let current: u32 = *(i as *const u32) & 0xFF;
        sum = (sum + current) & 0xFF;
    }

    return sum as u8;
}



/// Given a header, extract and return the type it has
pub fn find_type(header: &'static ACPISDTHeader) -> Option<SDTtype> {
    let sdt_sig: [(&[u8; 4], SDTtype); 2]=
        [(b"RSDT", SDTtype::RSDT),
         (b"APIC", SDTtype::MADT)];

    /* Uncomment for printing signatures!
    for i in header.signature.iter() {
        println!("SIG {}", *i as char);
    }
    */

    for &(sig,sdtt) in sdt_sig.iter() {
        if sig.iter()
              .zip(header.signature.iter())
              .all(|(x,y)| x == y) {
            return Some(sdtt);
        }
    }

    return None;
}

