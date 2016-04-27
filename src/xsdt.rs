#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]

use core::cmp::Ordering;
use core::mem;
use core::ptr;


#[repr(C)]
pub struct RSDPdesc {
    signature:  [u8; 8],
    checksum:   u8,
    oemid:      [u8; 6],
    revision:   u8,
    rsdt_addr:  u32,
}


impl RSDPdesc {
    pub fn new() -> RSDPdesc {
        unimplemented!();
    }

}

/// Find RSD Pointer
/// #Safety
/// Read-only from memory addresses
pub fn find_rsdp() -> usize {
    // Start by getting the ebda start address which is located at 0x40e
    let ebda_start: usize;
    unsafe {
        asm!( "mov rax, 0x40e"
            : "={rax}"(ebda_start) :
            : "{rax}"
            : "intel" );
    }
    // EBDA is max 1 KB  long
    let ebda_end = ebda_start + 0x400;


    let mut current: usize = ebda_start;

    while current <= ebda_end {
        if verify_rsdp(current) {
            return current;
        } else {
            current = current + 0x8;
        }
    }

    println!("RSDP not in EBDA");

    // It can also be located somewhere between the following addresses
    let mbos_start: usize = 0x000E0000;
    let mbos_end:   usize = 0x000FFFFF;
    current = mbos_start;

    while current <= mbos_end {
        if verify_rsdp(current) {
            return current;
        } else {
            current = current + 0x8;
        }
    }

    return 0x0;
}


fn verify_rsdp(rsdp: usize) -> bool {
    // We need an ASCII string containing "RSD PTR"
    let str: [u8; 8] = [82, 83, 68, 32, 80, 84, 82, 32];
    let mut cmpa: usize = rsdp;


    for char in &str {
        let cmpc: u8;
        unsafe {
            asm!( "mov rax, [rcx]"
                : "={rax}"(cmpc)
                : "{rcx}"(cmpa)
                : "{rax}","{rcx}"
                : "intel" );
        }

        if cmpc.cmp(char) != Ordering::Equal {
            return false;
        }


        cmpa = cmpa + 0x1;
    }


    // TODO: Implement checksum verification
    let rsdp_desc = gen_rsdp_desc(rsdp);
    //println!("{:x}",rsdp_desc.checksum);

    return true;
}


fn gen_rsdp_desc<'a>(rsdpp: usize) -> &'a *const RSDPdesc {
    //let mut current: usize = rsdpp;

    //let signature:  *const [u8; 8];
    //let checksum:   *const u8;
    //let oemid:      *const [u8; 6];
    //let revision:   *const u8;
    //let rsdt_addr:  *const u32;
    let ref rsdp_desc: *const RSDPdesc;

    unsafe {
        //rsdp_desc = mem::uninitialized();
        asm!( "mov rax, [rcx]"
            : "={rax}"(rsdp_desc)
            : "{rcx}"(rsdpp)
            : "{rax}","{rcx}"
            : "intel" );
    }

    //let rsdp_desc = RSDPdesc {
    //    signature:  *current,
    //    checksum:   *(current = current + 0x40),
    //};

    println!("{:x}", rsdp_desc as *const RSDPdesc);
    //println!("{:X}", rsdp_desc.checksum);

    return rsdp_desc;
}
