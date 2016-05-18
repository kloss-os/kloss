use core::intrinsics::{volatile_load, volatile_store};

use io;
use pipe;
use pipe::Buffer;


enum Keycode {
    NULL_KEY = 0,
    Q_PRESSED = 0x10,   Q_RELEASED = 0x90,
    W_PRESSED = 0x11,   W_RELEASED = 0x91,
    E_PRESSED = 0x12,   E_RELEASED = 0x92,
    R_PRESSED = 0x13,   R_RELEASED = 0x93,
    T_PRESSED = 0x14,   T_RELEASED = 0x94,
    Z_PRESSED = 0x15,   Z_RELEASED = 0x95,
    U_PRESSED = 0x16,   U_RELEASED = 0x96,
    I_PRESSED = 0x17,   I_RELEASED = 0x97,
    O_PRESSED = 0x18,   O_RELEASED = 0x98,
    P_PRESSED = 0x19,   P_RELEASED = 0x99,
    A_PRESSED = 0x1E,   A_RELEASED = 0x9E,
    S_PRESSED = 0x1F,   S_RELEASED = 0x9F,
    D_PRESSED = 0x20,   D_RELEASED = 0xA0,
    F_PRESSED = 0x21,   F_RELEASED = 0xA1,
    G_PRESSED = 0x22,   G_RELEASED = 0xA2,
    H_PRESSED = 0x23,   H_RELEASED = 0xA3,
    J_PRESSED = 0x24,   J_RELEASED = 0xA4,
    K_PRESSED = 0x25,   K_RELEASED = 0xA5,
    L_PRESSED = 0x26,   L_RELEASED = 0xA6,
    Y_PRESSED = 0x2C,   Y_RELEASED = 0xAC,
    X_PRESSED = 0x2D,   X_RELEASED = 0xAD,
    C_PRESSED = 0x2E,   C_RELEASED = 0xAE,
    V_PRESSED = 0x2F,   V_RELEASED = 0xAF,
    B_PRESSED = 0x30,   B_RELEASED = 0xB0,
    N_PRESSED = 0x31,   N_RELEASED = 0xB1,
    M_PRESSED = 0x32,   M_RELEASED = 0xB2,

    ZERO_PRESSED    = 0x29,
    ONE_PRESSED     = 0x2,
    TWO_PRESSED     = 0x3,
    THREE_PRESSED   = 0x4,
    FOUR_PRESSED    = 0x5,
    FIVE_PRESSED    = 0x6,
    SIX_PRESSED     = 0x7,
    SEVEN_PRESSED   = 0x8,
    EIGHT_PRESSED   = 0x9,
    NINE_PRESSED    = 0xA,

    SHIFT_PRESSED = 0x2A, SHIFT_RELEASED = 0xAA,
    POINT_PRESSED = 0x34, POINT_RELEASED = 0xB4,

    SLASH_RELEASED = 0xB5,

    BACKSPACE_PRESSED = 0xE,BACKSPACE_RELEASED = 0x8E,
    SPACE_PRESSED = 0x39,   SPACE_RELEASED = 0xB9,
    ENTER_PRESSED = 0x1C,   ENTER_RELEASED = 0x9C,
}

static mut shift: bool = false;

pub unsafe fn getkbd(arg: usize) {
    let flag: u8;
    let data: u8;

    asm!("in al, 0x64"
          : "={al}"(flag)
          :
          : "{al}"
          : "intel" );

    asm!("in al, 0x60"
          : "={al}"(data)
          :
          : "{al}"
          : "intel" );

    if data == Keycode::SHIFT_PRESSED as u8 {
        shift = true;
    } else if data == Keycode::SHIFT_RELEASED as u8 {
        shift = false;
    } else if let Some(ref mut buf) = io::kbd_buffer {
        let mut read_char = data_to_ascii(data);
        if !shift && 0x41 <= read_char && read_char <= 0x5A {
            read_char += 0x20
        }
        buf.write(read_char);
        //println!("Value is {}", buf.read() as char);
    }

    //println!("Flag: {:x}, data: {:x}, {:x}", flag, data, data_to_ascii(data) );

    let lapic_reg = (io::LAPIC_BASE | (io::LAPIC_EOI as usize)) as *mut usize;
    volatile_store(lapic_reg, 0);
}


fn data_to_ascii(data: u8) -> u8 {
    match data {
        data if data == Keycode::Q_PRESSED  as u8 => b'Q',
        data if data == Keycode::W_PRESSED  as u8 => b'W',
        data if data == Keycode::E_PRESSED  as u8 => b'E',
        data if data == Keycode::R_PRESSED  as u8 => b'R',
        data if data == Keycode::T_PRESSED  as u8 => b'T',
        data if data == Keycode::Z_PRESSED  as u8 => b'Z',
        data if data == Keycode::U_PRESSED  as u8 => b'U',
        data if data == Keycode::I_PRESSED  as u8 => b'I',
        data if data == Keycode::O_PRESSED  as u8 => b'O',
        data if data == Keycode::P_PRESSED  as u8 => b'P',
        data if data == Keycode::A_PRESSED  as u8 => b'A',
        data if data == Keycode::S_PRESSED  as u8 => b'S',
        data if data == Keycode::D_PRESSED  as u8 => b'D',
        data if data == Keycode::F_PRESSED  as u8 => b'F',
        data if data == Keycode::G_PRESSED  as u8 => b'G',
        data if data == Keycode::H_PRESSED  as u8 => b'H',
        data if data == Keycode::J_PRESSED  as u8 => b'J',
        data if data == Keycode::K_PRESSED  as u8 => b'K',
        data if data == Keycode::L_PRESSED  as u8 => b'L',
        data if data == Keycode::Y_PRESSED  as u8 => b'Y',
        data if data == Keycode::X_PRESSED  as u8 => b'X',
        data if data == Keycode::C_PRESSED  as u8 => b'C',
        data if data == Keycode::V_PRESSED  as u8 => b'V',
        data if data == Keycode::B_PRESSED  as u8 => b'B',
        data if data == Keycode::N_PRESSED  as u8 => b'N',
        data if data == Keycode::M_PRESSED  as u8 => b'M',

        data if data == Keycode::ENTER_PRESSED as u8 => 0x0A,

        data if data == Keycode::BACKSPACE_PRESSED as u8 => 0x08,
        _ => 0x00,
    }
}
