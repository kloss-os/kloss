/// This module contains Phil Opp's Frame Allocator (with some modifications).


/// Include the `AreaFrameAllocator`
pub use self::area_frame_allocator::AreaFrameAllocator;

mod area_frame_allocator;

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
