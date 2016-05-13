
//MSR registers that we could be interested in
// Time stamp counter value
const TSC : u32 = 0x0000_0010;
// Time stamp counter adjustment
const TSM_ADJUST : u32 = 0x0000_003B;
// Prosessor ID value
const TSC_AUX : u32 = 0xC000_0103; 
// Loacal APIC
pub const APIC_BASE : u32 = 0x0000_001B;




/// Write MSR
/// # Examples 
/// `unsafe{write_msr(APIC_BASE, 0x0, 0xfec0_0900)}` 
/// # Safety 
/// - Does not check if input is a valid address.
/// - Does no check CPUID if MSR is supported.
/// - Does not check if targets has reserved bits.
pub unsafe fn write_msr(msr: u32, high: u32, low: u32) {
        asm!("wrmsr"
        :
        : "{ecx}"(msr), "{edx}"(high), "{eax}"(low)
        : "{ecx}","{edx}","{eax}"
        : "intel" );
}

/// Reads MSR
/// # Examples 
/// `let apic_msr_data = unsafe{read_msr(APIC_BASE)}` 
/// # Safety 
/// - Does not check if input is a valid address.
/// - Does no check CPUID if MSR is supported.
pub unsafe fn read_msr(msr: u32) -> u64 {
    let high: u32;
    let low: u32;
    asm!("rdmsr"
        : "={edx}"(high), "={eax}"(low)
        : "{ecx}"(msr)
        : "{ecx}","{edx}","{eax}"
        : "intel" );
    (((high as u64) << 32) | (low as u64)) as u64
}




