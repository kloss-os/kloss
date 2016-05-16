//! #ACPI Module
//! _Advanced Configuration and Power Interface_
//!
//! Contains functions that lets you load _System Descriptor Tables_, which in turn let you access
//! cool stuff. These include:
//!
//! + I/O things via IOAPIC (which is fancier than the old-school 8259PIC)
//! + SMT (multicore processing)
//! + A real-time clock
//! + Power button


use core::mem;

mod sdt;
mod rsdp;
pub mod apic;

/// Struct that contains addresses given by SDT, these can be memory mapped using an iterator
pub struct SDT_Loc {
    // Iterator addresses:
    cur_start: usize,
    cur_end: usize,

    rsdt_next: *const u32,
    rsdt_end: usize,

    // Special addresses:
    /// LAPIC controller address
    pub lapic_ctrl: usize,
    /// Starting address for IOAPIC
    pub ioapic_start:   usize,
    /// Ending address for IOAPIC
    pub ioapic_end: usize,
}

pub fn sdt_loc_new() -> SDT_Loc {
    SDT_Loc {
        cur_start: 0,
        cur_end: 0,
        rsdt_next: 0 as *const u32,
        rsdt_end: 0,
        lapic_ctrl: 0,
        ioapic_start: 0,
        ioapic_end: 0,
    }
}


impl SDT_Loc {
    pub fn sdt_loc_load(&mut self, rsdt: &'static sdt::RSDT) {
        self.cur_start = rsdt as *const _ as usize;
        self.cur_end = rsdt as *const _ as usize + rsdt.header.length as usize;
        self.rsdt_next = &rsdt.first_ptr;
        self.rsdt_end = rsdt as *const _ as usize + rsdt.header.length as usize;
        self.lapic_ctrl= 0;
        self.ioapic_start= 0;
        self.ioapic_end= 0;
    }
}

impl Iterator for SDT_Loc {
    type Item = (usize, usize, usize);

    // TODO: Ignore header for starting address
    fn next(&mut self) -> Option<(usize, usize, usize)> {
        if self.cur_start != 0 && self.cur_end != 0 {
            let current_next = self.rsdt_next as usize;
            let current = (self.cur_start, self.cur_end, current_next);

            // Set next cur_start and cur_end
            if self.rsdt_next as *const _ as usize != 0
                && unsafe { sdt::verify_struct(*self.rsdt_next as usize) }  {
                let next_header = unsafe { sdt::load_acpisdt_header(*self.rsdt_next as usize) };

                // Add special addresses in case we find the sdt::MADT
                if sdt::find_type(next_header) == Some(sdt::SDTtype::MADT) {
                    let madt = unsafe { &*(*self.rsdt_next as *const sdt::MADT) };

                    self.lapic_ctrl = madt.local_ctrl as usize;

                    if let Some(ioapic) = unsafe { apic::load_ioapic_entry(&madt) } {
                        self.ioapic_start = ioapic.address as usize;
                        // Address with offset 0x10 must be accessible as a 32-bit registers, but
                        // these margins shouldn't hurt anyone
                        self.ioapic_end = ioapic.address as usize + 0x20;
                    }
                }

                self.cur_start = next_header as *const _ as usize;
                self.cur_end = self.cur_start + next_header.length as usize;
            } else {
                self.cur_start = 0;
                self.cur_end = 0;
            }


            // Set  rsdt_next, if we haven't exhausted sdt::RSDT
            if self.rsdt_next as *const _ as usize != 0
                && (self.rsdt_next as *const _ as usize) < self.rsdt_end {
                self.rsdt_next = unsafe {self.rsdt_next.offset(1)};

            // Otherwise, sdt::RSDT parsing is done!
            } else {
                self.rsdt_next = 0 as *const u32;
            }


            // Return span of current SDT
            Some(current)
        } else {
            None
        }


    }
}





/// Loads an RSDT, scary stuff!
pub fn get_rsdt() -> Option<&'static sdt::RSDT> {
    if let Some(rsdp) = rsdp::load_rsdp() {
        if let Some(rsdt_root) = unsafe{ sdt::load_rsdt(rsdp.rsdt_addr as usize)} {
            return Some(rsdt_root);
        } else { return None; }
    } else {
        return None;
    }
}

/// If available, returns an IOAPIC address
pub fn get_ioapic_addr(rsdt: &'static sdt::RSDT) -> Option<u32> {
    if let Some(madt) = sdt::load_madt(rsdt) {
        if let Some(ioapic) = apic::load_ioapic_entry(madt) {
            return Some(ioapic.address);
        }
    }

    None
}






/// Print MADT table
/// debug function, should be removed for end product
pub unsafe fn print_madt(madt: &'static sdt::MADT) {
    println!("sdt::MADT data:");
    println!("Length: 0x{:x}", madt.header.length);
    println!("LCA: 0x{:x}", madt.local_ctrl);
    println!("Flags: 0x{:x}", madt.flags);

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
        let cur_head = &*(cur_addr as *const apic::IntCtrlHeaderEntry);

        if cur_head.entry_type == 0b00 {
            let lapic = &*(cur_addr as *const apic::LAPICEntry);
            println!("LAPIC PID: {:x}, ID {:x}, FLAGS {:x}",
                     lapic.acpi_proc_id, lapic.apic_id, lapic.flags);
        }

        i += cur_head.record_length as u32;
    }
}

/// This is mainly for testing, should delete once we're sure of things
pub fn test_rsdt(rsdt: &'static sdt::RSDT) {


    if let Some(madt) = unsafe { sdt::load_madt(rsdt) } {
        println!("Loaded MADT");
        unsafe { print_madt(&madt); }

        if let Some(ioapic) = unsafe { apic::load_ioapic_entry(&madt) } {
            println!("Loaded I/O APIC!");
            println!("ID: {:x}, Address: {:x}, Reserved {:x}, GSIB: {:x}",
                    ioapic.id, ioapic.address, ioapic.reserved, ioapic.gsib);

        } else {
            println!("Not loaded ioapic D:");
        }
    } else {
        println!("Not loaded madt!");
    }

}
