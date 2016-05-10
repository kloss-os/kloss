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

    // ---- 1st register

    /// Hypervisor present
    pub fn hv(&self) -> bool {
        (self.basic.0 & MASK_BIT_31) > 0
    }

    /// RdRand available (instruction for hardware random number)
    pub fn rdrand(&self) -> bool {
        (self.basic.0 & MASK_BIT_30) > 0
    }

    /// VCVTPH2PS and VCVTPS2PH instructions available
    pub fn f16c(&self) -> bool {
        (self.basic.0 & MASK_BIT_29) > 0
    }

    /// Advanced vector extensions available
    pub fn avx(&self) -> bool {
        (self.basic.0 & MASK_BIT_28) > 0
    }

    /// Complementary flag for XSAVE (I believe)
    // TODO: Look up OSXSAVE further
    pub fn osxsave(&self) -> bool {
        (self.basic.0 & MASK_BIT_27) > 0
    }

    /// Save processor extended state available
    pub fn xsave(&self) -> bool {
        (self.basic.0 & MASK_BIT_26) > 0
    }

    /// Advanced Enctyption Standard instruction set available
    pub fn aes(&self) -> bool {
        (self.basic.0 & MASK_BIT_25) > 0
    }

    /// Local APIC supports one-shot operation using TSC deadline value
    pub fn tscd(&self) -> bool {
        (self.basic.0 & MASK_BIT_24) > 0
    }

    /// If (intel?) POPCNT instruction is available
    pub fn popcnt(&self) -> bool {
        (self.basic.0 & MASK_BIT_23) > 0
    }

    /// Move Data After Swapping Bytes instruction available
    pub fn movbe(&self) -> bool {
        (self.basic.0 & MASK_BIT_22) > 0
    }

    /// x2APIC present
    pub fn x2apic(&self) -> bool {
        (self.basic.0 & MASK_BIT_21) > 0
    }

    /// SSE 4.2 supported
    pub fn sse4_2(&self) -> bool {
        (self.basic.0 & MASK_BIT_20) > 0
    }

    /// SSE 4.1 supported
    pub fn sse4_1(&self) -> bool {
        (self.basic.0 & MASK_BIT_19) > 0
    }

    /// Direct Cache Access
    pub fn dca(&self) -> bool {
        (self.basic.0 & MASK_BIT_18) > 0
    }

    /// Process Context Identifiers
    pub fn pcid(&self) -> bool {
        (self.basic.0 & MASK_BIT_17) > 0
    }
    
    /// Performance Debug Capability MSR
    pub fn pdcm(&self) -> bool {
        (self.basic.0 & MASK_BIT_15) > 0
    }

    /// MISC_ENABLE.ETPRD
    // TODO: Hitta info om denna
    pub fn etprd(&self) -> bool {
        (self.basic.0 & MASK_BIT_14) > 0
    }

    /// CMPXCHG16B instruction available
    /// (Atomic compare and exchange on 16-byte values)
    pub fn cx16(&self) -> bool {
        (self.basic.0 & MASK_BIT_13) > 0
    }

    /// Fused multiply-add
    pub fn fma(&self) -> bool {
        (self.basic.0 & MASK_BIT_12) > 0
    }

    /// DEBUG_INTERFACE MSR for silicon debug
    pub fn sdbg(&self) -> bool {
        (self.basic.0 & MASK_BIT_11) > 0
    }

    /// Context ID: the L1 data cache can be set to adaptive- or shared mode
    pub fn cid(&self) -> bool {
        (self.basic.0 & MASK_BIT_10) > 0
    }

    /// Supplemental Streaming SIMD Extensions 3
    pub fn ssse3(&self) -> bool {
        (self.basic.0 & MASK_BIT_9) > 0
    }

    /// Thermal Monitor 2
    pub fn tm2(&self) -> bool {
        (self.basic.0 & MASK_BIT_8) > 0
    }

    /// Enhanced SpeedStep
    pub fn est(&self) -> bool {
        (self.basic.0 & MASK_BIT_7) > 0
    }

    /// Safer mode trusted execution technology (Intel TXT, formerly known as
    /// LaGrande Technology) [Trusted Platform Module (TPM) Support]
    pub fn smx(&self) -> bool {
        (self.basic.0 & MASK_BIT_6) > 0
    }

    /// Hardware virtualization (Intel VMX)
    pub fn vmx(&self) -> bool {
        (self.basic.0 & MASK_BIT_5) > 0
    }

    /// CPL-qualified Debug Store 
    pub fn dscpl(&self) -> bool {
        (self.basic.0 & MASK_BIT_4) > 0
    }

    /// Monitor/MWait
    // TODO: Find mor info on `mon` flag
    pub fn mon(&self) -> bool {
        (self.basic.0 & MASK_BIT_3) > 0
    }

    /// 64/bit Debug Store
    pub fn dtes64(&self) -> bool {
        (self.basic.0 & MASK_BIT_2) > 0
    }

    /// PCLMUL Instruction set available (Intel Carry-Less
    /// Multiplication Instrucion)
    pub fn pclmul(&self) -> bool {
        (self.basic.0 & MASK_BIT_1) > 0
    }

    /// SSE 3 support
    pub fn sse3(&self) -> bool {
        (self.basic.0 & MASK_BIT_0) > 0
    }


    // ---- 2nd register

    /// Pending Break Event
    pub fn pbe(&self) -> bool {
        (self.basic.1 & MASK_BIT_31) > 0
    }

    /// Intel Itanium Architecture 64-bit (not same as Intel x86_64)
    pub fn ia64(&self) -> bool {
        (self.basic.1 & MASK_BIT_30) > 0
    }

    /// Thermal Monitor 1
    pub fn tm1(&self) -> bool {
        (self.basic.1 & MASK_BIT_29) > 0
    }

    /// Hyper Threading Technology
    pub fn htt(&self) -> bool {
        (self.basic.1 & MASK_BIT_28) > 0
    }

    /// SelfSnoop
    pub fn ss(&self) -> bool {
        (self.basic.1 & MASK_BIT_27) > 0
    }

    /// SSE 2 support
    pub fn sse2(&self) -> bool {
        (self.basic.1 & MASK_BIT_26) > 0
    }

    /// SSE support
    pub fn sse(&self) -> bool {
        (self.basic.1 & MASK_BIT_25) > 0
    }

    /// FXSAVE/FXRSTOR available
    pub fn fxsr(&self) -> bool {
        (self.basic.1 & MASK_BIT_24) > 0
    }

    /// ACPI via MSR (temperatire monitoring, clock speed modulation)
    pub fn acpi(&self) -> bool {
        (self.basic.1 & MASK_BIT_22) > 0
    }

    /// Debug Trace and EMON Store MSRs
    pub fn dtes(&self) -> bool {
        (self.basic.1 & MASK_BIT_21) > 0
    }

    /// CLFLUSH (Cache Line Flush) instruction available
    pub fn clfl(&self) -> bool {
        (self.basic.1 & MASK_BIT_19) > 0
    }

    /// Processor Serial Number
    pub fn psn(&self) -> bool {
        (self.basic.1 & MASK_BIT_18) > 0
    }
    
    /// Page Attribute Table
    pub fn pat(&self) -> bool {
        (self.basic.1 & MASK_BIT_16) > 0
    }

    /// SYSENTER/SYSEXIT instructions supported
    pub fn sep(&self) -> bool {
        (self.basic.1 & MASK_BIT_11) > 0
    }


    // -- BASIC FEATURE FLAGS (2)

    // ---- 1st register

    /// AVX512VL
    pub fn avx512vl(&self) -> bool {
        (self.basic2.0 & MASK_BIT_31) > 0
    }

    /// AVX512BW
    pub fn avx512nw(&self) -> bool {
        (self.basic2.0 & MASK_BIT_30) > 0
    }

    /// SHA
    pub fn sha(&self) -> bool {
        (self.basic2.0 & MASK_BIT_29) > 0
    }

    /// AVX512CD
    pub fn avx512cd(&self) -> bool {
        (self.basic2.0 & MASK_BIT_28) > 0
    }

    /// AVX512ER
    pub fn avx512er(&self) -> bool {
        (self.basic2.0 & MASK_BIT_27) > 0
    }

    /// AVX512PF
    pub fn avx512pf(&self) -> bool {
        (self.basic2.0 & MASK_BIT_26) > 0
    }

    /// Processor Trace (basic CPUID 0x0000_0014)
    pub fn pt(&self) -> bool {
        (self.basic2.0 & MASK_BIT_25) > 0
    }

    /// CLWB 
    pub fn clwb(&self) -> bool {
        (self.basic2.0 & MASK_BIT_24) > 0
    }

    /// CLFLUSHOPT
    pub fn clflushopt(&self) -> bool {
        (self.basic2.0 & MASK_BIT_23) > 0
    }

    /// PCOMMIT 
    pub fn pcommit(&self) -> bool {
        (self.basic2.0 & MASK_BIT_22) > 0
    }

    /// AVX512IFMA 
    pub fn avx512ifma(&self) -> bool {
        (self.basic2.0 & MASK_BIT_21) > 0
    }

    /// SMAP (`CR4.SMAP`, `CLAC` and `STAC`)
    pub fn smap(&self) -> bool {
        (self.basic2.0 & MASK_BIT_20) > 0
    }

    /// `ADXC` and `ADOX`
    pub fn adx(&self) -> bool {
        (self.basic2.0 & MASK_BIT_19) > 0
    }

    /// RDSEED
    pub fn rdseed(&self) -> bool {
        (self.basic2.0 & MASK_BIT_18) > 0
    }

    /// AVX512DQ
    pub fn avx512dq(&self) -> bool {
        (self.basic2.0 & MASK_BIT_17) > 0
    }

    /// AVX512F
    pub fn avx512f(&self) -> bool {
        (self.basic2.0 & MASK_BIT_16) > 0
    }

    /// Platform Quality of service Enforcement
    pub fn pqe(&self) -> bool {
        (self.basic2.0 & MASK_BIT_15) > 0
    }

    /// MPX
    pub fn mpx(&self) -> bool {
        (self.basic2.0 & MASK_BIT_14) > 0
    }

    /// `FP_CS` and `FP_DS` always saved as `0x0000`
    pub fn fpcsds(&self) -> bool {
        (self.basic2.0 & MASK_BIT_13) > 0
    }

    /// Platform Quality of service Monitoring
    pub fn pqm(&self) -> bool {
        (self.basic2.0 & MASK_BIT_12) > 0
    }

    /// `XBEGIN`, `XABORT`, `XEND`, `XTEST`, `DR7.RTM`, `DR6.RTM`
    pub fn rtm(&self) -> bool {
        (self.basic2.0 & MASK_BIT_11) > 0
    }

    /// INVPCID
    pub fn invpcid(&self) -> bool {
        (self.basic2.0 & MASK_BIT_10) > 0
    }

    /// Enhanced REP MOVSB/STOSB (while MISC_ENABLE.FSE=1) 
    pub fn erms(&self) -> bool {
        (self.basic2.0 & MASK_BIT_9) > 0
    }

    /// BMI2
    pub fn bmi2(&self) -> bool {
        (self.basic2.0 & MASK_BIT_8) > 0
    }

    /// CR4.SMEP
    pub fn smep(&self) -> bool {
        (self.basic2.0 & MASK_BIT_7) > 0
    }

    /// `FP_DP` for non-control instructions only if unmasked exception(s)
    pub fn fpdp(&self) -> bool {
        (self.basic2.0 & MASK_BIT_6) > 0
    }

    /// AVX2 (including VSIB)
    pub fn avx2(&self) -> bool {
        (self.basic2.0 & MASK_BIT_5) > 0
    }

    /// `XAQUIRE:`, `XRELEASE:`, `XTEST`
    pub fn hle(&self) -> bool {
        (self.basic2.0 & MASK_BIT_4) > 0
    }

    /// BMI1 and TZCNT
    pub fn bmi1(&self) -> bool {
        (self.basic2.0 & MASK_BIT_3) > 0
    }

    /// `CR4.SEE`, `PRMRR`, `ENCLS` and `ENCLU`, basic CPUID level `0x0000_0012`
    pub fn sgx(&self) -> bool {
        (self.basic2.0 & MASK_BIT_2) > 0
    }

    /// `TSC_ADJUST`
    pub fn tsc_adjust(&self) -> bool {
        (self.basic2.0 & MASK_BIT_1) > 0
    }

    /// `CR4.FSGSBASE` and `[RD|WR][FS|GS]BASE`
    pub fn fsgsbase(&self) -> bool {
        (self.basic2.0 & MASK_BIT_0) > 0
    }

    
    // ---- 2nd register
    
    /// SGX Launch Configuration
    pub fn sgx_lc(&self) -> bool {
        (self.basic2.1 & MASK_BIT_30) > 0
    }

    /// RDPID, TSC_AUX
    pub fn rdpid(&self) -> bool {
        (self.basic2.1 & MASK_BIT_22) > 0
    }

    /// Non-privileged read-only copy of current `CR4.PKE` value
    pub fn ospke(&self) -> bool {
        (self.basic2.1 & MASK_BIT_4) > 0
    }

    /// PKU
    pub fn pku(&self) -> bool {
        (self.basic2.1 & MASK_BIT_3) > 0
    }

    /// UMIP
    pub fn umip(&self) -> bool {
        (self.basic2.1 & MASK_BIT_2) > 0
    }

    /// AVX512VBMI
    pub fn avx512vbmi(&self) -> bool {
        (self.basic2.1 & MASK_BIT_1) > 0
    }

    /// PREFETCHWT1
    pub fn prefetchwt1(&self) -> bool {
        (self.basic2.1 & MASK_BIT_0) > 0
    }


    // -- EXTENDED FLAGS

    // ---- 1st register

    /// `MONITORX`/`MWAITX` 
    pub fn monx(&self) -> bool {
        (self.ext.0 & MASK_BIT_29) > 0
    }

    /// L2I perf counter extensions (MSRs C001_023[0...7]h)
    pub fn pcx_l2i(&self) -> bool {
        (self.ext.0 & MASK_BIT_28) > 0
    }

    /// performance TSC (MSR C001_0280h)
    pub fn perftsc(&self) -> bool {
        (self.ext.0 & MASK_BIT_27) > 0
    }

    /// Data breakpoint extensions (MSRs C001_1027h and C001_10[19...1B]h)
    pub fn dbx(&self) -> bool {
        (self.ext.0 & MASK_BIT_26) > 0
    }

    /// NB perf counter extensions (MSRs C001_024[0...7]h)
    pub fn pcx_nb(&self) -> bool {
        (self.ext.0 & MASK_BIT_24) > 0
    }

    /// Core perf counter extensions (MSRs C001_020[0...B]h)
    pub fn pcx_core(&self) -> bool {
        (self.ext.0 & MASK_BIT_23) > 0
    }

    /// Topology extensions: CPUID extended `0x8000_001D` and `0x8000_001E`
    pub fn topx(&self) -> bool {
        (self.ext.0 & MASK_BIT_22) > 0
    }

    /// TBM
    pub fn tbm(&self) -> bool {
        (self.ext.0 & MASK_BIT_21) > 0
    }

    /// Node ID: MSR `0xC001_100C`
    pub fn nodeid(&self) -> bool {
        (self.ext.0 & MASK_BIT_19) > 0
    }

    /// Translation cache extension, `EFER.TCE`
    pub fn tce(&self) -> bool {
        (self.ext.0 & MASK_BIT_17) > 0
    }

    /// FMA4
    pub fn fma4(&self) -> bool {
        (self.ext.0 & MASK_BIT_16) > 0
    }

    /// LWP
    pub fn lwp(&self) -> bool {
        (self.ext.0 & MASK_BIT_15) > 0
    }

    /// WatchDog Timer
    pub fn wdt(&self) -> bool {
        (self.ext.0 & MASK_BIT_13) > 0
    }

    /// `SKINIT`, `STGI`, `DEV`
    pub fn skinit(&self) -> bool {
        (self.ext.0 & MASK_BIT_12) > 0
    }

    /// XOP (was also used going to be used for SSE5A)
    pub fn xop(&self) -> bool {
        (self.ext.0 & MASK_BIT_11) > 0
    }

    /// Instruction Based Sampling
    pub fn ibs(&self) -> bool {
        (self.ext.0 & MASK_BIT_10) > 0
    }

    /// OS-Visible Workaround
    pub fn osvw(&self) -> bool {
        (self.ext.0 & MASK_BIT_9) > 0
    }

    /// 3DNow!P : `PREFETCH` and `PREFETCHW` (K8 Rev G and K8L+)
    pub fn _3dnow_p(&self) -> bool {
        (self.ext.0 & MASK_BIT_8) > 0
    }

    /// Misaligned SSE `MXCSR.MM`
    pub fn msse(&self) -> bool {
        (self.ext.0 & MASK_BIT_7) > 0
    }

    /// SSE 4A support
    pub fn sse4a(&self) -> bool {
        (self.ext.0 & MASK_BIT_6) > 0
    }

    /// LZCNT
    pub fn lzcnt(&self) -> bool {
        (self.ext.0 & MASK_BIT_5) > 0
    }

    /// MOV from/to CR8D by means of LOCK-prefixed MOV from/to CR0
    pub fn cr8d(&self) -> bool {
        (self.ext.0 & MASK_BIT_4) > 0
    }

    /// Extended APIC space (APIC_VER.EAS, EXT_APIC_FEAT, etc.)
    pub fn eas(&self) -> bool {
        (self.ext.0 & MASK_BIT_3) > 0
    }

    /// SVM
    pub fn svm(&self) -> bool {
        (self.ext.0 & MASK_BIT_2) > 0
    }

    /// CMP, HTT=1 indicates HTT (0) or CMP (1)
    pub fn cmp(&self) -> bool {
        (self.ext.0 & MASK_BIT_1) > 0
    }

    /// LAHF and SAHF in PM64
    pub fn ahf64(&self) -> bool {
        (self.ext.0 & MASK_BIT_0) > 0
    }

    // ---- 2nd register

    /// 3DNow!
    pub fn _3dnow(&self) -> bool {
        (self.ext.1 & MASK_BIT_31) > 0
    }

    /// Extended 3DNow!
    pub fn _3dnow_ext(&self) -> bool {
        (self.ext.1 & MASK_BIT_30) > 0
    }

    /// AMD64/EM64T, Long Mode
    pub fn lm(&self) -> bool {
        (self.ext.1 & MASK_BIT_29) > 0
    }

    /// `TSC`, `TSC_AUX`, `RDTSCP`, `CR4.TSD`
    pub fn tscp(&self) -> bool {
        (self.ext.1 & MASK_BIT_27) > 0
    }

    /// `PML3E.PS`
    pub fn pg1g(&self) -> bool {
        (self.ext.1 & MASK_BIT_26) > 0
    }

    /// `EFER.FFXSR`
    pub fn ffxsr(&self) -> bool {
        (self.ext.1 & MASK_BIT_25) > 0
    }

    // THERE ARE TWO MENINGS TO THIS FLAG
    
    /// Extended MMX (Cyrix only!)
    pub fn cyrix_mmx_ext(&self) -> bool {
        (self.ext.1 & MASK_BIT_24) > 0
    }

    /*
    TODO: Should be OR:ed with existing fxsr-flag IF processor
          is determined to be AMD K7.

    /// AMD K7 only: `FXSAVE`/`FXRSTOR`, `CR4.OSFXSR`
    pub fn fxsr(&self) -> bool {
        (self.ext.1 & MASK_BIT_24) > 0
    }
     */

    // EOF THERE ARE TWO MENINGS TO THIS FLAG

    /// AMD-specific MMX-SSE and SSE-MEM
    // TODO: Merge cyrix- and amd mmx_ext?
    pub fn amd_mmx_ext(&self) -> bool {
        (self.ext.1 & MASK_BIT_22) > 0
    }

    /// EFER.NXE, P?E.NX, #PF(1xxxx)
    pub fn nx(&self) -> bool {
        (self.ext.1 & MASK_BIT_20) > 0
    }

    /// MP-capable
    ///
    /// Note: AMD K7 processors prior to CPUID=0662h may
    ///       report 0 even if they are MP-capable.
    pub fn mp(&self) -> bool {
        (self.ext.1 & MASK_BIT_19) > 0
    }

    // THERE ARE TWO MENINGS TO THIS FLAG

    /// FCMOVcc/F(U)COMI(P) (implies FPU=1)
    pub fn fcmov(&self) -> bool {
        (self.ext.1 & MASK_BIT_16) > 0
    }

    /// AMD K7 Only: PAT MSR, PDE/PTE.PAT
    // TODO: Merge with existing pat flag?
    pub fn amd_k7_pat(&self) -> bool {
        (self.ext.1 & MASK_BIT_16) > 0
    }

    // EOF THERE ARE TWO MENINGS TO THIS FLAG

    /// SYSCALL/SYSRET, EFER/STAR MSRs 
    // TODO: OR with existing flag (?) called `sep` but not same description
    pub fn sep2(&self) -> bool {
        (self.ext.1 & MASK_BIT_11) > 0
    }


    // COMBINED FLAGS

    /// CMOV instructions supported (Conditional Move)
    pub fn cmov(&self) -> bool {
        ((self.basic.1 & MASK_BIT_15) > 0) |
        ((self.ext.1 & MASK_BIT_15) > 0)
    }

    /// Machine Check Architecture
    pub fn mca(&self) -> bool {
        ((self.basic.1 & MASK_BIT_14) > 0) |
        ((self.ext.1 & MASK_BIT_14) > 0)
    }

    /// Page Global Enable *global bit in PDEs and PTEs)
    pub fn pge(&self) -> bool {
        ((self.basic.1 & MASK_BIT_13) > 0) |
	((self.ext.1 & MASK_BIT_13) > 0)
    }

    /// Memory Type Range Registers
    pub fn mtrr(&self) -> bool {
        ((self.basic.1 & MASK_BIT_12) > 0) |
	((self.ext.1 & MASK_BIT_12) > 0)
    }

    /// Onboard APIC present
    pub fn apic(&self) -> bool {
        ((self.basic.1 & MASK_BIT_9) > 0) |
	((self.ext.1 & MASK_BIT_9) > 0)
    }

    /// CMPXCHG8 instruction (64-bit compare-and-swap) supported
    pub fn cx8(&self) -> bool {
        ((self.basic.1 & MASK_BIT_8) > 0) |
	((self.ext.1 & MASK_BIT_8) > 0)
    }

    /// Machine Check Exception
    pub fn mce(&self) -> bool {
        ((self.basic.1 & MASK_BIT_7) > 0) |
	((self.ext.1 & MASK_BIT_7) > 0)
    }

    /// Physical Address Extensoins (Support for >4GB RAM)
    pub fn pae(&self) -> bool {
        ((self.basic.1 & MASK_BIT_6) > 0) |
	((self.ext.1 & MASK_BIT_6) > 0)
    }

    /// Model-Specific Registers (RDMSR/WRMSR instructions supported)
    pub fn msr(&self) -> bool {
        ((self.basic.1 & MASK_BIT_5) > 0) |
	((self.ext.1 & MASK_BIT_5) > 0)
    }
    
    /// Time Stamp Counter
    pub fn tsc(&self) -> bool {
        ((self.basic.1 & MASK_BIT_4) > 0) |
	((self.ext.1 & MASK_BIT_4) > 0)
    }

    /// Page Size Extensions (4MB memory pages)
    pub fn pse(&self) -> bool {
        ((self.basic.1 & MASK_BIT_3) > 0) |
	((self.ext.1 & MASK_BIT_3) > 0)
    }

    /// Debugging Extensions (CR4.DE)
    pub fn de(&self) -> bool {
        ((self.basic.1 & MASK_BIT_2) > 0) |
	((self.ext.1 & MASK_BIT_2) > 0)
    }

    /// Virtual Mode Extensions (8086 mode)
    pub fn vme(&self) -> bool {
        ((self.basic.1 & MASK_BIT_1) > 0) |
	((self.ext.1 & MASK_BIT_1) > 0)
    }
    
    /// Onboard FPU (Floating Point Unit)
    pub fn fpu(&self) -> bool {
        ((self.basic.1 & MASK_BIT_0) > 0) |
	((self.ext.1 & MASK_BIT_0) > 0)
    }

    /// MultiMedia eXtensions
    pub fn mmx(&self) -> bool {
        ((self.basic.1 & MASK_BIT_23) > 0) |
        ((self.ext.1 & MASK_BIT_23) > 0)
    }

    /// 36-bit Page Size Extension available
    pub fn pse36(&self) -> bool {
        ((self.basic.1 & MASK_BIT_17) > 0) |
        ((self.ext.1 & MASK_BIT_17) > 0)
    }

}

