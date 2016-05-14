/// Basic pipe functionality
/// Idéa is to create a circular fifo buffer with a reader and a writer.
/// Buffer will be created by FrameAllocator, this will make the buffer 
/// 4096 bytres big.
/// Eventuall additions:
/// - Writers and readers can create new buffers.  Eg. different processes
///   could write to their own buffers/pipes.

use memory::FrameAllocator; //Is used to create a frame for the buffer



pub struct Buffer {
    writer: Writer,
    reader: Reader,
    full: bool,
    rdptr: u64,
    wrptr: u64,
    frame: Frame,
    
}


/// A writer that takes an input ASCII (to start with) and writes it to the buffe
impl Writer{
    pub fn new(address: usize, insert: u8, index: u64)
   

}
