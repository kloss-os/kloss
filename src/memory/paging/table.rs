/// Transfered from Phil Opp's guide

use memory::paging::entry::*;
use memory::paging::ENTRY_COUNT;
use memory::FrameAllocator;
use core::ops::{Index, IndexMut};
use core::marker::PhantomData;

/// Constant that lets us access pagetables
pub const P4: *mut Table<Level4> = 0xffffffff_fffff000 as *mut _;


pub struct Table<L: TableLevel> {
    /// ENTRY_COUNT is an array of 512 page table entries
    entries: [Entry; ENTRY_COUNT],
    level: PhantomData<L>,
}

/// Sets all entrys as unused
impl<L> Table<L> where L: TableLevel
{
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }
}
/// 
impl<L> Table<L> where L: HierarchicalLevel {
    /// Gives the address in the page one level down in the hierarchy. Only usable for p4, P3 and P2.
    fn next_table_address(&self, index: usize) -> Option<usize> {
        let entry_flags = self[index].flags();
        if entry_flags.contains(PRESENT) && !entry_flags.contains(HUGE_PAGE) {
            let table_address = self as *const _ as usize;
            Some((table_address << 9) | (index << 12))
        } else {
            None
        }
    }
    /// We convert the address into raw pointers through as casts and then convert them into Rust references through &mut *
    pub fn next_table<'a>(&self, index: usize) -> Option<&Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe { &*(address as *const _) })
    }
    /// We convert the address into raw pointers through as casts and then convert them into Rust references through &mut *
    pub fn next_table_mut(&mut self, index: usize) -> Option<&mut Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe { &mut *(address as *mut _)})
    }

    /// Creates a table and sets flags to Present and Writable, if flagged ass HUGE_PAGE error will occure since they are not supported for this kind of pageing.
    pub fn next_table_create<A>(&mut self,
                                index: usize,
                                allocator: &mut A)
                                -> &mut Table<L::NextLevel>
        where A: FrameAllocator
    {
        /// If next table returns none, an assert checks for HUGE_PAGE flag
        if self.next_table(index).is_none() {
            assert!(!self.entries[index].flags().contains(HUGE_PAGE),
                   "mapping code does not support huge pages");
            let frame = allocator.allocate_frame().expect("no frames available");
            self.entries[index].set(frame, PRESENT | WRITABLE);
            self.next_table_mut(index).unwrap().zero();
        }   
        self.next_table_mut(index).unwrap()
    }

}

/// Lets us get the entry for some_table[42]
impl<L> Index<usize> for Table<L> where L: TableLevel {
    type Output = Entry;

    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

/// Lets us get the entry for some_table[42]
impl<L> IndexMut<usize> for Table<L> where L: TableLevel {
    fn index_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }
}

/// Enums for page table levels to enable security, e.g. we don't want to go deeper than Page_Table_1, but 4->3, 3->2 and 2->1 is ok. 
pub trait TableLevel {}

pub enum Level4{}
pub enum Level3{}
pub enum Level2{}
pub enum Level1{}

impl TableLevel for Level4 {}
impl TableLevel for Level3 {}
impl TableLevel for Level2 {}
impl TableLevel for Level1 {}

/// Describes each layer/level of hierarchy in the page table. To differentiate P1 from the rest.
trait HierarchicalLevel: TableLevel {
    type NextLevel: TableLevel;
}

impl HierarchicalLevel for Level4 {
    type NextLevel = Level3;
}

impl HierarchicalLevel for Level3 {
    type NextLevel = Level2;
}

impl HierarchicalLevel for Level2 {
    type NextLevel = Level1;
}
