// IO Port Addresess
const PIT_IO_CHANNEL0     : u8 = 0x40;
const PIT_IO_CHANNEL1     : u8 = 0x41;
const PIT_IO_CHANNEL2     : u8 = 0x42;
const PIT_IO_MODE_COMMAND : u8 = 0x43;

// Bit masks for the PIT
const PIT_CHANNEL_SELECT_0    : u8 = 0b_00_000000;
const PIT_CHANNEL_SELECT_1    : u8 = 0b_01_000000;
const PIT_CHANNEL_SELECT_2    : u8 = 0b_10_000000;
const PIT_SELECT_READBACK     : u8 = 0b_11_000000;

const PIT_ACCESS_LATCH_COUNT          : u8 = 0b_00_00_0000;
const PIT_ACCESS_LATCH_LOBYTE         : u8 = 0b_00_01_0000;
const PIT_ACCESS_LATCH_HIBYTE         : u8 = 0b_00_10_0000;
const PIT_ACCESS_LATCH_LOBYTE_HIBYTE  : u8 = 0b_00_11_0000;

const PIT_OPERATING_MODE_INTERRUPT_TERMINAL   : u8 = 0b_000_0;
const PIT_OPERATING_MODE_ONESHOT              : u8 = 0b_001_0;
const PIT_OPERATING_MODE_RATEGEN              : u8 = 0b_010_0;
const PIT_OPERATING_MODE_SQWAVE               : u8 = 0b_100_0;
const PIT_OPERATING_MODE_SWSTROBE             : u8 = 0b_100_0;

const PIT_BCD_16_BIT_BINARY : u8 = 0b0;
const PIT_4_BIT_BCD         : u8 = 0b1;
