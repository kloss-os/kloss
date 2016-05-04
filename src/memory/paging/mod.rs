/// Purpose of these submudules is to create a recursive page table hierarchy
/// with four levels of pagetables in it.
/// To acomplish this a enum Hierarchy to differentiate between the top three
/// levels and the fourth. As a result of this we can extract the addresses stored
/// in the first three levels then jump to the last level (level 1) retrive
/// its address and then move to the offsets.

pub use self::entry::*;

///FrameAllocator is the method used to create Frames from Pages
use memory::{PAGE_SIZE, Frame, FrameAllocator}; // needed later
use self::table::{Table, Level4};
pub use self::mapper::Mapper;
use core::ptr::Unique;
use core::ops::{Deref, DerefMut};
use self::temporary_page::TemporaryPage;
use multiboot2::BootInformation;

//use self::paging::PhysicalAddress;
//use self::entry::HUGE_PAGE;

mod entry;
mod table;
mod mapper;
///Used to temporary map a frame to virtyal address
mod temporary_page;

const ENTRY_COUNT: usize = 512;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;


/// Copy so that it can be used after  passing 'map_to' and similar functions.
#[derive(Debug, Clone, Copy)]
pub struct Page {
    number: usize,
}


impl Page {
    pub fn containing_address(address: VirtualAddress) -> Page {
    assert!(address < 0x0000_8000_0000_0000 || address >= 0xffff_8000_0000_0000,
            "invalid address: 0x{:x}", address);
        Page { number: address / PAGE_SIZE }
    }

    /// Takes a VirtualAddress and calculates start address
    fn start_address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    }

    /// Calculates the p4-starting index/point
    fn p4_index(&self) -> usize {
        (self.number >> 27) & 0o777
    }

    /// Calculates the p3-starting index/point
    fn p3_index(&self) -> usize {
        (self.number >> 18) & 0o777
    }

    /// Calculates the p2-starting index/point
    fn p2_index(&self) -> usize {
        (self.number >> 9) & 0o777
    }

    /// Calculates the p1-starting index/point
    fn p1_index(&self) -> usize {
        (self.number >> 0) & 0o777
    }


}

pub struct ActivePageTable {
    mapper: Mapper,
}


impl Deref for ActivePageTable {
    type Target = Mapper;

    fn deref(&self) -> &Mapper {
        &self.mapper
    }
}


impl DerefMut for ActivePageTable {
    fn deref_mut(&mut self) -> &mut Mapper {
        &mut self.mapper
    }
}

impl ActivePageTable {
    unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            mapper: Mapper::new(),
        }
    }
    pub fn with<F>(&mut self,
                   table: &mut InactivePageTable,
                   f: F)
        where F: FnOnce(&mut Mapper)
    {
        use x86::{controlregs, tlb};
        let flush_tlb = || unsafe { tlb::flush_all() };
        
        {
            let backup = Frame::containing_address (
                unsafe { controlregs::cr3() } as usize);
            
            // map temporary_page to current p4 table
            let p4_table = temporary_page.map_table_frame(backup.clone(), self);
            
            // overwrite recursive mapping
            self.p4_mut()[511].set(table.p4_frame.clone(), PRESENT | WRITABLE);
            flush_tlb();
            
            // execute f in the new context
            f(self);
            
            // restore recursive mapping to original p4 table
            p4_table[511].set(backup, PRESENT | WRITABLE);
            flush_tlb();
        }
        
        temporary_page.unmap(self);
    }
    
    pub fn switch(&mut self, new_table: InactivePageTable) -> InactivePageTable {
        use x86::controlregs;
        
        let old_table = InactivePageTable {
            p4_frame: Frame::containing_address(unsafe { controlregs::cr3() } as usize),
        };
        unsafe {
            controlregs::cr3_write(new_table.p4_frame.start_address() as u64);
        }
        old_table
    }
}




pub struct InactivePageTable {
    p4_frame: Frame,
}
/// Creates valid, inactive page tables that are zeroed and recursively maped.
impl InactivePageTable {
    pub fn new(frame: Frame,
               active_table: &mut ActivePageTable,
               temporary_page: &mut TemporaryPage)
               -> InactivePageTable {
        {
            
            // The 'active_table' and 'temporary_table' arguments needs to 
            // be in a inner scope to ensure shadowing since the table 
            // variable exclusively borrows temporary_page as long as it's alive
            let table = temporary_page.map_table_frame(frame.clone(),
            active_table);
            // Zeroing table is done here *duh*
            table.zero();
            // Recursive mapping for the table
            table[511].set(frame.clone(), PRESENT | WRITABLE);
        }
        temporary_page.unmap(active_table);

        InactivePageTable {p4_frame: frame } 
    }
}

pub fn remap_the_kernel<A>(allocator: &mut A, boot_info: &BootInformation)
    where A: FrameAllocator
{
    //
    let mut temporary_page = TemporaryPage::new(Page { number: 0xcafebabe },
                                                allocator);
    let mut active_table = unsafe { ActivePageTable::new() };
    let mut new_table = {
        let frame = allocator.allocate_frame().expect("no more frames");
        InactivePageTable::new(frame, &mut active_table, &mut temporary_page)
    };
    active_table.with(&mut new_table, &mut temporary_page, |mapper| {
        let elf_sections_tag = boot_info.elf_sections_tag()
            .expect("Memmort map tag required");
        for section in elf_sections_tag.sections() {
            //TODO mapper.identity_map() all pages of 'section'
        }
    });
}





/// Basic tresting of different page table levels and allocations as well as mapping specific bits in specific levels
pub fn test_paging<A>(allocator: &mut A)
    where A: FrameAllocator
{
        let mut page_table = unsafe { ActivePageTable::new() };

    // test translate
    println!("Some = {:?}", page_table.translate(0));
    println!("Some = {:?}", page_table.translate(4096)); // second P1 entry
    println!("Some = {:?}", page_table.translate(512 * 4096)); // second P2 entry
    println!("Some = {:?}", page_table.translate(300 * 512 * 4096)); // 300th P2 entry
    println!("None = {:?}", page_table.translate(512 * 512 * 4096)); // second P3 entry
    println!("Some = {:?}", page_table.translate(512 * 512 * 4096 - 1)); // last mapped byte

    // test map_to
    let addr = 42 * 512 * 512 * 4096; // 42th P3 entry
    let page = Page::containing_address(addr);
    let frame = allocator.allocate_frame().expect("no more frames");
    println!("None = {:?}, map to {:?}",
             page_table.translate(addr),
             frame);
    page_table.map_to(page, frame, EntryFlags::empty(), allocator);
    println!("Some = {:?}", page_table.translate(addr));
    println!("next free frame: {:?}", allocator.allocate_frame());

    // test unmap
    println!("{:#x}",
             unsafe { *(Page::containing_address(addr).start_address() as *const u64) });
    page_table.unmap(Page::containing_address(addr), allocator);
    println!("None = {:?}", page_table.translate(addr));
}


/*
/// A struct to save the first pice of the address.
pub struct RecursivePageTable {
    p4: Unique<Table<Level4>>,
}

/// In order to find the complete address we implement a way to go through four levels of
/// page-tables in order to retrive four pices of the address.
impl RecursivePageTable {
    pub unsafe fn new() -> RecursivePageTable {
        RecursivePageTable { p4: Unique::new(table::P4) }
    }
    // This overwrites the 511th P4 entry and points it to the inactive table frame. and then flushes the TLB.
    pub fn with<F>(&mut self,
                   table: &mut InactivePageTable,
                   f: F)
        where F: FnOnce(&mut RecursivePageTable)
    {
        use x86::tlb;
        let flush_tlb = || unsafe { tlb::flush_all() };
        
        // overwrite recursive mapping
        self.p4_mut()[511].set(table.p4_frame.clone(), PRESENT |WRITABLE);
        flush_tlb();
        // execute f in the new context
        f(self);
        
        //TODO restore recursive mapping to original p4 table
    }

    fn p4(&self) -> &Table<Level4>{
        unsafe { self.p4.get() }
    }

    fn p4_mut(&mut self) -> &mut Table<Level4> {
        unsafe {self.p4.get_mut() }
    }


/// Translates a Virtual_Address to a corresponding PhysicalAddress
/// Returns 'None' if the address is not mapped
    pub fn translate(&self, virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
        let offset = virtual_address % PAGE_SIZE;
        self.translate_page(Page::containing_address(virtual_address))
            .map(|frame| frame.number * PAGE_SIZE + offset)
    }
    /// The page in the next level is saved in P3 and found through .next_table
    /// using the index where P4 was found.
    fn translate_page(&self, page: Page) -> Option<Frame> {
        let  p3 = self.p4().next_table(page.p4_index());
        // If page is flagged as HUGE_FRAME,
        let huge_page = || {
            p3.and_then(|p3| {
                let p3_entry= &p3[page.p3_index()]; //1GiB page?
                if let Some(start_frame) = p3_entry.pointed_frame() {
                    if p3_entry.flags().contains(HUGE_PAGE) {
                        // address must be 1GiB aligned
                        assert!(start_frame.number % (ENTRY_COUNT * ENTRY_COUNT) == 0);
                        return Some(Frame {
                            number: start_frame.number +
                                page.p2_index() * ENTRY_COUNT +
                                page.p1_index(),
                        });
                    }
                }
                if let Some(p2) = p3.next_table(page.p3_index()) {
                    let p2_entry = &p2[page.p2_index()];
                    //2MiB page?
                    if let Some(start_frame) = p2_entry.pointed_frame() {
                        if p2_entry.flags().contains(HUGE_PAGE) {
                            //address must be 2MiB aligned
                            assert!(start_frame.number % ENTRY_COUNT == 0);
                            return Some(Frame { number: start_frame.number +
                                                page.p1_index()});
                        }
                    }

                }
                None
            })
        };
        p3.and_then(|p3| p3.next_table(page.p3_index()))
            .and_then(|p2| p2.next_table(page.p2_index()))
            .and_then(|p1| p1[page.p1_index()].pointed_frame())
            .or_else(huge_page)

    }

    /// Maps the page to the frame with the provided flags.
    /// The `PRESENT` flag is added by default. Needs a
    /// `FrameAllocator` as it might need to create new page tables.
    pub fn map_to<A>(&mut self,
                     page: Page,
                     frame: Frame,
                     flags: EntryFlags,
                     allocator: &mut A)
        where A: FrameAllocator
    {
        let mut p3 = self.p4_mut().next_table_create(page.p4_index(), allocator);
        let mut p2 = p3.next_table_create(page.p3_index(), allocator);
        let mut p1 = p2.next_table_create(page.p2_index(), allocator);

        assert!(p1[page.p1_index()].is_unused());
        p1[page.p1_index()].set(frame, flags | PRESENT);
    }

    /// Maps the page to some free frame with the provided flags.
    /// The free frame is allocated from the given `FrameAllocator`.
    pub fn map<A>(&mut self,
                  page: Page,
                  flags: EntryFlags,
                  allocator: &mut A)
        where A: FrameAllocator
    {
        let frame = allocator.allocate_frame().expect("out of memory");
        self.map_to(page, frame, flags, allocator)
    }

    /// Identity map the the given frame with the provided flags.
    /// The `FrameAllocator` is used to create new page tables if needed.
    pub fn identity_map<A>(&mut self,
                           frame: Frame,
                           flags: EntryFlags,
                           allocator: &mut A)
        where A: FrameAllocator
    {
        let page = Page::containing_address(frame.start_address());
        self.map_to(page, frame, flags, allocator)
    }

    /// Unmaps the given page and adds all freed frames to the given
    /// `FrameAllocator`.
    fn unmap<A>(&mut self, page: Page, allocator: &mut A)
        where A: FrameAllocator
    {
        assert!(self.translate(page.start_address()).is_some());

        let p1 = self.p4_mut()
            .next_table_mut(page.p4_index())
            .and_then(|p3| p3.next_table_mut(page.p3_index()))
            .and_then(|p2| p2.next_table_mut(page.p2_index()))
            .expect("mapping code does not support huge pages");
        let frame = p1[page.p1_index()].pointed_frame().unwrap();
        p1[page.p1_index()].set_unused();
        unsafe { ::x86::tlb::flush(page.start_address()) };
        // TODO free p(1,2,3) table if empty
        // allocator.deallocate_frame(frame);

    }

}
*/

// InactivePageTable owns a P4 table, just as the RecursivePageTable does, but is not used by the CPU
