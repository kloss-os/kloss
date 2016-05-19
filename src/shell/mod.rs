//! # Shell
//! _Very_ barebones shell system
//! Run by creating a shell, followed by implementation run
//!
//! ```
//! mod shell;
//! unsafe { // Unsafe to read global variables!
//!     if let Some(ref mut buffer) = io::kbd_buffer {
//!         let mut shell = shell::Shell::new();
//!         shell.run(buffer);
//!     }
//! }
//! ```
//!
//!
//! Current accepted commands:
//! + `print [word | '-msr'] [register]`
//!     - Prints either word or the msr register given
//!     - `skriv` in Swedish
//! + `set-lang ['sv' | 'en']`
//!     - Sets language to Swedish, English respectively
//!     - `utse-uttrycksmedel` in Swedish
//! + `set-prompt arg`
//!     - Sets current prompt to arg

use collections::String;
use collections::str::SplitWhitespace;
use collections::str::FromStr;
use collections::vec::Vec;

use vga_buffer;


use pipe::Buffer;
use msr;
use io;


enum Lang {
    en,
    sv,
}

const DEFAULT_LANG: Lang = Lang::en;

const en_prompt: &'static str = "ultra_user@BanjOS: ";
const sv_prompt: &'static str = "ultra_anv@BanjOS: ";


pub struct Shell {
    current_lang: Lang,
    prompt: String,
    history: Vec<String>,
    cur_line: usize,
}


impl Shell {
    /// Generates new shell according to default language value
    pub fn new() -> Shell {
        Shell { current_lang: DEFAULT_LANG,
                prompt: String::from(match DEFAULT_LANG {
                    Lang::en => en_prompt,
                    Lang::sv => sv_prompt,
                }),
                history: Vec::new(),
                cur_line: 0,
        }
    }

    /// Main loop for SHELL, expects a readable input buffer (which could be used for testing)
    pub fn run(&mut self, input: &mut Buffer) {

        print!("\n");
        loop {
            print!("{}", self.prompt);

            let mut line: String = String::from("");

            // Input loop
            let mut end_of_input: bool = false;
            while !end_of_input {

                // Nag the io variable until there's something to read
                while unsafe { !io::kbd_buffer_empty } {

                    let mut_input: &mut Buffer = input;
                    for current in mut_input {
                        match current {
                            0x08 => { // Backspace
                                vga_buffer::step_left();
                                print!(" ");
                                vga_buffer::step_left();
                                line.pop();
                            },
                            _ => {
                                print!("{}", current as char);

                                line.push(current as char);

                                // This could be expanded to check for quotes, escape char, etc
                                if current == b'\n' {
                                    end_of_input = true;
                                }
                            },
                        }
                    }

                    // Set the io variable to empty
                    unsafe { io::kbd_buffer_empty = true; }
                }
            }

            if line != "\n" { // Ignore empty lines
                self.history.push(line.clone());
                self.cur_line += 1;
            }

            let ref mut split_line = line.split_whitespace();

            unsafe {
                match self.current_lang {
                    Lang::en => self.parse_line_en(split_line),
                    Lang::sv => self.parse_line_sv(split_line),
                }
            }
        }
    }


    /// Sets current language via received id string
    fn set_lang(&mut self, id: &str) {
        self.current_lang = match id {
                "en" => Lang::en,
                "sv" => Lang::sv,
                _ => DEFAULT_LANG,
        };

        self.prompt = String::from(match self.current_lang {
            Lang::en => en_prompt,
            Lang::sv => sv_prompt,
        });

    }

    /// Parses arguments in swedish
    fn parse_line_sv(&mut self, rd_line: &mut SplitWhitespace) {
        match rd_line.next() {
            Some("skriv") => match rd_line.next() {
                // NOTE: UNSAFE! CAN BREAK WHAT YOU'RE DOING! CAUTION!
                Some("-msr") => if let Some(reg) = rd_line.next() {
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
                self.set_lang(new_lang);
            },

            None => {},
            _ => println!("Tolkning av kommandot misslyckades."),
        }
    }


    /// Parses arguments in english
    fn parse_line_en(&mut self, rd_line: &mut SplitWhitespace) {
        match rd_line.next() {
            Some("print") => match rd_line.next() {
                // NOTE: UNSAFE! CAN BREAK WHAT YOU'RE DOING! CAUTION!
                Some("-msr") => if let Some(reg) = rd_line.next() {
                    if let Ok(regnum) = u32::from_str(reg) {
                        println!("{} contains 0x{:x}",
                                 reg, unsafe {msr::read_msr(regnum)} );
                    } else {
                        println!("Please give numeric argument!");
                    }
                } else {
                    println!("No argument given");
                },
                Some("history") =>
                    for line in &self.history {
                        print!("{}", line);
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
                self.set_lang(new_lang);
            },

            Some("set-prompt") => if let Some(new_prompt) = rd_line.next() {
                self.prompt = String::from(new_prompt);
                self.prompt.push_str(": ");
            },

            Some("clear") => vga_buffer::clear_screen(),

            None => {},
            _ => println!("Unrecognized command"),
        }
    }
}
