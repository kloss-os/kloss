/// Box implemenatations an memory stuffs
use alloc::boxed::Box;

/// Content in buffer
pub type Content = u8;

/// BUFFER_SIZE is set to a low number
const BUFFER_SIZE: usize = 5;

/// A circular buffer with fixed size [5]
/// Contains:
/// - Read pointer (rp), the current location of the reader
/// - Write pointer (wp), the current location of the writer
/// - Buffer (buf), list of BUFFER_SIZE, containing elements of size u8
pub struct Buffer {
    wp: usize,
    rp: usize,
    buf: [u8; BUFFER_SIZE],
}

/// Implementation of buffer
impl Buffer{
    // Allocates a new Buffer with two pointers, everything set to 0.
    pub fn new()-> Box<Buffer> {
        Box::new(Buffer{
            wp: 0,
            rp: 0,
            buf: [0; BUFFER_SIZE]
        })
    }
    
    // Check if writer +1 is equal to reader, returns boolean  
    pub fn is_full(&self) -> bool{
         (self.wp+1) == self.rp 
    }
    
    // Check if writer and reader are at the same position,
    // if so buffer should be empty.
    pub fn is_empty(&self) -> bool{
        self.rp == self.wp
    }

    // Write Content to current +1 writerpointer address. Check if full. 
    pub fn write(&mut self, insert: Content){
        assert!(!self.is_full(), "Buffer is full!");
        self.wp = (self.wp +1);
        self.buf[self.wp] = insert;

    }
    
    // Read Content from current +1 reader pointer,
    pub fn read(&mut self) -> Content {
        assert!(!self.is_empty(), "Reading empty space!");
        self.rp = (self.rp +1);
        self.buf[self.rp]
    }
}
