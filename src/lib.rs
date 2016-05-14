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
extern crate bump_allocator;
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
    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    // Dump memory areas to screen.
    println!("Memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length 0x{:x}",
                area.base_addr,
                area.length);
    }

    // Load the elf-sections tags from the (now parsed) Multiboot header
    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf-sections tag required");

    // ...and print them to screen.
    println!("Kernel sections:");
    for section in elf_sections_tag.sections() {
        println!("    addr: 0x{:x}, size 0x{:x}, flags: 0x:{:x}",
                 section.addr, section.size, section.flags);
    }

    // Calculate the start and end addresses of the actual kernel data in RAM.
    // This will be useful for kernel relocation (and memory allocation).
    // Note how we are using map() and anonymous functions _in kernel space_.
    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("Kernel start: 0x{:x}, kernel end: 0x{:x}",
             kernel_start, kernel_end);

    println!("Multiboot start: 0x{:x}, multiboot end: 0x{:x}",
             multiboot_start, multiboot_end);

    // Set up a frame allocator.
    let mut frame_allocator = memory::AreaFrameAllocator::new(
        kernel_start as usize,
        kernel_end as usize,
        multiboot_start,
        multiboot_end,
        memory_map_tag.memory_areas());

    enable_nxe_bit();
    enable_write_protect_bit();

    /*
    memory::test_paging(&mut frame_allocator);

    // Try allocating _all available frames_.
    for i in 0.. {
        use memory::FrameAllocator;
        if let None = frame_allocator.allocate_frame() {
            println!("Allocated {} frames", i);
            break;
        }
    }
    */

    println!("Setting up the IDT!");
    unsafe{
        irq::install();

        asm!("int 42" ::::"intel");
        asm!("int 128" ::::"intel");
        asm!("int 255" ::::"intel");

        // Enable global interrupts!
        x86::irq::enable();
    }



    //memory::test_paging(&mut frame_allocator);

    memory::remap_the_kernel(&mut frame_allocator, boot_info, sdt_loc);

    // Denna skit är tveksam
    //frame_allocator.allocate_frame();

    io::install_io(sdt_loc.lapic_ctrl, sdt_loc.ioapic_start);


    // Test heap
    use alloc::boxed::Box;
    let heap_test = Box::new(42);

    println!("It did not crash!");



    println!("It did not crash!");
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
