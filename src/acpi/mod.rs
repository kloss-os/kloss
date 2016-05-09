
use self::acpi_header::ACPISDTHeader;
use core::mem;
use core::intrinsics::{volatile_load, volatile_store};

mod acpi_header;
mod rsdp;



/// A struct designating the _Root System Description Table_ (RSDT).
/// This contains pointers to all the other SDT's in the system.
/// Signature is "RSDT"
#[repr(C, packed)]
pub struct RSDT {
    /// The header of the RSDT
    header:     ACPISDTHeader,
    /// This is the first pointer in an array
    /// You can use the first pointer, along with .offset() method to increment by 32-bit boundary.
    /// The length of the array is given by rsdt.header.length - mem::size_of::<ACPISDTHeader>()
    first_ptr:  u32,
}





/// The _Multiple APIC Description Table_ (MADT) describes all interrupt controllers
/// Signature is "APIC"
#[repr(C, packed)]
pub struct MADT {
    /// ACPI SDT Header
    header:     ACPISDTHeader,
    /// Local controller address
    local_ctrl: u32,
    /// Flags
    flags:      u32,

    /// After this, we have a variable length sequence of variable length records.
    /// Using .offset() it will increment the address by 16 bits.
    /// Since _it seems_ the ISO struct is on a 16-bit boundary, this makes sense.
    first_intctr: u8,
}

/// Universal header for interrupt controller description
#[repr(C, packed)]
pub struct IntCtrlHeader_entry {
    /// Describes type of interrupt controller
    entry_type:     u8,
    /// Length of its table
    record_length:  u8,
}

/// _Processor Local APIC_ (LAPIC, packed), indicated by entry type 0
/// Represents a single physical processor and its local interrupt controller
#[repr(C, packed)]
pub struct LAPIC_entry {
    /// Universal header
    header:     IntCtrlHeader_entry,
    /// ACPI Processor ID
    acpi_proc_id: u8,
    /// APIC ID
    apic_id:    u8,
    /// Flags, if 1 it is enabled
    flags:      u32,
}

/// _I/O APIC_ (IOAPIC), indicated by entry type 1
#[repr(C, packed)]
pub struct IOAPIC_entry {
    /// Universal header
    header:   IntCtrlHeader_entry,
    /// I/O APIC's ID
    id:       u8,
    /// Reserved, value is 0 (_padding?_)
    reserved: u8,
    /// I/O APIC's Address
    address:  u32,
    /// Global System Interrupt Base
    gsib:     u32,

}


/// _I/O SAPIC_ (IOSAPIC), indicated by entry type 6
#[repr(C, packed)]
pub struct IOSAPIC_entry {
    /// Universal header
    header:   IntCtrlHeader_entry,
    /// I/O APIC's ID
    id:       u8,
    /// Reserved, value is 0 (_padding?_)
    reserved: u8,
    /// I/O APIC's Address
    address:  u32,
    /// Global System Interrupt Base
    gsib:     u64,

}

/// _Interrupt Source Override_, indicated by entry type 2
#[repr(C, packed)]
pub struct ISO_entry {
    /// Universal header
    header:     IntCtrlHeader_entry,
    /// Bus Source
    bus_source: u8,
    /// IRQ Source
    irq_source: u8,
    /// Global System Interrupt
    global_sys_int: u32,
    /// Flags
    flags:      u16,
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
        if let Some(rsdt_root) = unsafe{load_rsdt_addr(rsdp.rsdt_addr as usize)} {
            return Some(rsdt_root);
        } else { return None; }
    } else {
        return None;
    }
}


/// Finds a table of a certain type in an rsdt
pub unsafe fn load_madt(rsdt: &'static RSDT) -> Option<&'static MADT> {
    // Make a raw pointer for the first SDT pointer
    let first: *const u32 = &rsdt.first_ptr;
    // Calculate amount of pointers
    let num_ptr = (rsdt.header.length
                   - mem::size_of::<ACPISDTHeader>() as u32)
                    / 4;

    // Step through the list until desired table is found
    for i in 0..num_ptr {
        let current = first.offset(i as isize);
        if  acpi_header::verify_struct(*current as usize) &&
            acpi_header::find_type(
                acpi_header::load_acpisdt_header(
                    *current as usize))
            == Some(acpi_header::SDTtype::MADT)
        {
            return Some(&*(*current as *const MADT));
        }
    }

    return None;
}


/// Get the I/O APIC
pub unsafe fn load_ioapic_entry(madt: &'static MADT) -> Option<&'static IOAPIC_entry> {
    // Get first byte address
    let start: *const u8 = &madt.first_intctr;

    // Get amount of bytes in bound
    let num_addr = madt.header.length
                    - mem::size_of::<ACPISDTHeader>() as u32
                    - (4 + 4); // Remove size of descriptors


    // Iterate over bound
    let mut i = 0;
    while i < num_addr {
        let cur_addr = start.offset(i as isize);
        let cur_head = &*(cur_addr as *const IntCtrlHeader_entry);

        if cur_head.entry_type == 0b01 {
            return Some( &*(cur_addr as *const IOAPIC_entry) );
        } else {
            i += cur_head.record_length as u32;
        }
    }

    return None;
}

/// Get the I/O SAPIC
pub unsafe fn load_iosapic_entry(madt: &'static MADT) -> Option<&'static IOSAPIC_entry> {
    // Get first byte address
    let start: *const u8 = &madt.first_intctr;

    // Get amount of bytes in bound
    let num_addr = madt.header.length
                    - mem::size_of::<ACPISDTHeader>() as u32
                    - (4 + 4); // Remove size of descriptors


    // Iterate over bound
    let mut i = 0;
    while i < num_addr {
        let cur_addr = start.offset(i as isize);
        let cur_head = &*(cur_addr as *const IntCtrlHeader_entry);

        if cur_head.entry_type == 0x06 {
            return Some( &*(*cur_addr as *const IOSAPIC_entry) );
        } else {
            i += cur_head.record_length as u32;
        }
    }

    return None;
}

/// Print MADT table
pub unsafe fn print_madt(madt: &'static MADT) {
    println!("MADT data:");
    println!("Length: 0x{:x}", madt.header.length);
    println!("LCA: 0x{:x}", madt.local_ctrl);
    println!("Flags: 0x{:x}", madt.flags);

    // Get first byte address
    let start: *const u8 = &madt.first_intctr;

    // Get amount of bytes in bound
    let num_addr = madt.header.length
                    - mem::size_of::<ACPISDTHeader>() as u32
                    - (4 + 4); // Remove size of descriptors


    // Iterate over bound
    let mut i = 0;
    while i < num_addr {
        let cur_addr = start.offset(i as isize);
        let cur_head = &*(cur_addr as *const IntCtrlHeader_entry);

        println!("Type: 0x{:x}, Len: 0x{:x} at address 0x{:x}",
                 cur_head.entry_type as u32,
                 cur_head.record_length as u32,
                 cur_addr as u32);

        i += cur_head.record_length as u32;
    }
}


/// Writes high:low into the specified msr
unsafe fn write_msr(msr: u32, high: u32, low: u32) {
    asm!("wrmsr"
        :
        : "{ecx}"(msr), "{edx}"(high), "{eax}"(low)
        : "{ecx}","{edx}","{eax}"
        : "intel" );
}


/// Reads MSR
unsafe fn read_msr(msr: u32) -> u64 {
    let high: u32;
    let low: u32;
    asm!("rdmsr"
        : "={edx}"(high), "={eax}"(low)
        : "{ecx}"(msr)
        : "{ecx}","{edx}","{eax}"
        : "intel" );
    (((high as u64) << 32) | (low as u64)) as u64
}

unsafe fn set_ioapic(new_addr: usize) {
    // Account for PAE
    let high: u32 = ((new_addr >> 32) & 0x0f) as u32;
    // Enable base MSR
    let low: u32 = ((new_addr & 0xFFFFF100) | 0x800) as u32;
    // Set MSR
    let msr: u32 = 0x1b;
    write_msr(msr, high, low);
}


unsafe fn read_ioapic(ioapicaddr: *mut u32, reg: u32) -> u32 {
    // Set IOWIN address
    let iowin = (ioapicaddr as *const u32).offset(4);

    // Write the selected register to IOREGSEL
    //volatile_store(ioapicaddr, reg);

    //volatile_load( iowin )

    *ioapicaddr = reg;
    *iowin
}





unsafe fn check_cpuid() {
    let cpuid_input: u32 = 0x1;
    let mut cpuid_output: u32;
    asm!("cpuid"
         : "={ecx}"(cpuid_output)
         : "{eax}"(cpuid_input)
         : "{eax}","{ecx}"
         : "intel" );

    println!("CPUID: 0x{:x}", cpuid_output);

}


unsafe fn mask_pic_irq() {
    let pic1_data: u16 = 0x21;
    let pic2_data: u16 = 0xA1;

    for i in 0..8 {
        let current_pic = if i > 8 { pic1_data } else { pic2_data };
        let current_irq = i % 8;
        let value: u8;
        asm!("in al, dx"
             : "={al}"(value)
             : "{dx}"(current_pic)
             : "{al}","{dx}"
             : "intel" );

        let masked = value | (1 << current_irq);
        asm!("out dx, al"
             :
             : "{dx}"(current_pic),"{al}"(value)
             : "{al}","{dx}"
             : "intel" );
    }
}

unsafe fn remap_pic() {
    // https://en.wikibooks.org/wiki/X86_Assembly/Programmable_Interrupt_Controller#Remapping

    asm!("mov al, 0x11\n\t\
          out 0x20, al\n\t\
          out 0xA0, al\n\t\
          mov al, 0x20\n\t\
          out 0x21, al\n\t\
          mov al, 0x28\n\t\
          out 0xA1, al\n\t\
          mov al, 0x04\n\t\
          out 0x21, al\n\t\
          mov al, 0x02\n\t\
          out 0xA1, al\n\t\
          mov al, 0x01\n\t\
          out 0x21, al\n\t\
          out 0xA1, al"
        :
        :
        : "{al}"
        : "intel"
        )
}




/// This is mainly for testing, should delete once we're sure of things
pub fn get_rsdt() -> u8 {

    unsafe { mask_pic_irq(); }
    unsafe { remap_pic(); }
    unsafe { set_ioapic(0xFEC04400); }

    if let Some(rsdt) = load_rsdt() {
        println!("Loaded RSDT, length is 0x{:x}!", rsdt.header.length);


        if let Some(madt) = unsafe { load_madt(&rsdt) } {
            println!("Loaded MADT");
            unsafe { print_madt(&madt); }

            if let Some(ioapic) = unsafe { load_ioapic_entry(&madt) } {
                println!("Loaded I/O APIC!");
                println!("ID: {:x}, Address: {:x}, Reserved {:x}, GSIB: {:x}",
                        ioapic.id, ioapic.address, ioapic.reserved, ioapic.gsib);

                unsafe { check_cpuid(); }
                //println!("IOAPIC contains 0x{:x}", unsafe { read_ioapic(ioapic.address as *mut u32, 0xF0) });
                println!("MSR 0x1b is 0x{:x}", unsafe{read_msr(0x1b)});
            } else {
                println!("Not loaded ioapic D:");
            }
        } else {
            println!("Not loaded madt!");
        }

        return 0x1;
    } else {
        println!("Didn't load rsdt!");
        return 0x0;
    }
}
