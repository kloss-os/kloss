//! #APIC Module
//! _Advanced Programmable Interrupt Controller_
//!
//! Contains structs and functions that can be found in the _MADT_, one of many SDTs
//!
//! APICs are used for interrupts via I/O and Interprocessor communication


use core::mem;
use acpi::sdt;

/// Universal header for interrupt controller description
#[repr(C, packed)]
pub struct IntCtrlHeaderEntry {
    /// Describes type of interrupt controller
    pub entry_type:     u8,
    /// Length of its table
    pub record_length:  u8,
}



/// _Processor Local APIC_ (LAPIC, packed), indicated by entry type 0
/// Represents a single physical processor and its local interrupt controller
#[repr(C, packed)]
pub struct LAPICEntry {
    /// Universal header
    pub header:     IntCtrlHeaderEntry,
    /// ACPI Processor ID
    pub acpi_proc_id: u8,
    /// APIC ID
    pub apic_id:    u8,
    /// Flags, if 1 it is enabled
    pub flags:      u32,
}




/// _I/O APIC_ (IOAPIC), indicated by entry type 1
#[repr(C, packed)]
pub struct IOAPICEntry {
    /// Universal header
    pub header:   IntCtrlHeaderEntry,
    /// I/O APIC's ID
    pub id:       u8,
    /// Reserved, value is 0 (_padding?_)
    pub reserved: u8,
    /// I/O APIC's Address
    pub address:  u32,
    /// Global System Interrupt Base
    pub gsib:     u32,

}




/// _I/O SAPIC_ (IOSAPIC), indicated by entry type 6
#[repr(C, packed)]
pub struct IOSAPICEntry {
    /// Universal header
    pub header:   IntCtrlHeaderEntry,
    /// I/O APIC's ID
    pub id:       u8,
    /// Reserved, value is 0 (_padding?_)
    pub reserved: u8,
    /// I/O APIC's Address
    pub address:  u32,
    /// Global System Interrupt Base
    pub gsib:     u64,

}



/// _Interrupt Source Override_, indicated by entry type 2
#[repr(C, packed)]
pub struct ISOEntry {
    /// Universal header
    pub header:     IntCtrlHeaderEntry,
    /// Bus Source
    pub bus_source: u8,
    /// IRQ Source
    pub irq_source: u8,
    /// Global System Interrupt
    pub global_sys_int: u32,
    /// Flags
    pub flags:      u16,
}




/// Get the (first) I/O APIC
/// # Safety
/// A valid and readable MADT should give a valid result
pub fn load_ioapic_entry(madt: &'static sdt::MADT) -> Option<&'static IOAPICEntry> {
    // Get first byte address
    let start: *const u8 = &madt.first_intctr;

    // Get amount of bytes in bound
    let num_addr = madt.header.length
                    - mem::size_of::<sdt::ACPISDTHeader>() as u32
                    - (4 + 4); // Remove size of descriptors


    // Iterate over bound
    let mut i = 0;
    while i < num_addr {
        let cur_addr = unsafe { start.offset(i as isize) };
        let cur_head = unsafe { &*(cur_addr as *const IntCtrlHeaderEntry) };

        if cur_head.entry_type == 0b01 {
            return Some( unsafe { &*(cur_addr as *const IOAPICEntry) } );
        } else {
            i += cur_head.record_length as u32;
        }
    }

    return None;
}


/// Get the (first) I/O SAPIC
pub unsafe fn load_iosapic_entry(madt: &'static sdt::MADT) -> Option<&'static IOSAPICEntry> {
    // Get first byte address
    let start: *const u8 = &madt.first_intctr;

    // Get amount of bytes in bound
    let num_addr = madt.header.length
                    - mem::size_of::<sdt::ACPISDTHeader>() as u32
                    - (4 + 4); // Remove size of descriptors


    // Iterate over bound
    let mut i = 0;
    while i < num_addr {
        let cur_addr = start.offset(i as isize);
        let cur_head = &*(cur_addr as *const IntCtrlHeaderEntry);

        if cur_head.entry_type == 0x06 {
            return Some( &*(*cur_addr as *const IOSAPICEntry) );
        } else {
            i += cur_head.record_length as u32;
        }
    }

    return None;
}
