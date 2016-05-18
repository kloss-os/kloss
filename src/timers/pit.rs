//! This module manages the PIT timer, specifically.  For more
//! information about the PIT, please see http://wiki.osdev.org/PIT

extern crate x86;

// IO Port Addresess

/// This is the system channel control register.
const PIT_PORT_CHANNEL0     : u16 = 0x40;
const PIT_PORT_CHANNEL1     : u16 = 0x41;
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

/// A reasonable frequency for the PIT
const PIT_FREQUENCY_MILLIHZ : u32 = 1193182; // mHz

/// Initialise the channel 0 PIT timer in Interrupt Terminal mode. It
/// will trigger IRQ 0 every NN ms, approximately.
///
/// Note that you must also have set up a reasonable ISR for the
/// relevant interrupt vector _before_ calling this function!
pub fn init() {

    let divisor = (PIT_FREQUENCY_MILLIHZ / 1000) as u16; // Hz
    let options = PIT_CHANNEL_SELECT_0 | PIT_16_BIT_BINARY
        | PIT_ACCESS_LATCH_COUNT | PIT_ACCESS_LATCH_LOBYTE_HIBYTE;

    unsafe {
        // prime a request
        x86::io::outb(PIT_PORT_CMD, options);

        // First write the lower byte
        x86::io::outb(PIT_PORT_CHANNEL0, divisor as u8);

        // ...then write the upper byte
        x86::io::outb(PIT_PORT_CHANNEL0, (divisor >> 8) as u8);
        }
}
