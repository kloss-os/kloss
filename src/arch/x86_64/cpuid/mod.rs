//! A module for using `cpuid`.
//!
//! The `cpuid` module allows for easy access to the CPUID command of the x86/x86_64 CPU
#![allow(dead_code)]

// RESOURCES:
// http://sandpile.org/x86/cpuid.htm
// PDF:s in google drive folder (simon)

// Sub-modules
mod features;

// Used modules
use self::features::{Features};

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
// Only xenon phi processors

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
// Virtual machines/processors like VMWare, Virtualbox, etc.

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
// Only for Transmeta Processors

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
// Only for Centaur Processors

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

// BIT MASKS
const MASK_BIT_0  : u32 = 0x1;
const MASK_BIT_1  : u32 = 0x1 << 1;
const MASK_BIT_2  : u32 = 0x1 << 2;
const MASK_BIT_3  : u32 = 0x1 << 3;
const MASK_BIT_4  : u32 = 0x1 << 4;
const MASK_BIT_5  : u32 = 0x1 << 5;
const MASK_BIT_6  : u32 = 0x1 << 6;
const MASK_BIT_7  : u32 = 0x1 << 7;
const MASK_BIT_8  : u32 = 0x1 << 8;
const MASK_BIT_9  : u32 = 0x1 << 9;
const MASK_BIT_10 : u32 = 0x1 << 10;
const MASK_BIT_11 : u32 = 0x1 << 11;
const MASK_BIT_12 : u32 = 0x1 << 12;
const MASK_BIT_13 : u32 = 0x1 << 13;
const MASK_BIT_14 : u32 = 0x1 << 14;
const MASK_BIT_15 : u32 = 0x1 << 15;
const MASK_BIT_16 : u32 = 0x1 << 16;
const MASK_BIT_17 : u32 = 0x1 << 17;
const MASK_BIT_18 : u32 = 0x1 << 18;
const MASK_BIT_19 : u32 = 0x1 << 19;
const MASK_BIT_20 : u32 = 0x1 << 20;
const MASK_BIT_21 : u32 = 0x1 << 21;
const MASK_BIT_22 : u32 = 0x1 << 22;
const MASK_BIT_23 : u32 = 0x1 << 23;
const MASK_BIT_24 : u32 = 0x1 << 24;
const MASK_BIT_25 : u32 = 0x1 << 25;
const MASK_BIT_26 : u32 = 0x1 << 26;
const MASK_BIT_27 : u32 = 0x1 << 27;
const MASK_BIT_28 : u32 = 0x1 << 28;
const MASK_BIT_29 : u32 = 0x1 << 29;
const MASK_BIT_30 : u32 = 0x1 << 30;
const MASK_BIT_31 : u32 = 0x1 << 31;


/// Struct for processor type/family/model/stepping
pub struct CPUModel {
    /// Processor type (2-bit encoded)
    ///
    /// 0 - Primary processor,
    /// 1 - Overdrive processor,
    /// 2 - Secondary processor (for MP),
    /// 3 - reserved
    pub cpu_type: u8,

    /// Processor family
    pub family:   u8,

    /// Procssor model
    pub model:    u16,

    /// Brand ID
    // TODO: Use extended level for AMD
    pub brand:    u8,
    
    /// Processor-specific strpping values
    pub stepping: u8,
    
    /// Logical processor count
    ///
    /// Oddly numbered:
    /// 0 - 1 core
    /// 1 - 1 core (probably)
    /// 2 - 2 cores
    /// ...
    /// n = n cores
    pub cpu_cnt:  u8,

    /// The (fixed) default APIC ID
    pub apic_id:  u8,

    /// CLFLUSH (8-byte) Chunk count
    pub clflush:  u8

}


/// Struct/Class for using CPUID
pub struct CPUID {
    /// Highest option available in call to Basic CPUID
    basic_limit: u32,
    /// Highest option available in call to Xenon Phi CPUID
    xenon_phi_limit: u32,
    /// Highest option available in call to Hypervisor CPUID
    hypervisor_limit: u32,
    /// Highest option available in call to Extended CPUID
    ext_limit: u32,
    /// Highest option available in call to Transmeta CPUID
    transmeta_limit: u32,
    /// Highest option available in call to Centaur CPUID
    centaur_limit: u32
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
            // Seems to return 0x0 if not present, however
            // the response is flagged as `unknown` for
            // options not available on CPU.
            //
            // TODO: Verify 0x0 returned for non-existing
            //       option-levels.
            let (base, _, _, _) = call_cpuid(BASIC_INFO);
            let (xenon, _, _, _) = call_cpuid(XENON_PHI_MAX);
            let (hyper, _, _, _) = call_cpuid(HYPERVISOR_VENDOR);
            let (ext, _, _, _) = call_cpuid(EXT_INFO);
            let (trans, _, _, _) = call_cpuid(TRANSMETA_INFO);
            let (centaur, _, _, _) = call_cpuid(CENTAUR_INFO);

            return CPUID {
                basic_limit: base,
                xenon_phi_limit: xenon,
                hypervisor_limit: hyper,
                ext_limit: ext,
                transmeta_limit: trans,
                centaur_limit: centaur
            };
        }
    }


    // DEBUG FUNCTIONS
    pub fn print_limits(&self) {
        println!("LIMITS:");
        println!("  base:      0x{:x}", self.basic_limit);
        println!("  xenon:     0x{:x}", self.xenon_phi_limit);
        println!("  hyper:     0x{:x}", self.hypervisor_limit);
        println!("  extended:  0x{:x}", self.ext_limit);
        println!("  transmeta: 0x{:x}", self.transmeta_limit);
        println!("  centaur:   0x{:x}\n", self.centaur_limit);
    }


    // CHECK FUNCTIONS

    /// Returns TRUE if basic options are available
    pub fn basic_available(&self) -> bool {
        self.basic_limit > 0
    }

    /// Returns TRUE if xenon phi options are available
    pub fn xenon_phi_available(&self) -> bool {
        self.xenon_phi_limit > 0
    }

    /// Returns TRUE if hypervisor options are available
    pub fn hypervisor_available(&self) -> bool {
        self.hypervisor_limit > 0
    }

    /// Returns TRUE if extended options are available
    pub fn ext_available(&self) -> bool {
        self.ext_limit > 0
    }
    
    /// Returns TRUE if transmeta options are available
    pub fn transmeta_available(&self) -> bool {
        self.transmeta_limit > 0
    }

    /// Returns TRUE if centaur options are available
    pub fn centaur_available(&self) -> bool {
        self.centaur_limit > 0
    }




    // BASIC OPTIONS

    /// Max supported basic option and vendor ID string
    pub fn basic_info(&self) -> (u32, [char;12]) {
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
    pub fn cpu_info(&self) -> Option<CPUModel> {
        match self.get(BASIC_FMS_FLAGS, self.basic_limit) {
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

    /// Get feature list
    pub fn features(&self) -> Features {
        let basic_ecx;
        let basic_edx;

        // Basic flags
        match self.get(BASIC_FMS_FLAGS, self.basic_limit) {
            Some((_,_,c,d)) => {
                // Call was OK, filter out values
                basic_ecx = c;
                basic_edx = d;
            },
            None => {
                basic_ecx = 0;
                basic_edx = 0;
            }
        }

        let basic2_ebx;
        let basic2_ecx;
        
        // Basic2 flags
        match self.get(BASIC_FLAGS, self.basic_limit) {
            Some((_,b,c,_)) => {
                // Call was OK, filter out values
                basic2_ebx = b;
                basic2_ecx = c;
            },
            None => {
                basic2_ebx = 0;
                basic2_ecx = 0;
            }
        }

        let ext_ecx;
        let ext_edx;

        // Extended flags
        match self.get(EXT_FMS_FLAGS, self.basic_limit) {
            Some((_,_,c,d)) => {
                // Call was OK, filter out values
                ext_ecx = c;
                ext_edx = d;
            },
            None => {
                ext_ecx = 0;
                ext_edx = 0;
            }
        }

        Features::new(basic_ecx, basic_edx, basic2_ebx, basic2_ecx,
        ext_ecx, ext_edx)

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
