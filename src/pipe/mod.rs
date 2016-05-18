/// Box implemenatations an memory stuffs
use alloc::boxed::Box;

/// Content in buffer
pub type Content = u8;

/// BUFFER_SIZE is set to a low number
const BUFFER_SIZE: usize = 128;
/// START_POINTER is kind of a test thing since usize would not work as intended
const START_POINT: usize = 0;

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

    // Steps writer one step forward, circle aspect included
    pub fn step_wp(&mut self){
        if self.wp == BUFFER_SIZE && self.rp != START_POINT{
            self.wp = 0; 
        } else {
            self.wp += 1; 
        }
    }

    // Steps reader one step forward, circle aspect included 
    pub fn step_rp(&mut self){
        if self.rp == BUFFER_SIZE && self.wp != START_POINT{
            self.rp = 0; 
        } else {
            self.rp += 1; 
        }
    }

    // Write Content to current +1 writerpointer address. Check if full. 
    pub fn write(&mut self, insert: Content){
        assert!(!self.is_full(), "Buffer is full!");
        self.step_wp();
        self.buf[self.wp] = insert;

    }

    // Read Content from current +1 reader pointer,
    pub fn read(&mut self) -> Content {
        assert!(!self.is_empty(), "Reading empty space!");
        self.step_rp();
        //self.rp = self.rp +1;
        self.buf[self.rp]
    }

//    pub fn unread(&mut self) -> bool {
//        self.rp < self.wp
//    }
}

/// Implements iterator for buffer, lets you process all unread data.
impl Iterator for Buffer {
    type Item = Content;
    fn next(&mut self) -> Option<Content> {
        if self.is_empty() {
            None
        } else {
            Some(self.read())
        }
    }
}


#[test]
fn buffer_add(){
    assert!(false);
}

