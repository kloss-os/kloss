use memory::{Frame, FrameAllocator};
use multiboot2::{MemoryAreaIter, MemoryArea};

/// This is a (fairly dumb) `FrameAllocator`, but it keeps track of
/// where the kernel and multiboot sectors are located, and allocates
/// frames linearly.
/// FIXME: There is currently *no way* to deallocate a frame!
pub struct AreaFrameAllocator {
    next_free_frame: Frame,
    current_area: Option<&'static MemoryArea>,
    areas: MemoryAreaIter,
    kernel_start: Frame,
    kernel_end: Frame,
    multiboot_start: Frame,
    multiboot_end: Frame,
}

/// This is the method part of the "object".
impl AreaFrameAllocator {

    /// Constrcutor function. Note how it is the only public function!
    pub fn new(kernel_start: usize, kernel_end: usize,
               multiboot_start: usize, multiboot_end: usize,
               memory_areas: MemoryAreaIter) -> AreaFrameAllocator
    {
        let mut allocator = AreaFrameAllocator {
            next_free_frame : Frame::containing_address(0),
            current_area: None,
            areas: memory_areas,
            kernel_start: Frame::containing_address(kernel_start),
            kernel_end: Frame::containing_address(kernel_end),
            multiboot_start: Frame::containing_address(multiboot_start),
            multiboot_end: Frame::containing_address(multiboot_end),
        };

        // Set up the next pointer.
        allocator.choose_next_area();
        allocator
    }

    /// Private helper function:
    /// Determine the next available area, and return either `Some(area)` or
    /// None, if all frames are occupied. Note the Haskelliness!
    fn choose_next_area(&mut self) {
        self.current_area = self.areas.clone().filter(|area| {
            let address = area.base_addr + area.length -1;
            Frame::containing_address(address as usize) >= self.next_free_frame
        }).min_by_key(|area| area.base_addr);

        if let Some(area) = self.current_area {
            let start_frame = Frame::containing_address(area.base_addr as usize);
            if self.next_free_frame < start_frame {
                self.next_free_frame = start_frame;
            }
        }

    }

    /// Helper function: returns true if `frame` is inside the kernel
    /// area, false otherwise.
    fn frame_in_kernel(&self, frame: &Frame) -> bool {
        return *frame >= self.kernel_start &&
            *frame <= self.kernel_end;
    }

    /// Helper function: returns true if `frame` is inside the multiboot
    /// area, false otherwise.
    fn frame_in_multiboot(&self, frame: &Frame) -> bool {
        return *frame >= self.multiboot_start &&
            *frame <= self.multiboot_end;
    }
}

/// Implementations for the `FrameAllocator` interface for
/// `AreaFrameAllocator`
impl FrameAllocator for AreaFrameAllocator {


    /// Allocate a frame. Uses an optimistic approach, where the
    /// algorithm tries to allocate a frame from the beginning to the
    /// end, and determines if it failed afterwards.
    fn allocate_frame(&mut self) -> Option<Frame> {
        if let Some(area) = self.current_area {
            // If the frame is free, return it by constructing an
            // identical frame.
            let frame = Frame{ number: self.next_free_frame.number };

            let current_area_last_frame = {
                let address = area.base_addr + area.length - 1;
                Frame::containing_address(address as usize)
            };

            // Now for everything that could go wrong with this
            // optimistic update:

            if frame > current_area_last_frame {
                // all frames in the current area are in use, switch to
                // the next area.
                self.choose_next_area();

            } else if self.frame_in_kernel(&frame) {
                // `frame` is used by the kernel
                self.next_free_frame = Frame {
                    number: self.kernel_end.number +1
                };

            } else if self.frame_in_multiboot(&frame) {
                // `frame`  is used by the multiboot info structure
                self.next_free_frame = Frame {
                    number: self.multiboot_end.number + 1
                };

            // End of things that could go wrong
            } else {
                // `frame` is unused, increment `next_free_frame` and return it.
                self.next_free_frame.number += 1;
                return Some(frame);
            }

            // `frame` was not valid, try it again with the updated
            // `next_free_frame`
            self.allocate_frame()
        } else {
            None // no frames left
        }
    }

    /// FIXME: Not implemented!
    fn deallocate_frame(&mut self, _frame: Frame) {
        unimplemented!()
    }

}
