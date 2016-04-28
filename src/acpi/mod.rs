mod rsdp;

pub fn get_rsdp() -> u8 {
    if let Some(rsdpdesc) = rsdp::load_rsdp() {
        return rsdpdesc.checksum;
    } else {
        return 0x0;
    }
}
