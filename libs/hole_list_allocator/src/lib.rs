// This crate is really just a wrapper around the already-created crate
// `linked_list_allocator` by Philipp Oppermann, as to create a `static`
// heap that can be accessed by the rust allocation functions.

#![feature(allocator)]
#![feature(const_fn)]

#![allocator]
#![no_std]

use spin::Mutex;
use linked_list_allocator::Heap;

extern crate spin;
extern crate linked_list_allocator;
#[macro_use]
extern crate lazy_static;

/// Start of heap. Supposedly 0o_000_001_000_000_0000 is the address of the
/// second P3 entry. It does not matter which address we choose as long as
/// it is unused.
pub const HEAP_START: usize = 0o_000_001_000_000_0000;

/// Size of the heap
pub const HEAP_SIZE: usize = 100 * 1024;

// Create a static reference to the heap after
// compile-time using lazy_static
lazy_static! {
    static ref HEAP: Mutex<Heap> = Mutex::new(unsafe {
        Heap::new(HEAP_START, HEAP_SIZE)
    });
}

/// Implementation of rust allocation function.
/// Required implementation in allocator-crates
#[no_mangle]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    HEAP.lock().allocate_first_fit(size, align).expect("out of memory")
}

/// Implementation of rust deallocation function.
/// Required implementation in allocator-crates
#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, size: usize, align: usize) {
    unsafe {
        HEAP.lock().deallocate(ptr, size, align)
    }
}

/// Implementation of rust usable size function.
/// Required implementation in allocator-crates
#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}

/// Implementation of rust reallocate in place function.
/// In order to keep it simple we don't support this function
/// Required implementation in allocator-crates
#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, size: usize,
              _new_size: usize, _align: usize) -> usize {
    size
}

/// Implementation of rust reallocation function.
/// Required implementation in allocator-crates
#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, size: usize, new_size: usize,
                                align: usize) -> *mut u8 {
    use core::{ptr, cmp};
    
    // from: https://github.com/rust-lang/rust/blob/
    //     c66d2380a810c9a2b3dbb4f93a830b101ee49cc2/
    //     src/liballoc_system/lib.rs#L98-L101

    let new_ptr = __rust_allocate(new_size, align);
    unsafe { ptr::copy(ptr, new_ptr, cmp::min(size, new_size)) };
    __rust_deallocate(ptr, size, align);
    new_ptr
}

/// As we do not support unwind as of yet,
/// simply loop to infinity and beyond... 
#[no_mangle]
pub extern fn _Unwind_Resume() {
   loop{}
}
