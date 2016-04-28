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


