//! # Shell
//! _Very_ barebones shell system

use collections::String;
use collections::str::SplitWhitespace;
use collections::str::FromStr;

use pipe::Buffer;
use msr;
use io;


enum Lang {
    en,
    sv,
}
const DEFAULT_LANG: Lang = Lang::en;
static mut lang: Lang = DEFAULT_LANG;


pub fn main(input: &mut Buffer) {
    let mut prompt = "kernel@BanjOS: ";
    let mut current_char: u8 = 0x00;
    let mut read_buffer: Buffer = *Buffer::new();


    print!("\n");
    loop {
        current_char = 0x00;

        print!("{}", prompt);


        // Input loop
        while current_char != b'\n' {
            while unsafe { !io::kbd_buffer_empty } {
                current_char = input.read();
                print!("{}", current_char as char);

                let mut_buf: &mut Buffer = &mut read_buffer;
                mut_buf.write(current_char);

                if input.is_empty() {
                    unsafe { io::kbd_buffer_empty = true; }
                }
            }
        }



        // Place read line in string
        let mut_buf: &mut Buffer = &mut read_buffer;
        let mut line: String = String::from("");
        for c in mut_buf {
            line.push(c as char);
        }
        let ref mut split_line = line.split_whitespace();

        unsafe {
            match lang {
                Lang::en => parse_line_en(split_line),
                Lang::sv => parse_line_sv(split_line),
                //_ => parse_line_en(split_line),
            }
        }
    }
}


fn set_lang(id: &str) {
    unsafe {
        lang =
            match id {
                "en" => Lang::en,
                "sv" => Lang::sv,
                _ => DEFAULT_LANG,
            }
    }
}

fn parse_line_sv(rd_line: &mut SplitWhitespace) {
    match rd_line.next() {
        Some("skriv") => match rd_line.next() {
            Some("msr") => if let Some(reg) = rd_line.next() {
                if let Ok(regnum) = u32::from_str(reg) {
                    println!("Registret {} har informationen 0x{:x}",
                             reg, unsafe {msr::read_msr(regnum)} );
                } else {
                    println!("Var god skriv numeriskt argument!");
                }
            } else {
                println!("Inget argument givet");
            },
            Some(thing) => println!("{}", thing),
            _ => println!("Inget argument givet"),
        },
        Some("kulor") => match rd_line.next() {
            Some(thing) =>
                println!("kulor till {}", thing),
            _ =>
                println!("kulor till dig!"),
        },
        Some("utse-uttrycksmedel") => if let Some(new_lang) = rd_line.next() {
            set_lang(new_lang);
        },
        None => {},
        _ => println!("Tolkning av kommandot misslyckades."),
    }
}


fn parse_line_en(rd_line: &mut SplitWhitespace) {
    match rd_line.next() {
        Some("print") => match rd_line.next() {
            Some("msr") => if let Some(reg) = rd_line.next() {
                if let Ok(regnum) = u32::from_str(reg) {
                    println!("{} contains 0x{:x}",
                             reg, unsafe {msr::read_msr(regnum)} );
                } else {
                    println!("Please give numeric argument!");
                }
            } else {
                println!("No argument given");
            },
            Some(thing) => println!("{}", thing),
            _ => println!("No argument given!"),
        },
        Some("balls") => match rd_line.next() {
            Some(thing) =>
                println!("Balls to {} too", thing),
            _ =>
                println!("Balls to you too"),
        },
        Some("set-lang") => if let Some(new_lang) = rd_line.next() {
            set_lang(new_lang);
        },
        None => {},
        _ => println!("Unrecognized command"),
    }
}

