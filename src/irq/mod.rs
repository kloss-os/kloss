//! This is a module to handle and set up the (software) side of
//! interrupts. It (and its submodules) deals with setting up the IDT,
//! wrapping assembler magic and so on.
//! See also the x86 crate!
//!
//! # Usage
//! Before doing anything else, you need to perform
//! `irq::install()` to configure the CPU to use the provided
//! interrupt descriptor table (IDT). This will install the module's
//! default ISRs (Interrupt Service Routines) for every exception and
//! interrupt. There are then two ways you can go about adding your own
//! ISRs:
//!
//! 1. Add them using `irq::idt::set_gate()`. This will override the
//!    system routine in the IDT and the provided routine _must be an
//!    assembler routine_ following interrupt handler calling
//!    conventions until Rust supports naked functions.
//! 2. Add a Rust handler using `irq::set_handler()`. Note that this is
//!    _way slower_ and more indirect than the direct CPU dispatch, as it
//!    involves at least one layer of indirection between calls.
//!
//! Also note that you need to define and export the non-mangled function
//! `rust_interrupt_handler` from your main file, and in that function
//! call `irq::entry()`, that is the dispatch entry function.


/// Default flags for all system trap gates.
pub static DEFAULT_FLAGS: u8 = self::idt::FLAG_TYPE_TRAP_GATE
    | self::idt::FLAG_DPL_KERNEL_MODE | self::idt::FLAG_GATE_ENABLED;

// Modules and re-exports
pub mod idt;

mod asm_wrappers;

pub use self::asm_wrappers::{null_interrupt_handler as isr_null};

use self::asm_wrappers::*;

mod dispatch;

#[cfg(test)]
mod tests;

// Exception entry point re-export
pub use self::dispatch::entry;

// End modules and re-exports

/// Set the (module-internal) interrupt handler for vector `vec`.
///
/// **Warning**: This will only work if:
///
/// 1. The module's IDT was installed using `irq::install()`, and
/// 2. The ISR for `vec` has not been replaced using
///    `irq::idt::set_gate()`.
///
/// - `vec` can be any integer in the range [0, 255].
/// - `f` must be a handling Rust function taking as its single
///   argument the triggered interrupt. `f` can be `unsafe`.
///
/// # Examples
///
/// Run the dummy function `krnl_dontpanic` via the high-level interrupt
/// handling system whenever the interrupt number 42 is triggered:
///
/// ```
/// fn unsafe krnl_dontpanic(vec: usize) {
///     println!("Don't panic! I just caught interrupt no {}", vec);
/// }
/// //...
/// //...
/// set_handler(42, krnl_dontpanic);
/// ```
pub fn set_handler(vec: usize,
                   f: unsafe fn(usize)) -> () {

    unsafe {self::dispatch::set_handler(vec, f);}

}

/// Install and initialise the system IDT and ISR:s for every interrupt
/// and exception.
pub unsafe fn install() {

    self::idt::install();

    // Here follows 256 identical lines:
    self::idt::set_gate(0, isr_0,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(1, isr_1,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(2, isr_2,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(3, isr_3,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(4, isr_4,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(5, isr_5,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(6, isr_6,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(7, isr_7,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(8, isr_8,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(9, isr_9,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(10, isr_10,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(11, isr_11,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(12, isr_12,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(13, isr_13,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(14, isr_14,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(15, isr_15,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(16, isr_16,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(17, isr_17,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(18, isr_18,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(19, isr_19,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(20, isr_20,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(21, isr_21,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(22, isr_22,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(23, isr_23,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(24, isr_24,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(25, isr_25,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(26, isr_26,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(27, isr_27,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(28, isr_28,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(29, isr_29,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(30, isr_30,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(31, isr_31,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(32, isr_32,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(33, isr_33,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(34, isr_34,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(35, isr_35,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(36, isr_36,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(37, isr_37,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(38, isr_38,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(39, isr_39,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(40, isr_40,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(41, isr_41,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(42, isr_42,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(43, isr_43,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(44, isr_44,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(45, isr_45,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(46, isr_46,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(47, isr_47,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(48, isr_48,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(49, isr_49,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(50, isr_50,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(51, isr_51,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(52, isr_52,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(53, isr_53,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(54, isr_54,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(55, isr_55,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(56, isr_56,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(57, isr_57,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(58, isr_58,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(59, isr_59,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(60, isr_60,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(61, isr_61,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(62, isr_62,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(63, isr_63,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(64, isr_64,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(65, isr_65,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(66, isr_66,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(67, isr_67,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(68, isr_68,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(69, isr_69,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(70, isr_70,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(71, isr_71,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(72, isr_72,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(73, isr_73,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(74, isr_74,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(75, isr_75,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(76, isr_76,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(77, isr_77,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(78, isr_78,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(79, isr_79,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(80, isr_80,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(81, isr_81,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(82, isr_82,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(83, isr_83,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(84, isr_84,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(85, isr_85,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(86, isr_86,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(87, isr_87,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(88, isr_88,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(89, isr_89,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(90, isr_90,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(91, isr_91,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(92, isr_92,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(93, isr_93,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(94, isr_94,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(95, isr_95,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(96, isr_96,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(97, isr_97,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(98, isr_98,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(99, isr_99,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(100, isr_100,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(101, isr_101,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(102, isr_102,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(103, isr_103,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(104, isr_104,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(105, isr_105,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(106, isr_106,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(107, isr_107,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(108, isr_108,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(109, isr_109,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(110, isr_110,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(111, isr_111,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(112, isr_112,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(113, isr_113,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(114, isr_114,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(115, isr_115,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(116, isr_116,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(117, isr_117,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(118, isr_118,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(119, isr_119,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(120, isr_120,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(121, isr_121,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(122, isr_122,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(123, isr_123,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(124, isr_124,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(125, isr_125,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(126, isr_126,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(127, isr_127,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(128, isr_128,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(129, isr_129,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(130, isr_130,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(131, isr_131,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(132, isr_132,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(133, isr_133,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(134, isr_134,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(135, isr_135,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(136, isr_136,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(137, isr_137,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(138, isr_138,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(139, isr_139,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(140, isr_140,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(141, isr_141,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(142, isr_142,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(143, isr_143,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(144, isr_144,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(145, isr_145,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(146, isr_146,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(147, isr_147,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(148, isr_148,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(149, isr_149,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(150, isr_150,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(151, isr_151,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(152, isr_152,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(153, isr_153,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(154, isr_154,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(155, isr_155,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(156, isr_156,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(157, isr_157,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(158, isr_158,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(159, isr_159,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(160, isr_160,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(161, isr_161,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(162, isr_162,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(163, isr_163,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(164, isr_164,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(165, isr_165,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(166, isr_166,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(167, isr_167,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(168, isr_168,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(169, isr_169,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(170, isr_170,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(171, isr_171,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(172, isr_172,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(173, isr_173,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(174, isr_174,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(175, isr_175,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(176, isr_176,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(177, isr_177,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(178, isr_178,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(179, isr_179,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(180, isr_180,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(181, isr_181,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(182, isr_182,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(183, isr_183,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(184, isr_184,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(185, isr_185,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(186, isr_186,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(187, isr_187,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(188, isr_188,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(189, isr_189,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(190, isr_190,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(191, isr_191,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(192, isr_192,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(193, isr_193,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(194, isr_194,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(195, isr_195,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(196, isr_196,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(197, isr_197,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(198, isr_198,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(199, isr_199,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(200, isr_200,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(201, isr_201,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(202, isr_202,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(203, isr_203,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(204, isr_204,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(205, isr_205,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(206, isr_206,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(207, isr_207,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(208, isr_208,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(209, isr_209,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(210, isr_210,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(211, isr_211,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(212, isr_212,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(213, isr_213,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(214, isr_214,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(215, isr_215,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(216, isr_216,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(217, isr_217,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(218, isr_218,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(219, isr_219,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(220, isr_220,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(221, isr_221,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(222, isr_222,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(223, isr_223,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(224, isr_224,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(225, isr_225,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(226, isr_226,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(227, isr_227,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(228, isr_228,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(229, isr_229,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(230, isr_230,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(231, isr_231,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(232, isr_232,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(233, isr_233,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(234, isr_234,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(235, isr_235,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(236, isr_236,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(237, isr_237,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(238, isr_238,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(239, isr_239,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(240, isr_240,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(241, isr_241,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(242, isr_242,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(243, isr_243,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(244, isr_244,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(245, isr_245,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(246, isr_246,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(247, isr_247,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(248, isr_248,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(249, isr_249,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(250, isr_250,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(251, isr_251,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(252, isr_252,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(253, isr_253,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(254, isr_254,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    self::idt::set_gate(255, isr_255,
                        self::idt::SELECT_TARGET_PRIV_1,
                        DEFAULT_FLAGS);
    // End of IDT installation.
}
