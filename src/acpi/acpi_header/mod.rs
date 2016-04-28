/// A struct representing the ACPI SDT header, it has to be packed C-style
/// Based on OSDEV C struct
#[repr(C)]
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
    Invalid,
}

/// Verifies struct by summing all its bytes and comparing to 0
pub unsafe fn verify_struct(address: usize) -> bool {
    let header = load_acpisdt_header(address);
    return sum_bytes(address, header.length as usize) == 0;
}

/// Casts an acpisdt header from an address
pub unsafe fn load_acpisdt_header(address: usize) -> &'static ACPISDTHeader {
    &*(address as *const ACPISDTHeader)
}


/// Helper to calculate the sum of an array of bytes.
/// #Safety
/// Be _certain_ that starting address is a valid memory space
unsafe fn sum_bytes(start: usize, len: usize) -> u8 {
    let mut sum: u32 = 0;

    for i in start..(start + len) {
        let current: u32 = *(i as *const u32) & 0xFF;
        sum = (sum + current) & 0xFF;
    }

    return sum as u8;
}

fn cmp_sig(a: [u8; 4], b: [u8; 4]) -> bool {
    a.iter().zip(b.iter()).all(|(x,y)| x == y)
}


pub fn find_type(header: &'static ACPISDTHeader) -> SDTtype {
    let sdt_sig: [(&[u8; 4], SDTtype); 2]=
        [(b"RSDT", SDTtype::RSDT),
         (b"APIC", SDTtype::MADT)];

    for i in header.signature.iter() {
        println!("SIG {}", *i as char);
    }
    for &(sig,sdtt) in sdt_sig.iter() {
        if sig.iter()
              .zip(header.signature.iter())
              .all(|(x,y)| x == y) {
            return sdtt;
        }
    }

    return SDTtype::Invalid;
}
