/// This module contains Phil Opp's Frame Allocator (with some modifications).

mod area_frame_allocator;
mod paging;


/// Include the `AreaFrameAllocator`
pub use self::area_frame_allocator::AreaFrameAllocator;

/// Include `PhysicalAddress`
use self::paging::PhysicalAddress;
pub use self::paging::{test_paging};
use self::paging::{remap_the_kernel};
use multiboot2::BootInformation;
use acpi::{SDT_Loc};

/// The standard Page/Frame size
pub const PAGE_SIZE: usize = 4096;


/// Initialization of memory and mapping
pub fn init(boot_info: &BootInformation, sdt_loc: &mut SDT_Loc) {
    assert_has_not_been_called!("memory::init must be called only once");

    // Get memory map tag
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    // Load the elf-sections tags from the (now parsed) Multiboot header
    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf-sections tag required");

    // Calculate the start and end addresses of the actual kernel data in RAM.
    // This will be useful for kernel relocation (and memory allocation).
    // Note how we are using map() and anonymous functions _in kernel space_.
    let kernel_start = elf_sections_tag.sections()
        .filter(|s| s.is_allocated()).map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections_tag.sections()
        .filter(|s| s.is_allocated()).map(|s| s.addr + s.size).max()
        .unwrap();

    println!("Kernel start: 0x{:#x}, kernel end: 0x{:#x}",
             kernel_start,
             kernel_end);
    println!("Multiboot start: 0x{:#x}, multiboot end: 0x{:#x}",
             boot_info.start_address(),
             boot_info.end_address());

    // Set up a frame allocator.
    let mut frame_allocator = AreaFrameAllocator::new(
        kernel_start as usize,
        kernel_end as usize,
        boot_info.start_address(),
        boot_info.end_address(),
        memory_map_tag.memory_areas());

    // Remap the kernel
    let mut active_table = paging::remap_the_kernel(
        &mut frame_allocator, boot_info, sdt_loc);


    // TODO: Move apic/SDT page mapping here?


    // ===============================================================
    // Map kernel heap using `active_table`
    // ===============================================================

    use self::paging::Page;
    use hole_list_allocator::{HEAP_START, HEAP_SIZE};
    
    // Get start- and end page of heap
    let heap_start_page = Page::containing_address(HEAP_START);
    let heap_end_page = Page::containing_address(HEAP_START + HEAP_SIZE-1);

    // Map all pages used by heap
    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        active_table.map(page, paging::WRITABLE, &mut frame_allocator);
    }

}

/// The `Frame` is represented by its `number`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    /// Generate a Frame from a given (pointer) address
    fn containing_address(address: usize) -> Frame {
        Frame{ number: address / PAGE_SIZE }
    }
    
    /// Returns Start address
    fn start_address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    }

    /// Does what the trait 'Clone' does, but if implemented like this it remains private. If not prvate like this,  the frame allocator could free the same frame twice and so on.
    fn clone(&self) -> Frame {
        Frame { number: self.number }
    }

    // To map a section we need to iterate over all sections of the frame. 
    fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end,
        }
    }
}

struct FrameIter {
    start: Frame,
    end: Frame,
}

/// The itterator that iterates the sections frames
impl Iterator for FrameIter {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if self.start <= self.end {
            let frame = self.start.clone();
            self.start.number += 1;
            Some(frame)
        } else {
            None
        }
    }
}

/// Define a `FrameAllocator` interface: every `FrameAllocator` can
/// allocate and de-allocate frames (and that's it).
pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

#[test]
fn it_works() {

}
