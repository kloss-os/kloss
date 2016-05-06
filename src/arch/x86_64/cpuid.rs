//! A module for using `cpuid`.
//!
//! The `cpuid` module allows for easy access to the CPUID command of the x86/x86_64 CPU
#![allow(dead_code)]

// RESOURCES:
// http://sandpile.org/x86/cpuid.htm
// PDF:s in google drive folder (simon)

// CONSTANTS FOR USING CPUID
// -- BASIC CPUID
const BASIC_INFO : u32 = 0x0;
const BASIC_1 : u32 = 0x1;
const BASIC_2 : u32 = 0x2;
const BASIC_3 : u32 = 0x3;
const BASIC_4 : u32 = 0x4;
const BASIC_5 : u32 = 0x5;


// -- EXTENDED CPUID
const EXT_INFO : u32 = 0x80000000;
const EXT_1 : u32 = 0x80000001;
const EXT_2 : u32 = 0x80000002;
const EXT_3 : u32 = 0x80000003;
const EXT_4 : u32 = 0x80000004;
const EXT_5 : u32 = 0x80000005;
const EXT_6 : u32 = 0x80000006;
const EXT_7 : u32 = 0x80000007;
const EXT_8 : u32 = 0x80000008;

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

    /// Call CPUID using supplied option, only
    /// if option does not exceed highest option
    /// available.
    pub fn get(&self, option: u32) -> Option<(u32,u32,u32,u32)> {
        // Check if valid basic- or extended option
        let basic = (option >= BASIC_INFO) && (option <= self.basic_highest_option);
        let extended = (option >= EXT_INFO) && (option <= self.ext_highest_option);

        if basic || extended {
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
