pub use self::entry::*;
use memory::{PAGE_SIZE, Frame, FrameAllocator}; // needed later
use self::table::{Table, Level4};
use core::ptr::Unique;
use self::paging::PhysicalAddress;

mod entry;
mod table;

const ENTRY_COUNT: usize = 512;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub struct Page {
    number: usize,   
} 

impl Page {
    pub fn containing_adress(address: VirtualAddress) -> Page {
    assert!(address < 0x0000_8000_0000_0000 || address >= 0xffff_8000_0000_0000, 
            "invalid address: 0x{:x}", address);
    Page { number: address / PAGE_SIZE }
    }

    fn start_address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    } 
}

/// Translates a Virtual_Address toa PhysicalAddress
pub fn translate(virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
    let offset = virtual_address % PAGE_SIZE;
    translate_page(Page::containing_address(virtual_address))
        .map(|frame| frame.number * PAGE_SIZE + offset)
}

fn translate_page(page: Page) -> Option<Frame> {
    use self::entry::HUGE_PAGE;

    let p3 = unsafe { &*table::p4 }.next_table(page.p4_index());

    let huge_page = || {
        // TODO
    };
    p3.and_then(|p3| p3.next_table(page.p3_index()))
        .and_then(|p2| p2.next_table(page.p2_index()))
        .and_then(|p1| p1[page.p1_index()].pointed_frame())
        .or_else(huge_page)
}



