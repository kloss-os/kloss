/// Basic pipe functionality
/// Idéa is to create a circular fifo buffer with a reader and a writer.
/// Buffer will be created by FrameAllocator, this will make the buffer 
/// 4096 bytes big.
/// Future additions:
/// - Writers and readers can create new buffers.  Eg. different processes
///   could write to their own buffers/pipes.

/*
use memory::FrameAllocator; //Is used to create a frame for the buffer

/// Pointer to Writers location on memory
pub type WriterLocation = usize;
/// Pointer to Readers location on memory
pub type ReaderLoacation = usize;
*/

/// Pointer to Buffer location on memory
pub type BufferLocation = usize;
/// Content in buffer
pub type Content = u8;

/// A circular buffer with fixed size
/// Contains:
/// - Writer that can wrte to it,
/// - Reader that can read from it
/// - Boolean that tells if buffer is full or not, overflow check
/// (- Read pointer (rdptr), the current location of the reader)
/// (- Write pointer (wrptr), the current location of the writer)
pub struct Buffer {
    // writer: Writer, // Traits?
    // reader: Reader, // Traits?
    rdptr: u64, // Maybe, or reader know this.
    wrptr: u64, // Maybe, or writer knows this. 
    //frame: Frame, // Unclear what to use at the moment
}

/// Implementation of buffer
impl Buffer{
    // Constructor
    pub fn new(BufferLocation: BufferLocation, rdptr: u64 = 0, wrptr: u64 = 0) 
               -> Buffer
    
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
        // TODO: Add one to pointer and write to that space  
    }
    
    // Read Content from current +1 reader pointer,
    pub fn read(&self) -> Content {
        assert!(self.is_empty(), "Reading empty space!")
        // TODO: Add one to pointer and read it
    }
}

/// A writer that takes an input ASCII (to start with) and writes it to the buffe
impl Writer{
    // Function enabeling writing to a specific buffer
    pub fn write(Buffer: BufferLocation, insert: Content)
}

/// A reader that extraxts a buffer value
impl Reader{
    // Function for reading from buffer
    pub fn read(Buffer: BufferLocation) -> Content
}
