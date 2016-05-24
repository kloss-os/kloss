use core::intrinsics::{volatile_load, volatile_store};

use io;

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

    ZERO_PRESSED = 0x29,
    ONE_PRESSED = 0x2,
    NINE_PRESSED = 0xA,

    POINT_PRESSED = 0x34,
    POINT_RELEASED = 0xB4,

    SLASH_RELEASED = 0xB5,

    BACKSPACE_PRESSED = 0xE,
    BACKSPACE_RELEASED = 0x8E,
    SPACE_PRESSED = 0x39,
    SPACE_RELEASED = 0xB9,
    ENTER_PRESSED = 0x1C,
    ENTER_RELEASED = 0x9C,
}


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

    println!("Flag: {:x}, data: {:x}, {}", flag, data, data_to_char(data) );

    io::send_LAPIC_EOI()
}


fn data_to_char(data: u8) -> &'static str {
    match data {
        data if data == Keycode::Q_PRESSED  as u8 => "Pressed Q!",
        data if data == Keycode::Q_RELEASED as u8 => "Released Q!",
        data if data == Keycode::W_PRESSED  as u8 => "Pressed W!",
        data if data == Keycode::W_RELEASED as u8 => "Released W!",
        data if data == Keycode::E_PRESSED  as u8 => "Pressed E!",
        data if data == Keycode::E_RELEASED as u8 => "Released E!",
        data if data == Keycode::R_PRESSED  as u8 => "Pressed R!",
        data if data == Keycode::R_RELEASED as u8 => "Released R!",
        data if data == Keycode::T_PRESSED  as u8 => "Pressed T!",
        data if data == Keycode::T_RELEASED as u8 => "Released T!",
        data if data == Keycode::Z_PRESSED  as u8 => "Pressed Z!",
        data if data == Keycode::Z_RELEASED as u8 => "Released Z!",
        data if data == Keycode::U_PRESSED  as u8 => "Pressed U!",
        data if data == Keycode::U_RELEASED as u8 => "Released U!",
        data if data == Keycode::I_PRESSED  as u8 => "Pressed I!",
        data if data == Keycode::I_RELEASED as u8 => "Released I!",
        data if data == Keycode::O_PRESSED  as u8 => "Pressed O!",
        data if data == Keycode::O_RELEASED as u8 => "Released O!",
        data if data == Keycode::P_PRESSED  as u8 => "Pressed P!",
        data if data == Keycode::P_RELEASED as u8 => "Released P!",
        data if data == Keycode::A_PRESSED  as u8 => "Pressed A!",
        data if data == Keycode::A_RELEASED as u8 => "Released A!",
        data if data == Keycode::S_PRESSED  as u8 => "Pressed S!",
        data if data == Keycode::S_RELEASED as u8 => "Released S!",
        data if data == Keycode::D_PRESSED  as u8 => "Pressed D!",
        data if data == Keycode::D_RELEASED as u8 => "Released D!",
        data if data == Keycode::F_PRESSED  as u8 => "Pressed F!",
        data if data == Keycode::F_RELEASED as u8 => "Released F!",
        data if data == Keycode::G_PRESSED  as u8 => "Pressed G!",
        data if data == Keycode::G_RELEASED as u8 => "Released G!",
        data if data == Keycode::H_PRESSED  as u8 => "Pressed H!",
        data if data == Keycode::H_RELEASED as u8 => "Released H!",
        data if data == Keycode::J_PRESSED  as u8 => "Pressed J!",
        data if data == Keycode::J_RELEASED as u8 => "Released J!",
        data if data == Keycode::K_PRESSED  as u8 => "Pressed K!",
        data if data == Keycode::K_RELEASED as u8 => "Released K!",
        data if data == Keycode::L_PRESSED  as u8 => "Pressed L!",
        data if data == Keycode::L_RELEASED as u8 => "Released L!",
        data if data == Keycode::Y_PRESSED  as u8 => "Pressed Y!",
        data if data == Keycode::Y_RELEASED as u8 => "Released Y!",
        data if data == Keycode::X_PRESSED  as u8 => "Pressed X!",
        data if data == Keycode::X_RELEASED as u8 => "Released X!",
        data if data == Keycode::C_PRESSED  as u8 => "Pressed C!",
        data if data == Keycode::C_RELEASED as u8 => "Released C!",
        data if data == Keycode::V_PRESSED  as u8 => "Pressed V!",
        data if data == Keycode::V_RELEASED as u8 => "Released V!",
        data if data == Keycode::B_PRESSED  as u8 => "Pressed B!",
        data if data == Keycode::B_RELEASED as u8 => "Released B!",
        data if data == Keycode::N_PRESSED  as u8 => "Pressed N!",
        data if data == Keycode::N_RELEASED as u8 => "Released N!",
        data if data == Keycode::M_PRESSED  as u8 => "Pressed M!",
        data if data == Keycode::M_RELEASED as u8 => "Released M!",

        data if data == Keycode::BACKSPACE_PRESSED as u8 => "Backspace pressed!",
        data if data == Keycode::BACKSPACE_RELEASED as u8 => "Backspace released!",
        _ => "Pressed unknown",
    }
}
