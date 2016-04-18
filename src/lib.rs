#![feature(lang_items)]
#![no_std]

#![feature(const_fn)]
#![feature(unique)]

extern crate rlibc;
extern crate spin;

#[macro_use]
#[doc(inline)]
mod vga_buffer;

/// This is the kernel main function! Control is passed after the ASM
/// parts have finished.
#[no_mangle]
pub extern fn rust_main() {
    vga_buffer::clear_screen();
    println!("Hello Rust!!{}", "!");
    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}
