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

//use acpi::rsdp;
use acpi::*;
use core::mem;
use core::option;

//use self::paging::PhysicalAddress;
//use self::entry::HUGE_PAGE;

mod entry;
mod table;
mod mapper;

///Used to temporary map a frame to virtal address
mod temporary_page;

const ENTRY_COUNT: usize = 512;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;


/// Copy so that it can be used after  passing 'map_to' and similar functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page {
    number: usize,
}


impl Page {
    /// The address space is split into two halves , high/low, where the higher
    /// half contains addresses and the sign extentions, the lower half contains
    /// just adresses. This is checked here with an assert.
    pub fn containing_address(address: VirtualAddress) -> Page {
    assert!(address < 0x0000_8000_0000_0000 ||
            address >= 0xffff_8000_0000_0000,
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

    /// Returns inclusive range iterator of pages
    pub fn range_inclusive(start: Page, end: Page) -> PageIter {
        PageIter {
            start: start,
            end: end
        }
    }
}

pub struct PageIter {
    start: Page,
    end: Page
}

impl Iterator for PageIter {
    type Item = Page;

    fn next(&mut self) -> Option<Page> {
        if self.start <= self.end {
            let page = self.start;
            self.start.number += 1;
            Some(page)
        } else {
            None
        }
    }
}


pub struct ActivePageTable {
    mapper: Mapper,
}

/// Dereference the ActivePageTable
/// Returns reference to Mapper
impl Deref for ActivePageTable {
    type Target = Mapper;

    fn deref(&self) -> &Mapper {
        &self.mapper
    }
}

/// Dereference the ActivePageTable
/// Returns a mutable reference to Mapper
impl DerefMut for ActivePageTable {
    fn deref_mut(&mut self) -> &mut Mapper {
        &mut self.mapper
    }
}

/// Does the recursive mapping to the four levels of page tables.
impl ActivePageTable {
    unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            mapper: Mapper::new(),
        }
    }
    /// Module that temporarily changes the recursive mapping.
    /// It overwrites the 511th P4 entry and points it to the
    /// inactive table frame.
    /// "It overwrites the 511th P4 entry and points it to the inactive table frame. Then it flushes the translation lookaside buffer (TLB), which still contains some old translations. We need to flush all pages that are part of the recursive mapping, so the easiest way is to flush the TLB completely.""
    pub fn with<F>(&mut self,
                   table: &mut InactivePageTable,
                   temporary_page: &mut temporary_page::TemporaryPage,
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
            p4_frame: Frame::containing_address(
                unsafe { controlregs::cr3() } as usize),
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
/// Creates valid, inactive page tables that are zeroed and recursively mapped.
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
/// Remaps the kernel sections by creating a temporary page.
pub fn remap_the_kernel<A>(allocator: &mut A, boot_info: &BootInformation, sdt_loc: &mut SDT_Loc)
    -> ActivePageTable
    where A: FrameAllocator{
    use core::ops::Range;
    // Create a temporary page at some page number, in this case 0xcafebabe
    let mut temporary_page =
        TemporaryPage::new(Page { number: 0xcafebabe }, allocator);
    // Created by constructor
    let mut active_table = unsafe { ActivePageTable::new() };
    // Created by constructor
    let mut new_table = {
        let frame = allocator.allocate_frame().expect("no more frames");
        InactivePageTable::new(frame, &mut active_table, &mut temporary_page)
    };

    active_table.with(&mut new_table, &mut temporary_page, |mapper| {
        let elf_sections_tag = boot_info.elf_sections_tag()
            .expect("Memory map tag required");
        // Identity map the allocated kernel sections
        // Skip sections that are not loaded to memory.
        // We require pages to be aligned, see src/arch/x86_64/linker.ld for implementations
        for section in elf_sections_tag.sections() {
            if !section.is_allocated() {
                // section is not loaded to memory
                continue;
            }

            assert!(section.addr as usize % PAGE_SIZE == 0,
                    "sections need to be page aligned");
            // println!("mapping section at addr: {:#x}, size: {:#x}",
            //          section.addr,
            //          section.size);

            let flags = EntryFlags::from_elf_section_flags(section);

            let start_frame = Frame::containing_address(section.start_address());
            let end_frame = Frame::containing_address(section.end_address() - 1);
            // 'range_inclusive' iterates over all frames on a section
            for frame in Frame::range_inclusive(start_frame, end_frame) {
                mapper.identity_map(frame, flags, allocator);
            }
        }

        // identity map the VGA text buffer
        let vga_buffer_frame = Frame::containing_address(0xb8000);
        mapper.identity_map(vga_buffer_frame, WRITABLE, allocator);

        // identity map the multiboot info structure
        let multiboot_start =
            Frame::containing_address(boot_info.start_address());
        let multiboot_end =
            Frame::containing_address(boot_info.end_address() - 1);
        for frame in Frame::range_inclusive(multiboot_start, multiboot_end) {
            mapper.identity_map(frame, PRESENT, allocator);
        }




        for (start, end, next) in &mut sdt_loc.into_iter() {
            //println!("Allocating addresses {:x} to {:x} and {:x}", start, end, next);
            let start_addr = Frame::containing_address(start);
            let end_addr = Frame::containing_address(end);
            for frame in Frame::range_inclusive(start_addr, end_addr) {
                if mapper.is_unused(&frame, allocator) {
                    mapper.identity_map(frame, PRESENT, allocator);
                }
            }

            if next != 0 {
                let next_header_frame = Frame::containing_address(next);
                if mapper.is_unused(&next_header_frame, allocator) {
                    mapper.identity_map(next_header_frame, PRESENT, allocator);
                }
            }
        }

        let ioapic_start = Frame::containing_address(sdt_loc.ioapic_start);
        let ioapic_end = Frame::containing_address(sdt_loc.ioapic_end);
        for frame in Frame::range_inclusive(ioapic_start, ioapic_end) {
            if mapper.is_unused(&frame, allocator) {
                mapper.identity_map(frame, WRITABLE, allocator);
            }
        }

        let lapic_addr = Frame::containing_address(sdt_loc.lapic_ctrl);
        if mapper.is_unused(&lapic_addr, allocator) {
            mapper.identity_map(lapic_addr, WRITABLE, allocator);
        }



    });
    // TODO: Delete when appropriate
    let old_table = active_table.switch(new_table);
    //println!("NEW TABLE!!!");

    // TODO: Delete when appropriate
    let old_p4_page = Page::containing_address(old_table.p4_frame.start_address());
    active_table.unmap(old_p4_page, allocator);
    //println!("guard page at {:#x}", old_p4_page.start_address());

    active_table
}






/// Basic tresting of different page table levels and allocations as well as mapping specific bits in specific levels
pub fn test_paging<A>(allocator: &mut A)
    where A: FrameAllocator
{
        let mut page_table = unsafe { ActivePageTable::new() };

    // test translate
    println!("Some = {:?}", page_table.translate(0));
    // second P1 entry
    println!("Some = {:?}", page_table.translate(4096));
     // second P2 entry
    println!("Some = {:?}", page_table.translate(512 * 4096));
    // 300th P2 entry
    println!("Some = {:?}", page_table.translate(300 * 512 * 4096));
    // second P3 entry
    println!("None = {:?}", page_table.translate(512 * 512 * 4096));
    // last mapped byte
    println!("Some = {:?}", page_table.translate(512 * 512 * 4096 - 1));

    // test map_to

    // 42th P3 entry
    let addr = 42 * 512 * 512 * 4096;
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
