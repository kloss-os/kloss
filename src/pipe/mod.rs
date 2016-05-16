/// Box implemenatations an memory stuffs
use core::mem;

/// Content in buffer
pub type Content = u8;


const BUFFER_SIZE = 5;

/// A circular buffer with fixed size
/// Contains:
/// - Read pointer (rdptr), the current location of the reader
/// - Write pointer (wrptr), the current location of the writer
pub struct Buffer {
    wrptr: u8,
    rdptr: u8,
    buffer: [u8; BUFFER_SIZE],
}

pub fn box_buffer
/// Implementation of buffer
impl Buffer{
    // Constructor
    pub fn new()-> Box<Buffer> {
        Box::new(Buffer{
            wrptr: 0,
            rdptr: 0,
            buffer: [0; BUFFER_SIZE]
        })
    }
    
    // Check if writer +1 is equal to reader, returns boolean  
    pub fn is_full(&self) -> Bool{
         (self.wrptr+1) == self.rdptr 
    }
    
    // Check if writer and reader are at the same position,
    // if so buffer should be empty.
    pub fn is_empty(&self) -> Bool{
        self.rdptr == self.wrptr 
    }

    // Write Content to current +1 writerpointer address. Check if full. 
    pub fn write(&self, insert: Content){
        assert!(self.is_full(), "Buffer is full!");
        self.buffer[wrptr + 1] = Content;
        self.wrptr = (wrptr + 1);
    }
    
    // Read Content from current +1 reader pointer,
    pub fn read(&self) -> Content {
        assert!(self.is_empty(), "Reading empty space!");
        self.rdptr = (rdptr +1);
        self.buffer[rdptr];
    }
}
