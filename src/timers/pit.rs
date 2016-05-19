//! This module manages the PIT timer, specifically.  For more
//! information about the PIT, please see http://wiki.osdev.org/PIT

use x86::io::{inb, outb};

// IO Port Addresess

/// This is the system channel control register.
const PIT_PORT_CHANNEL0     : u16 = 0x40;

/// This is the channel 1 control register.
const PIT_PORT_CHANNEL1     : u16 = 0x41;

/// This is the channel 2 control register.
const PIT_PORT_CHANNEL2     : u16 = 0x42;

/// This is the IO Port number for the PIT command register.
const PIT_PORT_CMD          : u16 = 0x43;

// Bit masks for the PIT

/// Select PIT Channel 0. It generates an IRQ0.
const PIT_CHANNEL_SELECT_0    : u8 = 0b_00_000000;

/// Select PIT Channel 1
const PIT_CHANNEL_SELECT_1    : u8 = 0b_01_000000;

/// Select PIT Channel 2
const PIT_CHANNEL_SELECT_2    : u8 = 0b_10_000000;

/// Select PIT Channel 0
const PIT_SELECT_READBACK     : u8 = 0b_11_000000;

/// Latch the counter (pause timer) so that we can read back the current
/// count.
const PIT_ACCESS_LATCH_COUNT          : u8 = 0b_00_00_0000;

/// Write the lower byte of the counter register
const PIT_ACCESS_LATCH_LOBYTE         : u8 = 0b_00_01_0000;

/// Write the higher byte of the counter register
const PIT_ACCESS_LATCH_HIBYTE         : u8 = 0b_00_10_0000;

/// Write the lower, followed by the higher byte to the counter
/// register, in sequence.
const PIT_ACCESS_LATCH_LOBYTE_HIBYTE  : u8 = 0b_00_11_0000;

/// Operate in Interrupt Terminal mode (basically a counter)
const PIT_OPERATING_MODE_INTERRUPT_TERMINAL   : u8 = 0b_000_0;

/// Operate in one-shot mode
const PIT_OPERATING_MODE_ONESHOT              : u8 = 0b_001_0;

/// Operate in rate-generation mode
const PIT_OPERATING_MODE_RATEGEN              : u8 = 0b_010_0;

/// Operate as a Square Wave generator.
const PIT_OPERATING_MODE_SQWAVE               : u8 = 0b_100_0;

/// Operate as a strobe generator
const PIT_OPERATING_MODE_SWSTROBE             : u8 = 0b_100_0;

/// Default: Store the counter as a 16-bit binary number.
const PIT_16_BIT_BINARY     : u8 = 0b0;

/// This is apparently super deprecated, but it's a different
/// implementation of the counter register format. Don't use it.
const PIT_4_BIT_BCD         : u8 = 0b1;

/// The standard frequency of the PIT, in Hz.
const PIT_FREQUENCY_HZ : u32 = 119318;

/// The maximum possible rate (except =0, if that's supported...).  =
/// 54.924... ms.
pub const MAX_RATE : u16 = 2^16 -1;

/// The minimum possible rate. Far, far lower than a ms. Probably only
/// jitter.
pub const MIN_RATE : u16 = 1;

/// The rate to use to get exactly one ms delay.
pub const RATE_1_MS : u16 = 1193;

/// The rate to use to get half a ms delay.
pub const RATE_HALF_MS : u16 = 597;



/// Set the system timer. Formula is apparently: time in ms = divisor /
/// (3579545 / 3) * 1000. 0 is the special max value, meaning
/// divisor 65536 = 55 ms. Maybe. Depends on hardware.
pub fn set_timer(divisor : u16) {
    let options = PIT_CHANNEL_SELECT_0
        | PIT_16_BIT_BINARY
        | PIT_ACCESS_LATCH_LOBYTE_HIBYTE
        | PIT_OPERATING_MODE_INTERRUPT_TERMINAL;

    unsafe {
        // prime a request
        outb(PIT_PORT_CMD, options);

        // First write the lower byte
        outb(PIT_PORT_CHANNEL0, divisor as u8);

        // ...then write the upper byte
        outb(PIT_PORT_CHANNEL0, (divisor >> 8) as u8);
        }
}

/// Initialise the channel 0 PIT timer in Interrupt Terminal mode. It
/// will trigger IRQ 0 every NN ms, approximately.
///
/// Note that you must also have set up a reasonable ISR for the
/// relevant interrupt vector _before_ calling this function!
pub fn init() {

    set_timer(RATE_1_MS);
}

/// Function to call when the PIT times out.
/// Argument is ignored.
pub unsafe fn handle_timeout(_iv : usize) {
    println!("Timer reset! Now at {}", read_count());

    set_timer(RATE_1_MS);
}

/// Latch the PIT and read its current count.
pub fn read_count() -> u16 {

    unsafe {

        // We want to read the latch count
        outb(PIT_PORT_CMD, PIT_ACCESS_LATCH_COUNT);

        let low = inb(PIT_PORT_CHANNEL0) as u16;
        let high = inb(PIT_PORT_CHANNEL0) as u16;

        // Re-assemble a 2-byte number:
        (low | high << 8)
    }

}
