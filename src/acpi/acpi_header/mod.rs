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


#[repr(C)]
pub struct SDT<T> {
    header: ACPISDTHeader,
    data: T,
}


impl<T> SDT<T> {
    pub unsafe fn load_struct(address: usize) -> Option<T> {
        if verify_struct(address) {
            return Some(&*(address as *const T));
        } else {
            return None
        }
    }

}

/// Verifies struct by summing all its bytes and comparing to 0
pub unsafe fn verify_struct(address: usize) -> bool {
    let header = &*(address as *const ACPISDTHeader);
    return sum_bytes(address, header.length as usize) == 0;
}

/// Helper to calculate the sum of an array of bytes.
/// #Safety
/// Be _certain_ that starting address is a valid memory space
unsafe fn sum_bytes(start: usize, len: usize) -> u8 {
    let mut sum: u32 = 0;

    for i in start..(start + len) {
        let current: u32 = *(i as *const u32);
        sum = (sum + current) & 0xFF;
    }

    return sum as u8;
}
