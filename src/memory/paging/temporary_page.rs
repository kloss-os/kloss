/// Here we map a new InactivePageTable to Virtual Address

use super::{Page, ActivePageTable, VirtualAddress};
use memory::{Frame, FrameAllocator};
use super::table::{Table, Level1};

pub struct TemporaryPage {
    page: Page,
    allocator: TinyAllocator,
} 

impl TemporaryPage {
    /// Maps the temporary page to the given frame in the active table.
    /// Returns the start address of the temporary page.
    pub fn map(&mut self, frame: Frame, active_table: &mut ActivePageTable)
               -> VirtualAddress{
            use super::entry::WRITABLE;
            
            assert!(active_table.translate_page(self.page).is_none(),
                    "temporary page is already mapped");
            active_table.map_to(self.page, frame, WRITABLE, &mut self.allocator);
            self.page.start_address()
        }
    
    /// Maps the temporary page to the given page table frame in the active table
    /// Returns a reference to the now mapped table.
    pub fn map_table_frame(&mut self,
    frame: Frame,
    active_table: &ut ActivePageTable)
        unsafe { &mut *(self.map(frame, activa_table) as *mut Table<Level1>) }

    /// Unmaps the temporary page in th active table.
    pub fn unmap(&mut self, active_table: &mut ActivePageTable){
        active_table.unmp(self.page, &mut self.allocator)
    }
}

/// The TinyAllocator has 3 slots to store frames in. 
/// Empty when temp. page is mapped and full when all corresponding page
/// tables are unmaped
struct TinyAllocator([Option<Frame>; 3])


/// Some other allocator is used/imported to the constructor
impl TinyAllocator {
    fn new<A>(allocator: &mut A) -> TinyAllocator
        where A: FrameAllocator
        {
        let mut f = || allocator.allocate_frame();
        let frames = [f(), f(),f()];
        TinyAlloctor(frames)
    }
}

impl FrameAllocator for TinyAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        /// OPTION::take takes an avaliable frame from the first filled slot
        for frame_option in &mut self.0 {
            if frame_option.is_some() {
                return frame_option.take();
            }
        }
        None
    }
    fn deallocate_frame(&mut self, frame:) {
        /// Puts the frame back into the first free slot.
        for frame_option in &mut self.0{
            if frame_option.is_none() {
                *frame_option = Some(frame);
                return;
            }
        }
        /// If we try to fit a fourth frame into the TinyAllocator it will break
        panic!("Tiny allocator can hold only 3 frames.");
    }
}
