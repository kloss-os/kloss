//! Banjos is an experimental kernel for (reasonably) modern x86_64
//! systems written in Rust. It was intended as an educational exercise,
//! and strives to minimise complexity while maximising good programming
//! practices, clarity, and use of high-level designs over raw
//! performance.

#![feature(concat_idents)]

#![feature(lang_items)]
#![feature(const_fn, unique)]
#![feature(alloc, collections)]
#![no_std]

#![feature(asm)]

#![feature(core_intrinsics)]

extern crate rlibc;
extern crate spin;
extern crate multiboot2;

#[macro_use]
extern crate bitflags;

// Use the int! macro
#[macro_use]
extern crate x86;

#[macro_use]
extern crate once;
//extern crate bump_allocator;
extern crate hole_list_allocator;
extern crate alloc;
#[macro_use]
extern crate collections;

#[macro_use]
#[doc(inline)]
mod vga_buffer;
mod memory;

mod acpi;
mod io;

#[macro_use]
mod irq;
mod arch;

mod msr;
mod pipe;
/// This is the kernel main function! Control is passed after the ASM
/// parts have finished.
///
/// Note how the multiboot data is passed in from a raw pointer (usize),
/// via the assembler parts.
#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {

    vga_buffer::clear_screen();
    println!("Hello Rust!!!");

    let rsdt = acpi::get_rsdt();
    let mut sdt_loc = &mut acpi::sdt_loc_new();
    let ioapic: u32;

    // This should be generalised for incompatible processors...?
    if let Some(ref rsdtr) = rsdt {
        sdt_loc.sdt_loc_load(rsdtr);
        if let Some(ioapicaddr) = acpi::get_ioapic_addr(rsdtr) {
            ioapic = ioapicaddr;
        } else {
            ioapic = 0x0;
        }
    } else {
        println!("FAILED to load RSDT");
        ioapic = 0x0;
    }

    // Parse the boot info data from the Multiboot header
    let boot_info = unsafe{
        multiboot2::load(multiboot_information_address)
    };

    enable_nxe_bit();
    enable_write_protect_bit();

    // Initialize memory mapping and paging, as well as
    // kernel-remap and all other memory-related set-up
    memory::init(boot_info, sdt_loc);

    println!("Setting up the IDT!");
    unsafe{
        // Install IRQ
        irq::install();
        // Enable global interrupts!
        x86::irq::enable();
    }

    io::install_io(sdt_loc.lapic_ctrl, sdt_loc.ioapic_start);

    // ======================================================
    // Test heap
    // ======================================================

    use alloc::boxed::Box;
    use collections::String;
    // let heap_test = Box::new(42);

    //let v = vec![1,2,3,4,5];
    //for i in &v {
    //    print!("{}", i);
    //}

    //let hello = String::from("Hello from heap!");
    //println!("{}",hello);
    
    use pipe::Buffer;
    let mut buffer = Buffer::new();
    buffer.write(42);
    println!("{}", buffer.read());

    // ======================================================
    // EOF Test heap
    // ======================================================

    // Final print before infloop
    println!("It did not crash!");

    // Loop to infinity and beyond!
    loop{}
}


fn enable_nxe_bit() {
    use x86::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86::controlregs::{cr0, cr0_write};
    let wp_bit = 1 << 16;
    unsafe { cr0_write(cr0() | wp_bit) };
}

/// This is an override for a language feature. Don't know what it does.
#[cfg(not(feature = "tests"))]
#[lang = "eh_personality"]
extern fn eh_personality() {}

/// This is an override for the Rust panic handler, and it runs when
/// something crashes. This version displays PANIC and a description of
/// where the panic occurred.  Try invoking the panic!(); macro to see
/// it in action.
#[cfg(not(feature = "tests"))]
#[lang = "panic_fmt"]
extern fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);

    loop{}
}

// These functions are called from the ASM interrupt wrappers, and they
// need to be here, unfortunately.

#[no_mangle]
pub extern fn rust_interrupt_handler(intnr: usize) {
    irq::entry(intnr);
}

#[no_mangle]
pub extern fn rust_exception_handler() {
    println!("Handled exception!");
    irq::entry(0);
}


