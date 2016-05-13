#![feature(const_fn)]
#![feature(allocator)]

#![allocator]
#![no_std]

use spin::Mutex;

extern crate spin;

/// Start of heap. Supposedly 0o_000_001_000_000_0000 is the address of the
/// second P3 entry. It does not matter which address we choose as long as
/// it is unused.
pub const HEAP_START: usize = 0o_000_001_000_000_0000;

/// Size of heap
pub const HEAP_SIZE:  usize = 100 * 1024; // 100 KiB

/// Create static bump allocator for use with rust-specific
/// allocator functions.
static BUMP_ALLOCATOR: Mutex<BumpAllocator> = Mutex::new(
    BumpAllocator::new(HEAP_START, HEAP_SIZE));

#[derive(Debug)]
struct BumpAllocator {
    heap_start: usize,
    heap_size: usize,
    next: usize
} 

impl BumpAllocator {
    /// Create new allocator, using memory in the range
    /// [heap_start, heap_start + heap_size]
    const fn new(heap_start: usize, heap_size: usize) -> BumpAllocator {
        BumpAllocator {
            heap_start: heap_start,
            heap_size: heap_size,
            next: heap_start
        }
    }

    /// Allocates a block of memory with the given usize and alignment
    fn allocate(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        let alloc_start = align_up(self.next, align);
        let alloc_end = alloc_start + size;

        if alloc_end <= self.heap_start + self.heap_size {
            self.next = alloc_end;
            Some(alloc_start as *mut u8)
        } else {
            None
        }
    }
}

/// Align downwards. Returns the greatest x with alignment `align`
/// so that x <= addr. The alignment must be power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {
    if align.is_power_of_two() {
        addr & !(align - 1)
    } else if align == 0 {
        addr
    } else {
        panic!("`align` must be power of 2");
    }
}

/// Align upwards. Returns the smallest x with alignment `align`
/// so that x >= addr. The alignment must be a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
    align_down(addr + align - 1, align)
}


/// Implementation of rust allocation function.
/// Required implementation in allocator-crates
#[no_mangle]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    // We use expect to panic in case of 'out of memory'
    BUMP_ALLOCATOR.lock().allocate(size, align).expect("out of memory")
}


/// Implementation of rust deallocation function.
/// Required implementation in allocator-crates
#[no_mangle]
pub extern fn __rust_deallocate(_ptr: *mut u8, _size: usize, _align: usize) {
    // just leak it
}

/// Implementation of rust usable size function.
/// Required implementation in allocator-crates
#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    // We never allocate more memory, so return `size`
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
pub extern fn __rust_reallocate(ptr: *mut u8, size: usize, 
                                new_size: usize, align: usize) -> *mut u8 {
    use core::{ptr, cmp};

    // from: https://github.com/rust-lang/rust/blob/
    //     c66d2380a810c9a2b3dbb4f93a830b101ee49cc2/
    //     src/liballoc_system/lib.rs#L98-L101

    let new_ptr = __rust_allocate(new_size, align);
    unsafe { ptr::copy(ptr, new_ptr, cmp::min(size, new_size)) };
    __rust_deallocate(ptr, size, align);
    new_ptr
}
