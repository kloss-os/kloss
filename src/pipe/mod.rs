/// Box implemenatations an memory stuffs
use alloc::boxed::Box;

/// Content in buffer
pub type Content = u8;

/// BUFFER_SIZE is set to a low number
const BUFFER_SIZE: usize = 5;
/// START_POINTER is kind of a test thing since usize would not work as intended
const START_POINT: usize = 0;

/// A circular buffer with fixed size [5]
/// Contains:
/// - Read pointer (rp), the current location of the reader
/// - Write pointer (wp), the current location of the writer
/// - Buffer (buf), list of BUFFER_SIZE, containing elements of size u8
#[cfg(not(test))]
pub struct Buffer {
    wp: usize,
    rp: usize,
    buf: [u8; BUFFER_SIZE],
}
#[cfg(test)]
pub struct Buffer {
    pub wp: usize,
    pub rp: usize,
    pub buf: [u8; BUFFER_SIZE],
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
         (self.wp+1) % BUFFER_SIZE == self.rp 
    }
    
    // Check if writer and reader are at the same position,
    // if so buffer should be empty.
    pub fn is_empty(&self) -> bool{
        self.rp == self.wp
    }

    // Steps writer one step forward, circle aspect included
    pub fn step_wp(&mut self){
        if self.wp == BUFFER_SIZE { //&& self.rp != START_POINT{
            self.wp = START_POINT; 
        } else {
            self.wp += 1; 
        }
    }

    // Steps reader one step forward, circle aspect included 
    pub fn step_rp(&mut self){
        if self.rp == BUFFER_SIZE{ // && self.wp != START_POINT{
            self.rp = START_POINT; 
        } else {
            self.rp += 1; 
        }
    }

    // Write Content to current +1 writerpointer address. Check if full. 
    pub fn write(&mut self, insert: Content){
        assert!(!self.is_full(), "Buffer is full!");
        self.buf[self.wp] = insert;
        self.step_wp(); 
    }
    
    // Read Content from current +1 reader pointer,
    pub fn read(&mut self) -> Content {
        assert!(!self.is_empty(), "Reading empty space!");
        let content = self.buf[self.rp];
        self.step_rp();
        return content;
    }
}

#[test]
/// Test pointers set to zero
fn buffer_zero_ptr(){
    let mut b = Buffer::new();
    assert!(b.wp == 0 && b.rp == 0);
}

#[test]
/// Add 42, read and expect 42.
fn buffer_add_read(){
    let mut b = Buffer::new();
    b.write(42);
    let first = b.read();
    assert!(first == 42);
}

#[test]
/// Test pointers after one write and read
fn buffer_pointers_after_one_read(){
    let mut b = Buffer::new();
    b.write(1);
    let first = b.read();
    assert!(b.wr == 1 && b.rp == 2);
}

#[test]
#[should_panic]
/// Test to read first, should panic
fn buffer_read_first(){
    let mut b = Buffer::new();
    let first = b.read(); // Should result in a buffer_empty panic
}

#[test]
/// Test is_empty
fn buffer_is_empty(){
    let mut b = Buffer::new();
    assert!(b.is_empty() == true);
}

#[test]
#[should_panic]
/// Add 42, read twice, should fail if working properly
fn buffer_add_once_read_twice(){
    let mut b = Buffer::new();
    b.write(42);
    let first = b.read();
    let second = b.read();  
}

#[test]
#[should_panic]
/// Add five times, should fail if working properly.
fn buffer_add_five_times(){
    let mut b = Buffer::new();
    b.write(1); // wp == 1, rp == 0 
    b.write(2); // wp == 2, rp == 0
    b.write(3); // wp == 3, rp == 0
    b.write(4); // wp == 4, rp == 0 
    b.write(5); // wp == 0, rp == 0 <- Should fail here since 4+1 % 5 = 0 = rp = buffer full
    b.write(6); // wp == 1, rp == 0
}

#[test]
/// Add four times, read four times
fn buffer_read_write_four_times(){
    let b = Buffer::new();
    b.write(1);
    b.write(2);
    b.write(3);
    b.write(4);
    let first  = b.read();
    let second = b.read();
    let third  = b.read();
    let four   = b.read();
    assert!(four == 4);
}

#[test]
/// Add five times, check if buffer is full
fn buffer_read_write_four_times(){
    let b = Buffer::new();
    b.write(1); // wp should be 1
    b.write(2); // wp should be 2
    b.write(3); // wp should be 3
    b.write(4); // wp should be 4
    b.write(5); // wp should be 0 and all buffer slots should be full
    assert!(b.is_full() == true);
}

#[test]
/// Test pointer stepping with four inserts and reads
fn buffer_pinters_after_four_reads_and_writes(){
    let mut b = Buffer::new();
    b.write(1);
    b.write(2);
    b.write(3);
    b.write(4);
    let first  = b.read();
    let second = b.read();
    let third  = b.read();
    let four   = b.read();
    assert!(b.wp == 4 && b.rp == 4);
}

#[test]
/// Read and write six times in 'random' order. 
/// Testing if circularity is correct.
fn buffer_read_write_random_one_circle(){
    let mut b = Buffer::new();
    b.write(1);
    let first = b.read();
    b.write(2);
    let second = b.read();
    b.write(3);
    b.write(4);
    let third = b.read();
    let four  = b.read();
    b.write(5);
    b.write(6);
    let five = b.read();
    let six = b.read();
    assert!(first == 1 && second == 2 && third == 3 && four == 4 && five == 5 && six == 6);
}

#[test]
/// Test pointers after six read writes in 'random' order. 
/// Testing if circularity works as intended
fn buffer_pointers_after_six_read_write(){
    let mut b = Buffer::new();
    b.write(1);
    let first = b.read();
    b.write(2);
    let second = b.read();
    b.write(3);
    b.write(4);
    let third = b.read();
    let four  = b.read();
    b.write = (5);
    b.write = (6);
    let five = b.read();
    let six = b.read();
    assert!(b.wp == 1 && b.rp ==1);
}
