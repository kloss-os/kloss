#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]


mod rsdp;


/// A struct representing the ACPI SDT header, it has to be packed C-style
/// Based on OSDEV C struct
#[repr(C)]
pub struct ACPISDTHeader {
    signature:  [u8; 4],
    length:     u32,
    revision:   u8,
    checksum:   u8,
    OEMID:      [u8; 6],
    OEMTableID: [u8; 8],
    OEMRev:     u32,
    CreatorID:  u32,
    CreatorRev: u32,
}



