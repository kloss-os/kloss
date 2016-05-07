//! A module for using `cpuid`.
//!
//! The `cpuid` module allows for easy access to the CPUID command of the x86/x86_64 CPU
#![allow(dead_code)]

// RESOURCES:
// http://sandpile.org/x86/cpuid.htm
// PDF:s in google drive folder (simon)

// CONSTANTS FOR USING CPUID
// -- BASIC

/// Maximum supported basic option and vendor ID string
const BASIC_INFO      : u32 = 0x0000_0000;

/// Processor type/family/model/stepping and feature flags
const BASIC_FMS_FLAGS : u32 = 0x0000_0001;

/// Cache configuration descriptors (v1)
const BASIC_CACHES_V1 : u32 = 0x0000_0002;

/// Pricessor Serial Number
const BASIC_PSN       : u32 = 0x0000_0003;

/// Cache configuration descriptors (v2)
const BASIC_CACHES_V2 : u32 = 0x0000_0004;

/// Monitor information
const BASIC_MON       : u32 = 0x0000_0005;

/// Power management information
const BASIC_PWR_MGMT  : u32 = 0x0000_0006;

/// Feature flags
const BASIC_FLAGS     : u32 = 0x0000_0007;

/// Reserved instruction
const BASIC_RES1      : u32 = 0x0000_0008; // Reserved

/// Direct Cache Access parameters
const BASIC_DCA       : u32 = 0x0000_0009;

/// Architectural Performance Monitor information
const BASIC_PEMO      : u32 = 0x0000_000a;

/// Processor Topology information
const BASIC_TOPOLOGY  : u32 = 0x0000_000b;

/// Reserved instruction
const BASIC_RES2      : u32 = 0x0000_000c; // Reserved

/// Extended State enumeration features
const BASIC_X_STATE   : u32 = 0x0000_000d;

/// Reserved instruction
const BASIC_RES3      : u32 = 0x0000_000e; // Reserved

/// Platform Quality of service Monitoring (PQM) enumeration
const BASIC_PQM       : u32 = 0x0000_000f;

/// Platform Quality of service Enforcement (PQE) enumeration
const BASIC_PQE       : u32 = 0x0000_0010;

/// Reserved instruction
const BASIC_RES4      : u32 = 0x0000_0011; // Reserved

/// SGX resource enumeration
const BASIC_SGX       : u32 = 0x0000_0012;

/// Reserved instruction
const BASIC_RES5      : u32 = 0x0000_0013; // Reserved

/// Processor Trace (PT) capability enumeration
const BASIC_PT        : u32 = 0x0000_0014;

/// Processor frequency information 1/2
const BASIC_FREQ1     : u32 = 0x0000_0015;

/// Processor frequency information 2/2
const BASIC_FREQ2     : u32 = 0x0000_0016;

/// Processor vendor attribute information
const BASIC_ATTR      : u32 = 0x0000_0017;


// -- XENON PHI 

/// Maximum supported option
const XENON_PHI_MAX   : u32 = 0x2000_0000;

/// Xenon Phi Processor flag(s)
const XENON_PHI_FLAGS : u32 = 0x2000_0001;

/// Reserved instruction
const XENON_PHI_RES1  : u32 = 0x2000_0002; // Reserved

/// Reserved instruction
const XENON_PHI_RES2  : u32 = 0x2000_0003; // Reserved

/// Reserved instruction
const XENON_PHI_RES3  : u32 = 0x2000_0004; // Reserved

/// Reserved instruction
const XENON_PHI_RES4  : u32 = 0x2000_0005; // Reserved

/// Reserved instruction
const XENON_PHI_RES5  : u32 = 0x2000_0006; // Reserved

/// Reserved instruction
const XENON_PHI_RES6  : u32 = 0x2000_0007; // Reserved


// -- HYPERVISOR
// Virtual processors etc. (it seems)

/// Vendor information
const HYPERVISOR_VENDOR    : u32 = 0x4000_0000;

/// Hypervisor interface
const HYPERVISOR_INTERFACE : u32 = 0x4000_0001;

/// Hypervisor version
const HYPERVISOR_VERSION   : u32 = 0x4000_0002;

/// Hypervisor feature flags
const HYPERVISOR_FEATURES  : u32 = 0x4000_0003;

/// Hypervisor recommendations
const HYPERVISOR_RECOMM    : u32 = 0x4000_0004;

/// Hypervisor limit information
const HYPERVISOR_LIMITS    : u32 = 0x4000_0005;

/// Hypervisor hardware features detected and in use
const HYPERVISOR_HARDWARE  : u32 = 0x4000_0006;

/// Reserved instruction
const HYPERVISOR_RES1      : u32 = 0x4000_0007; // Reserved


// -- EXTENDED

/// Maximum supported basic option and vendor ID
const EXT_INFO                : u32 = 0x8000_0000;

/// Processor family/model/stepping and feature flags
const EXT_FMS_FLAGS           : u32 = 0x8000_0001;

/// Processor name string (part 1/3)
const EXT_PROCESSOR_NAME_STR1 : u32 = 0x8000_0002;

/// Processor name string (part 2/3)
const EXT_PROCESSOR_NAME_STR2 : u32 = 0x8000_0003;

/// Processor name string (part 3/3)
const EXT_PROCESSOR_NAME_STR3 : u32 = 0x8000_0004;

/// L1 Cache and L1 TLB configuration descriptors
const EXT_L1CACHE_V1          : u32 = 0x8000_0005;

/// L2/L3 cache and L2 TLB configuration descriptors
const EXT_L2L3_CACHE_V1       : u32 = 0x8000_0006;

/// Processor capabilities
const EXT_CAPABILITIES        : u32 = 0x8000_0007;

/// Addewss size information and misc. information
const EXT_ADDR_MISC           : u32 = 0x8000_0008;

/// Reserved instruction
const EXT_RES1                : u32 = 0x8000_0009;

/// Shared Virtual Memory information
const EXT_SVM                 : u32 = 0x8000_000a;

/// Reserved instruction
const EXT_RES2                : u32 = 0x8000_000b;

/// Reserved instruction
const EXT_RES3                : u32 = 0x8000_000c;

/// Reserved instruction
const EXT_RES4                : u32 = 0x8000_000d;

/// Reserved instruction
const EXT_RES5                : u32 = 0x8000_000e;

/// Reserved instruction
const EXT_RES6                : u32 = 0x8000_000f;

/// Reserved instruction
const EXT_RES7                : u32 = 0x8000_0010;

/// Reserved instruction
const EXT_RES8                : u32 = 0x8000_0011;

/// Reserved instruction
const EXT_RES9                : u32 = 0x8000_0012;

/// Reserved instruction
const EXT_RES10               : u32 = 0x8000_0013;

/// Reserved instruction
const EXT_RES11               : u32 = 0x8000_0014;

/// Reserved instruction
const EXT_RES12               : u32 = 0x8000_0015;

/// Reserved instruction
const EXT_RES13               : u32 = 0x8000_0016;

/// Reserved instruction
const EXT_RES14               : u32 = 0x8000_0017;

/// Reserved instruction
const EXT_RES15               : u32 = 0x8000_0018;

/// TLB Configuration descriptors
const EXT_1G_TLB              : u32 = 0x8000_0019;

/// Performance optimization identifiers
const EXT_PERF_HINTS          : u32 = 0x8000_001a;

/// Instruction Based Sampling (IBS) information
const EXT_IBS                 : u32 = 0x8000_001b;

/// Light Weight Profiling (LWP) information
const EXT_LWP                 : u32 = 0x8000_001c;

/// Cache configuration descriptors (v2)
const EXT_CACHES_V2           : u32 = 0x8000_001d;

/// Topology information: APIC/unit/node information
const EXT_TOPOLOGY            : u32 = 0x8000_001e;

/// SME/SEV information
const EXT_SME_SEV             : u32 = 0x8000_001f;


// -- TRANSMETA

/// Maximum supported transmeta option and vendor ID string
const TRANSMETA_INFO        : u32 = 0x8086_0000;

/// Processor family/model/stepping and feature flags
const TRANSMETA_FMS_FLAGS   : u32 = 0x8086_0001;

/// Hardware/Software revision
const TRANSMETA_HW_SW_REV   : u32 = 0x8086_0002;

/// Transmeta information string (part 1/4)
const TRANSMETA_CMS_STRING1 : u32 = 0x8086_0003;

/// Transmeta information string (part 2/4)
const TRANSMETA_CMS_STRING2 : u32 = 0x8086_0004;

/// Transmeta information string (part 3/4)
const TRANSMETA_CMS_STRING3 : u32 = 0x8086_0005;

/// Transmeta information string (part 4/4)
const TRANSMETA_CMS_STRING4 : u32 = 0x8086_0006;

/// Processor current core clock frequency (MHz), current core clock voltage (mV),
/// current (LongRun) performance level [0-100%] and current gate delay (fs) 
const TRANSMETA_MHZ_MV      : u32 = 0x8086_0007;


// -- CENTAUR

// Maximum supported option
const CENTAUR_INFO      : u32 = 0xC000_0000;

/// Processor information and flags
const CENTAUR_FMS_FLAGS : u32 = 0xC000_0001;

/// Reserved instruction
const CENTAUR_RES1      : u32 = 0xC000_0002;

/// Reserved instruction
const CENTAUR_RES2      : u32 = 0xC000_0003;

/// Reserved instruction
const CENTAUR_RES3      : u32 = 0xC000_0004;

/// Reserved instruction
const CENTAUR_RES4      : u32 = 0xC000_0005;

/// Reserved instruction
const CENTAUR_RES5      : u32 = 0xC000_0006;

/// Reserved instruction
const CENTAUR_RES6      : u32 = 0xC000_0007;


// EOF CONSTANTS FOR USING CPUID 


// BITMASKS
// -- Generic
/// Mask lower byte
const MASK_BYTE : u32 = 0xff;

/// Mask half byte
const MASK_HALF_BYTE : u32 = 0xf;

// Struct for listing CPU flags
pub struct CPUFlags {
    // ECX Register
    pub hv: bool,
    pub rdrand: bool,
    pub f16c: bool,
    pub avx: bool,
    pub osxsave: bool,
    pub xsave: bool,
    pub aes: bool,
    pub tscd: bool,
    pub popcnt: bool,
    pub movbe: bool,
    pub x2apic: bool,
    pub sse4_2: bool,
    pub sse4_1: bool,
    pub dca: bool,
    pub pcid: bool,
    // Bit 16 reserved
    pub pdcm: bool,
    pub etprd: bool,
    pub cx16: bool,
    pub fma: bool,
    pub sdbg: bool,
    pub cid: bool,
    pub ssse3: bool,
    pub tm2: bool,
    pub est: bool,
    pub smx: bool,
    pub vmx: bool,
    pub dscpl: bool,
    pub mon: bool,
    pub dtes64: bool,
    pub pclmul: bool,
    pub sse3: bool,
    
    // EDX register
    pub pbe: bool,
    pub ia_64: bool,
    pub tm1: bool,
    pub htt: bool,
    pub ss: bool,
    pub sse2: bool,
    pub sse: bool,
    pub fxsr: bool,
    pub mmx: bool,
    pub acpi: bool,
    pub dtes: bool,
    // Bit 20 reserved
    pub clfl: bool,
    pub psn: bool,
    pub pse36: bool,
    pub pat: bool,
    pub cmov: bool,
    pub mca: bool,
    pub pge: bool,
    pub mtrr: bool,
    pub sep: bool,
    // Bit 10 reserved
    pub apic: bool,
    pub cx8: bool,
    pub mce: bool,
    pub pae: bool,
    pub msr: bool,
    pub tsc: bool,
    pub pse: bool,
    pub de: bool,
    pub vme: bool,
    pub fpu: bool
}

/// Struct for processor type/family/model/stepping
pub struct CPUModel {
    pub cpu_type: u8,
    pub family:   u8,
    pub model:    u16,
    pub brand:    u8,
    pub stepping: u8,
    pub cpu_cnt:  u8,
    pub apic_id:  u8,
    pub clflush:  u8

}

/// Struct for keeping tab on highest available
/// option for current CPU.
pub struct CPUID {
    /// Highest option available in call to Basic CPUID
    basic_highest_option: u32,
    /// Highest option available in call to Extended CPUID
    ext_highest_option: u32
}

impl CPUID {
    /// Create new CPUID and detect
    /// highest available option
    ///
    /// # Examples
    ///
    /// ```
    ///  let cpuid = CPUID::new()
    /// ```
    pub fn new() -> CPUID {
        unsafe {
            let (base, _, _, _) = call_cpuid(BASIC_INFO);
            let (ext, _, _, _) = call_cpuid(EXT_INFO); // TODO: Change to EXT-INFO after fixing get-function

            return CPUID {basic_highest_option: base, ext_highest_option: ext};
        }
    }

    // BASIC OPTIONS

    /// Max supported basic option and vendor ID string
    pub fn basicInfo(&self) -> (u32, [char;12]) {
        // Call CPUID
        let resp;
        unsafe {
            resp = call_cpuid(BASIC_INFO);
        }
        
        // Extract info
        let (a,b,c,d) = resp;
        let mut id : [char;12] = ['\0';12];

        // Correctly arrange letters of vendor ID string
        // as they are ordered ebx -> edx -> ecx with
        // letters arranged LSB to MSB (eg. "h t u A" is
        // actually "A u t h").
        id[0]  = (b & MASK_BYTE) as u8 as char;
        id[1]  = ((b >> 8) & MASK_BYTE) as u8 as char;
        id[2]  = ((b >> 16) & MASK_BYTE) as u8 as char;
        id[3]  = ((b >> 24) & MASK_BYTE) as u8 as char;

        id[4]  = (d & MASK_BYTE) as u8 as char;
        id[5]  = ((d >> 8) & MASK_BYTE) as u8 as char;
        id[6]  = ((d >> 16) & MASK_BYTE) as u8 as char;
        id[7]  = ((d >> 24) & MASK_BYTE) as u8 as char;

        id[8]   = (c & MASK_BYTE) as u8 as char;
        id[9]   = ((c >> 8) & MASK_BYTE) as u8 as char;
        id[10]  = ((c >> 16) & MASK_BYTE) as u8 as char;
        id[11]  = ((c >> 24) & MASK_BYTE) as u8 as char;

        // Return tuple of values
        (a, id)
    }

    /// Get processor family/model/stepping
    /// TODO: verify it works!!!!
    pub fn cpuModel(&self) -> Option<CPUModel> {
        match self.get(BASIC_FMS_FLAGS, self.basic_highest_option) {
            Some((a,b,_,_)) => {
                // Call was OK, filter out values
                Some(CPUModel {
                    cpu_type: ((a >> 12) & MASK_HALF_BYTE) as u8,
                    family: (((a >> 8) & MASK_BYTE) + ((a >> 20) & MASK_BYTE)) as u8,
                    model:  (((a >> 12) & (MASK_BYTE << 4)) | (a >> 4 & MASK_BYTE)) as u16,
                    brand:  (b & MASK_BYTE) as u8,
                    stepping: (a & MASK_HALF_BYTE) as u8,
                    cpu_cnt: ((b >> 16) & MASK_BYTE) as u8,
                    apic_id: ((b >> 24) & MASK_BYTE) as u8,
                    clflush: ((b >> 8) & MASK_BYTE) as u8
                })
            },
            None => {
                None
            }
        }
    }

    /// Get basic flags
    pub fn flags(&self) -> Option<CPUFlags> {
        match self.get(BASIC_FMS_FLAGS, self.basic_highest_option) {
            Some((_,_,c,d)) => {
                // Call was OK, filter out values
                Some(CPUFlags {
                    hv:      (c & (0x1 << 31)) > 0,
                    rdrand:  (c & (0x1 << 30)) > 0,
                    f16c:    (c & (0x1 << 29)) > 0,
                    avx:     (c & (0x1 << 28)) > 0,
                    osxsave: (c & (0x1 << 27)) > 0,
                    xsave:   (c & (0x1 << 26)) > 0,
                    aes:     (c & (0x1 << 25)) > 0,
                    tscd:    (c & (0x1 << 24)) > 0,
                    popcnt:  (c & (0x1 << 23)) > 0,
                    movbe:   (c & (0x1 << 22)) > 0,
                    x2apic:  (c & (0x1 << 21)) > 0,
                    sse4_2:  (c & (0x1 << 20)) > 0,
                    sse4_1:  (c & (0x1 << 19)) > 0,
                    dca:     (c & (0x1 << 18)) > 0,
                    pcid:    (c & (0x1 << 17)) > 0,
                    // Bit 16 reserved
                    pdcm:    (c & (0x1 << 15)) > 0,
                    etprd:   (c & (0x1 << 14)) > 0,
                    cx16:    (c & (0x1 << 13)) > 0,
                    fma:     (c & (0x1 << 12)) > 0,
                    sdbg:    (c & (0x1 << 11)) > 0,
                    cid:     (c & (0x1 << 10)) > 0,
                    ssse3:   (c & (0x1 << 9))  > 0,
                    tm2:     (c & (0x1 << 8))  > 0,
                    est:     (c & (0x1 << 7))  > 0,
                    smx:     (c & (0x1 << 6))  > 0,
                    vmx:     (c & (0x1 << 5))  > 0,
                    dscpl:   (c & (0x1 << 4))  > 0,
                    mon:     (c & (0x1 << 3))  > 0,
                    dtes64:  (c & (0x1 << 2))  > 0,
                    pclmul:  (c & (0x1 << 1))  > 0,
                    sse3:    (c & 0x1)         > 0,
                    
                    // Snd register
                    pbe:   (d & (0x1 << 31)) > 0,
                    ia_64: (d & (0x1 << 30)) > 0,
                    tm1:   (d & (0x1 << 29)) > 0,
                    htt:   (d & (0x1 << 28)) > 0,
                    ss:    (d & (0x1 << 27)) > 0,
                    sse2:  (d & (0x1 << 26)) > 0,
                    sse:   (d & (0x1 << 25)) > 0,
                    fxsr:  (d & (0x1 << 24)) > 0,
                    mmx:   (d & (0x1 << 23)) > 0,
                    acpi:  (d & (0x1 << 22)) > 0,
                    dtes:  (d & (0x1 << 21)) > 0,
                    // Bit 20 reserved
                    clfl:  (d & (0x1 << 19)) > 0,
                    psn:   (d & (0x1 << 18)) > 0,
                    pse36: (d & (0x1 << 17)) > 0,
                    pat:   (d & (0x1 << 16)) > 0,
                    cmov:  (d & (0x1 << 15)) > 0,
                    mca:   (d & (0x1 << 14)) > 0,
                    pge:   (d & (0x1 << 13)) > 0,
                    mtrr:  (d & (0x1 << 12)) > 0,
                    sep:   (d & (0x1 << 11)) > 0,
                    // Bit 10 reserved
                    apic:  (d & (0x1 << 9)) > 0,
                    cx8:   (d & (0x1 << 8)) > 0,
                    mce:   (d & (0x1 << 7)) > 0,
                    pae:   (d & (0x1 << 6)) > 0,
                    msr:   (d & (0x1 << 5)) > 0,
                    tsc:   (d & (0x1 << 4)) > 0,
                    pse:   (d & (0x1 << 3)) > 0,
                    de:    (d & (0x1 << 2)) > 0,
                    vme:   (d & (0x1 << 1)) > 0,
                    fpu:   (d & 0x1)        > 0,
                })
            },
            None => {
                None
            }
        }
        
    }

    /// Call CPUID using supplied option, only
    /// if option does not exceed highest option
    /// available.
    pub fn get(&self, option: u32, max_avail: u32) -> Option<(u32,u32,u32,u32)> {
        if option <= max_avail {
            unsafe {
                Some(call_cpuid(option))
            }
        } else {
            None
        }
    }
}

/// Actual call to CPUID by way of assembly
unsafe fn call_cpuid(option: u32) -> (u32, u32, u32, u32) {
    let eax: u32;
    let ebx: u32;
    let ecx: u32;
    let edx: u32;
    asm!(
        "cpuid"
            : "={eax}"(eax), "={ebx}"(ebx), "={ecx}"(ecx), "={edx}"(edx)
            : "{eax}"(option)
            : "{eax}", "{ebx}","{ecx}","{edx}"
            : "intel"
    );
    (eax, ebx, ecx, edx)
}
