//! A module for handling CPUID feature flags.
#![allow(dead_code)]

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


pub struct Features {
    // Basic features
    /// Primary basic feature flags
    basic:  (u32, u32),
    /// Secondary basic feature flags
    basic2: (u32, u32),
    /// Extended feature flags
    ext:    (u32, u32)
}

impl Features {
    /// Creates a new Features object
    pub fn new(basic_ecx: u32, basic_edx: u32,
               basic2_ecx: u32, basic2_edx: u32,
               ext_ecx: u32, ext_edx: u32
    ) -> Features {
        Features {
            basic:  (basic_ecx, basic_edx),
            basic2: (basic_ecx, basic_edx),
            ext:    (ext_ecx, ext_edx)
        } 
    }

    // -- BASIC FLAGS

    // -- 1st register

    pub fn hv(&self) -> bool {
        (self.basic.0 & MASK_BIT_31) > 0
    }


    pub fn rdrand(&self) -> bool {
        (self.basic.0 & MASK_BIT_30) > 0
    }


    pub fn f16c(&self) -> bool {
        (self.basic.0 & MASK_BIT_29) > 0
    }


    pub fn avx(&self) -> bool {
        (self.basic.0 & MASK_BIT_28) > 0
    }


    pub fn osxsave(&self) -> bool {
        (self.basic.0 & MASK_BIT_27) > 0
    }


    pub fn xsave(&self) -> bool {
        (self.basic.0 & MASK_BIT_26) > 0
    }

    
    pub fn aes(&self) -> bool {
        (self.basic.0 & MASK_BIT_25) > 0
    }

    
    pub fn tscd(&self) -> bool {
        (self.basic.0 & MASK_BIT_24) > 0
    }


    pub fn popcnt(&self) -> bool {
        (self.basic.0 & MASK_BIT_23) > 0
    }


    pub fn movbe(&self) -> bool {
        (self.basic.0 & MASK_BIT_22) > 0
    }


    pub fn x2apic(&self) -> bool {
        (self.basic.0 & MASK_BIT_21) > 0
    }


    pub fn sse4_2(&self) -> bool {
        (self.basic.0 & MASK_BIT_20) > 0
    }


    pub fn sse4_1(&self) -> bool {
        (self.basic.0 & MASK_BIT_19) > 0
    }


    pub fn dca(&self) -> bool {
        (self.basic.0 & MASK_BIT_18) > 0
    }


    pub fn pcid(&self) -> bool {
        (self.basic.0 & MASK_BIT_17) > 0
    }
    

    pub fn pdcm(&self) -> bool {
        (self.basic.0 & MASK_BIT_15) > 0
    }


    pub fn etprd(&self) -> bool {
        (self.basic.0 & MASK_BIT_14) > 0
    }


    pub fn cx16(&self) -> bool {
        (self.basic.0 & MASK_BIT_13) > 0
    }


    pub fn fma(&self) -> bool {
        (self.basic.0 & MASK_BIT_12) > 0
    }


    pub fn sdbg(&self) -> bool {
        (self.basic.0 & MASK_BIT_11) > 0
    }


    pub fn cid(&self) -> bool {
        (self.basic.0 & MASK_BIT_10) > 0
    }


    pub fn ssse3(&self) -> bool {
        (self.basic.0 & MASK_BIT_9) > 0
    }


    pub fn tm2(&self) -> bool {
        (self.basic.0 & MASK_BIT_8) > 0
    }


    pub fn est(&self) -> bool {
        (self.basic.0 & MASK_BIT_7) > 0
    }


    pub fn smx(&self) -> bool {
        (self.basic.0 & MASK_BIT_6) > 0
    }


    pub fn vmx(&self) -> bool {
        (self.basic.0 & MASK_BIT_5) > 0
    }

    pub fn dscpl(&self) -> bool {
        (self.basic.0 & MASK_BIT_4) > 0
    }


    pub fn mon(&self) -> bool {
        (self.basic.0 & MASK_BIT_3) > 0
    }


    pub fn dtes64(&self) -> bool {
        (self.basic.0 & MASK_BIT_2) > 0
    }


    pub fn pclmul(&self) -> bool {
        (self.basic.0 & MASK_BIT_1) > 0
    }

    pub fn sse3(&self) -> bool {
        (self.basic.0 & MASK_BIT_0) > 0
    }

    // -- 2nd register

    pub fn pbe(&self) -> bool {
        (self.basic.1 & MASK_BIT_31) > 0
    }


    pub fn ia64(&self) -> bool {
        (self.basic.1 & MASK_BIT_30) > 0
    }


    pub fn tml(&self) -> bool {
        (self.basic.1 & MASK_BIT_29) > 0
    }


    pub fn htt(&self) -> bool {
        (self.basic.1 & MASK_BIT_28) > 0
    }


    pub fn ss(&self) -> bool {
        (self.basic.1 & MASK_BIT_27) > 0
    }


    pub fn sse2(&self) -> bool {
        (self.basic.1 & MASK_BIT_26) > 0
    }

    
    pub fn sse(&self) -> bool {
        (self.basic.1 & MASK_BIT_25) > 0
    }

    
    pub fn fxsr(&self) -> bool {
        (self.basic.1 & MASK_BIT_24) > 0
    }


    pub fn mmx(&self) -> bool {
        (self.basic.1 & MASK_BIT_23) > 0
    }


    pub fn acpi(&self) -> bool {
        (self.basic.1 & MASK_BIT_22) > 0
    }


    pub fn dtes(&self) -> bool {
        (self.basic.1 & MASK_BIT_21) > 0
    }


    pub fn clfl(&self) -> bool {
        (self.basic.1 & MASK_BIT_19) > 0
    }


    pub fn psn(&self) -> bool {
        (self.basic.1 & MASK_BIT_18) > 0
    }


    pub fn pse36(&self) -> bool {
        (self.basic.1 & MASK_BIT_17) > 0
    }
    

    pub fn pat(&self) -> bool {
        (self.basic.1 & MASK_BIT_16) > 0
    }


    pub fn cmov(&self) -> bool {
        (self.basic.1 & MASK_BIT_15) > 0
    }


    pub fn mca(&self) -> bool {
        (self.basic.1 & MASK_BIT_14) > 0
    }


    pub fn pge(&self) -> bool {
        (self.basic.1 & MASK_BIT_13) > 0
    }


    pub fn mtrr(&self) -> bool {
        (self.basic.1 & MASK_BIT_12) > 0
    }


    pub fn sep(&self) -> bool {
        (self.basic.1 & MASK_BIT_11) > 0
    }


    pub fn apic(&self) -> bool {
        (self.basic.1 & MASK_BIT_9) > 0
    }


    pub fn cx8(&self) -> bool {
        (self.basic.1 & MASK_BIT_8) > 0
    }


    pub fn mce(&self) -> bool {
        (self.basic.1 & MASK_BIT_7) > 0
    }


    pub fn pae(&self) -> bool {
        (self.basic.1 & MASK_BIT_6) > 0
    }


    pub fn msr(&self) -> bool {
        (self.basic.1 & MASK_BIT_5) > 0
    }

    pub fn tsc(&self) -> bool {
        (self.basic.1 & MASK_BIT_4) > 0
    }


    pub fn pse(&self) -> bool {
        (self.basic.1 & MASK_BIT_3) > 0
    }


    pub fn de(&self) -> bool {
        (self.basic.1 & MASK_BIT_2) > 0
    }


    pub fn vme(&self) -> bool {
        (self.basic.1 & MASK_BIT_1) > 0
    }

    pub fn fpu(&self) -> bool {
        (self.basic.1 & MASK_BIT_0) > 0
    }
}


/*
   OLD STRUCT

// Struct for listing CPU flags
pub struct Features {
    // ECX Register

    /// Hypervisor present
    pub hv: bool,

    /// RdRand available (instruction for hardware random number)
    pub rdrand: bool,

    /// VCVTPH2PS and VCVTPS2PH instructions available
    pub f16c: bool,

    /// Advanced vector extensions available
    pub avx: bool,

    /// Complementary flag for XSAVE (I believe)
    // TODO: Look up OSXSAVE further
    pub osxsave: bool,

    /// Save processor extended state available
    pub xsave: bool,

    /// Advanced Enctyption Standard instruction set available
    pub aes: bool,

    /// Local APIC supports one-shot operation using TSC deadline value
    pub tscd: bool,

    /// If (intel?) POPCNT instruction is available
    pub popcnt: bool,

    /// Move Data After Swapping Bytes instruction available
    pub movbe: bool,

    /// x2APIC present
    pub x2apic: bool,

    /// SSE 4.2 supported
    pub sse4_2: bool,

    /// SSE 4.1 supported 
    pub sse4_1: bool,
    
    /// Direct Cache Access
    pub dca: bool,

    /// Process Context Identifiers
    pub pcid: bool,

    // Bit 16 reserved

    /// Performance Debug Capability MSR
    pub pdcm: bool,

    /// MISC_ENABLE.ETPRD
    // TODO: Hitta info om denna
    pub etprd: bool,

    /// CMPXCHG16B instruction available
    /// (Atomic compare and exchange on 16-byte values)
    pub cx16: bool,

    /// Fused multiply-add
    pub fma: bool,

    /// DEBUG_INTERFACE MSR for silicon debug
    pub sdbg: bool,

    /// Context ID: the L1 data cache can be set to adaptive- or shared mode
    pub cid: bool,

    /// Supplemental Streaming SIMD Extensions 3
    pub ssse3: bool,

    /// Thermal Monitor 2
    pub tm2: bool,

    /// Enhanced SpeedStep
    pub est: bool,

    /// Safer mode trusted execution technology (Intel TXT, formerly known as
    /// LaGrande Technology) [Trusted Platform Module (TPM) Support]
    pub smx: bool,

    /// Hardware virtualization (Intel VMX)
    pub vmx: bool,

    /// CPL-qualified Debug Store 
    pub dscpl: bool,

    /// Monitor/MWait
    // TODO: Find mor info on `mon` flag
    pub mon: bool,

    /// 64/bit Debug Store
    pub dtes64: bool,

    /// PCLMUL Instruction set available (Intel Carry-Less
    /// Multiplication Instrucion)  
    pub pclmul: bool,

    /// SSE 3 support
    pub sse3: bool,
    
    // EDX register
    
    /// Pending Break Event
    pub pbe: bool,

    /// Intel Itanium Architecture 64-bit (not same as Intel x86_64)
    pub ia64: bool,

    /// Thermal Monitor 1
    pub tm1: bool,

    /// Hyper Threading Technology
    pub htt: bool,

    /// SelfSnoop
    pub ss: bool,

    /// SSE 2 support
    pub sse2: bool,

    /// SSE support
    pub sse: bool,

    /// FXSAVE/FXRSTOR available
    pub fxsr: bool,

    /// MultiMedia eXtensions
    pub mmx: bool,

    /// ACPI via MSR (temperatire monitoring, clock speed modulation)
    pub acpi: bool,

    /// Debug Trace and EMON Store MSRs
    pub dtes: bool,

    // Bit 20 reserved

    /// CLFLUSH (Cache Line Flush) instruction available
    pub clfl: bool,

    /// Processor Serial Number
    pub psn: bool,

    /// 36-bit Page Size Extension available
    pub pse36: bool,

    /// Page Attribute Table
    pub pat: bool,

    /// CMOV instructions supported (Conditional Move)
    pub cmov: bool,

    /// Machine Check Architecture
    pub mca: bool,

    /// Page Global Enable *global bit in PDEs and PTEs)
    pub pge: bool,

    /// Memory Type Range Registers
    pub mtrr: bool,

    /// SYSENTER/SYSEXIT instructions supported
    pub sep: bool,

    // Bit 10 reserved

    /// Onboard APIC present
    pub apic: bool,

    /// CMPXCHG8 instruction (64-bit compare-and-swap) supported
    pub cx8: bool,

    /// Machine Check Exception
    pub mce: bool,

    /// Physical Address Extensoins (Support for >4GB RAM)
    pub pae: bool,

    /// Model-Specific Registers (RDMSR/WRMSR instructions supported)
    pub msr: bool,

    /// Time Stamp Counter
    pub tsc: bool,

    /// Page Size Extensions (4MB memory pages)
    pub pse: bool,

    /// Debugging Extensions (CR4.DE)
    pub de: bool,

    /// Virtual Mode Extensions (8086 mode)
    pub vme: bool,

    /// Onboard FPU (Floating Point Unit)
    pub fpu: bool
}





*/
