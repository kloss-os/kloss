use self::acpi_sdt::ACPISDTHeader;

mod acpi_sdt;
mod rsdp;


pub struct RSDT {
    header:     ACPISDTHeader,
    next_rsdt:  *const RSDT,
}


unsafe fn load_rsdt_addr(rsdt_addr: usize) -> Option<&'static RSDT> {
    let rsdt = &*(rsdt_addr as *const RSDT);
    //if verify_acpisdth(&rsdt.header) {
    if sum_bytes(rsdt_addr, rsdt.header.length as usize) == 0 {
        return Some(rsdt);
    } else {
        return None;
    }
}

pub fn load_rsdt_root() -> Option<&'static RSDT> {
    if let Some(rsdp) = rsdp::load_rsdp() {
        if let Some(rsdt_root) = unsafe{load_rsdt_addr(rsdp.rsdt_addr as usize)} {
            return Some(rsdt_root);
        } else { return None; }
    } else {
        return None;
    }
}


pub fn get_rsdt() -> u8 {
    if let Some(rsdt) = load_rsdt_root() {
        println!("Loaded root rsdt!");
        return 0x1;
    } else {
        println!("Didn't root rsdt!");
        return 0x0;
    }
}


unsafe fn sum_bytes(start: usize, len: usize) -> u8 {
    let mut sum: u32 = 0;

    for i in start..(start + len) {
        let current: u32 = *(i as *const u32);
        sum = (sum + current) & 0xFF;
    }

    return sum as u8;
}
