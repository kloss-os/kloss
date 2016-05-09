// This is the main kernel file.
#![feature(lang_items)]
#![no_std]

#![feature(const_fn)]
#![feature(unique)]
#![feature(asm)]

extern crate rlibc;
extern crate spin;
extern crate multiboot2;


extern {
    fn general_interrupt_handler();
    fn general_exception_handler();
}

#[macro_use]
extern crate bitflags;
extern crate x86;

#[macro_use]
#[doc(inline)]
mod vga_buffer;
mod memory;
mod idt;
mod arch;

/// This is the kernel main function! Control is passed after the ASM
/// parts have finished.
///
/// Note how the multiboot data is passed in from a raw pointer (usize),
/// via the assembler parts.
#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {

    vga_buffer::clear_screen();
    println!("Hello Rust!!!");

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
/*
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
        idt::idt_install();
        let flags =   idt::FLAG_TYPE_TRAP_GATE
                    | idt::FLAG_DPL_KERNEL_MODE
                    | idt::FLAG_GATE_ENABLED;


        // Install interrupt handlers for *everything*!
        for ev in 0..33 {
            idt::idt_set_gate(ev, general_exception_handler,
                              idt::SELECT_TARGET_PRIV_1, flags);
        }

        for iv in 33..256 {
            idt::idt_set_gate(iv, general_interrupt_handler,
                              idt::SELECT_TARGET_PRIV_1, flags);
        }

        // Test out interrupts
        // for iv in 0..256 {
        //     println!("Sending interrupt #{}", iv);
        //     asm!("int 42" ::::"intel");
        // }
        // asm!("int 42" ::::"intel");
        // asm!("int 42" ::::"intel");

        asm!("int 42" ::::"intel");

        // Enable global interrupts!
        asm!("sti" ::::"intel");
    }

    // TESTING CPUID MODULE
    use arch::x86_64::cpuid;
    let m_cpuid: cpuid::CPUID = cpuid::CPUID::new();
    
    // Basic info
    let (max,vid) = m_cpuid.basicInfo();
    println!("\nCPUID BASE INFO:");
    println!("  Max instruction: 0x{:x}\n  Vendor ID: {:?}\n", max, vid);
    m_cpuid.print_limits();
    // FMT
 //   match m_cpuid.cpuModel() {
 //       Some(fms) => println!("CPUID FMT:\n  stepping: 0x{:x}\n  type: 0x{:x}\n  model:\
 //                              0x{:x}\n  family: 0x{:x}\n  brand: 0x{:x}\n  cpu count: 0x{:x}\n  \
 //                              apic-id: 0x{:x}\n  clflush: 0x{:x}",
 //                             fms.stepping, fms.cpu_type, fms.model, fms.family, fms.brand,
 //                             fms.cpu_cnt, fms.apic_id, fms.clflush),
 //       None => println!("FMT command not available.")

 //   }

    // FEATURES
 //   match m_cpuid.features() {
 //       Some(cpuflags) => println!("SSE3: {}", cpuflags.sse3),
 //       None => println!("Flag command not available.")
 //   }

 //   println!("Ran {} recursive calls", call_recursively(10));
 //   println!("3! = {}", fac(3));

    memory::test_paging(&mut frame_allocator);

    loop{}
}

fn call_recursively(n: u64) -> u64 {
    match n {
        0 => 0,
        _ => 1 + call_recursively(n-1)
    }
}

fn fac(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => n * fac(n-1)
    }
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

#[no_mangle]
pub extern fn rust_interrupt_handler() {

    println!("Handled interrupt!");
}

#[no_mangle]
pub extern fn rust_exception_handler() {
    println!("Handled exception!");
}
