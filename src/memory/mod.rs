/// This module contains Phil Opp's Frame Allocator (with some modifications).


/// Include the `AreaFrameAllocator`
pub use self::area_frame_allocator::AreaFrameAllocator;

/// Include `PhysicalAddress`
use self::paging::PhysicalAddress;
pub use self::paging::{test_paging, remap_the_kernel};

mod area_frame_allocator;
mod paging;
/// The standard Page/Frame size
pub const PAGE_SIZE: usize = 4096;

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
