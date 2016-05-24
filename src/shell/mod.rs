//! _Very_ barebones, sloppy, implementation of a shell
//!
//! Run by creating a shell, followed by implementation:
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
//!
//! + `echo WORD`
//!     - Prints _WORD_
//!     - `eko` in Swedish
//! + `msr REGISTER`
//!     - Prints _REGISTER_, provided it is an integer
//!     - `msr` in Swedish
//! + `history`
//!     - Prints the previously entered commands
//!     - `historik` in Swedish
//! + `set-lang LANG`
//!     - Sets language given by _LANG_ to Swedish (sv), English (en) respectively
//!     - `välj-språk` in Swedish
//! + `set-name ARG`
//!     - Sets current username to _ARG_
//!     - `välj-namn` in Swedish
//! + `set-host ARG`
//!     - Sets current host to _ARG_
//!     - `välj-värd` in Swedish
//! + `clear`
//!     - Clears screen
//!     - `rensa` in Swedish


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


/// Struct for the Shell, this could allow multiple TTY:s if one would like
pub struct Shell {
    /// Language currently set
    current_lang: Lang,
    /// The prompt (start of each new line)
    host: String,
    /// Vector containing previously entered lines
    history: Vec<String>,
    /// Index of current line
    cur_line: usize,
    /// User name
    user_name: String,
}


impl Shell {
    /// Generates new shell according to default language value
    pub fn new() -> Shell {
        Shell { current_lang: DEFAULT_LANG,
                host: String::from("BanjOS"),
                history: Vec::new(),
                cur_line: 0,
                user_name: String::from("super_user"),
        }
    }

    /// Main loop for SHELL, expects a readable input buffer (which could be used for testing)
    pub fn run(&mut self, input: &mut Buffer) {

        print!("\n");
        loop {
            print!("{}@{}: ", self.user_name, self.host);

            let mut line: String = String::from("");
            let mut tmp_line: String = String::from("");
            let mut tmp_cur_line: usize = self.cur_line;

            // Input loop
            let mut end_of_input: bool = false;
            while !end_of_input {

                // Nag the io variable until there's something to read
                while unsafe { !io::kbd_buffer_empty } {

                    let mut_input: &mut Buffer = input;
                    for current in mut_input {
                        match current {
                            0x08 => { // Backspace
                                if !line.is_empty() {
                                    vga_buffer::step_left();
                                    print!(" ");
                                    vga_buffer::step_left();
                                    line.pop();
                                }
                            },
                            0x82 => { // UP arrow
                                if tmp_cur_line > 0 {
                                    for _ in line.chars() {
                                        vga_buffer::step_left();
                                        print!(" ");
                                        vga_buffer::step_left();
                                    }

                                    tmp_cur_line -= 1;

                                    print!("{}", self.history[tmp_cur_line]);
                                    line = self.history[tmp_cur_line].clone();
                                }
                            },
                            0x83 => { // DOWN arrow
                                if tmp_cur_line + 1 < self.cur_line {
                                    for _ in line.chars() {
                                        vga_buffer::step_left();
                                        print!(" ");
                                        vga_buffer::step_left();
                                    }

                                    tmp_cur_line += 1;

                                    print!("{}", self.history[tmp_cur_line]);
                                    line = self.history[tmp_cur_line].clone();

                                } else if tmp_cur_line + 1 == self.cur_line {
                                    for _ in line.chars() {
                                        vga_buffer::step_left();
                                        print!(" ");
                                        vga_buffer::step_left();
                                    }
                                    tmp_cur_line += 1;

                                    print!("{}", tmp_line);
                                    line = tmp_line.clone();
                                }
                            },
                            _ => {
                                let cur_char = match current {
                                    0xC5 => 'Å', // Å
                                    0xE5 => 'å', // å
                                    0xC4 => 'Ä', // Ä
                                    0xE4 => 'ä', // ä
                                    0xD6 => 'Ö', // Ö
                                    0xF6 => 'ö', // ö
                                    ch => ch as char,
                                };
                                print!("{}", cur_char as char);

                                line.push(cur_char as char);
                                tmp_line.push(cur_char as char);

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

            // Remove newline char
            line.pop();

            if !line.is_empty() { // Ignore empty lines
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
    }

    /// Parses arguments in swedish
    fn parse_line_sv(&mut self, rd_line: &mut SplitWhitespace) {
        match rd_line.next() {
            Some("eko") => match rd_line.next() {
                Some(thing) => println!("{}", thing),
                _ => println!("Inget argument givet"),
            },

            // NOTE: UNSAFE! CAN BREAK WHAT YOU'RE DOING! CAUTION!
            Some("msr") => if let Some(reg) = rd_line.next() {
                if let Ok(regnum) = u32::from_str(reg) {
                    println!("Registret {} har informationen 0x{:x}",
                             reg, unsafe {msr::read_msr(regnum)} );
                } else {
                    println!("Var vänlig skriv numeriskt argument!");
                }
            } else {
                println!("Inget argument givet");
            },

            Some("historik") =>
                for line in &self.history {
                    println!("{}", line);
                },

            Some("välj-värd") => if let Some(new_host) = rd_line.next() {
                self.host = String::from(new_host);
            } else {
                println!("Inget argument givet");
            },

            Some("välj-språk") => if let Some(new_lang) = rd_line.next() {
                self.set_lang(new_lang);
            },

            Some("välj-namn") => if let Some(new_name) = rd_line.next() {
                self.user_name = String::from(new_name);
            },

            Some("rensa") => vga_buffer::clear_screen(),

            Some("avsluta") =>
                println!("Jag kan inte låta dig göra det, {}", self.user_name),

            None => {},
            _ => println!("Tolkning av kommandot misslyckades."),
        }
    }


    /// Parses arguments in english
    fn parse_line_en(&mut self, rd_line: &mut SplitWhitespace) {
        match rd_line.next() {
            Some("echo") => match rd_line.next() {
                Some(thing) => println!("{}", thing),
                _ => println!("No argument given!"),
            },

            Some("history") =>
                for line in &self.history {
                    println!("{}", line);
                },

            // NOTE: UNSAFE! CAN BREAK WHAT YOU'RE DOING! CAUTION!
            Some("msr") => {
                if let Some(reg) = rd_line.next() {
                    if let Ok(regnum) = u32::from_str(reg) {
                        println!("{} contains 0x{:x}",
                                 reg, unsafe {msr::read_msr(regnum)} );
                    } else {
                        println!("Please give numeric argument!");
                    }
                } else {
                    println!("No argument given");
                }
            },

            Some("set-lang") => if let Some(new_lang) = rd_line.next() {
                self.set_lang(new_lang);
            },

            Some("set-host") => if let Some(new_host) = rd_line.next() {
                self.host = String::from(new_host);
            } else {
                println!("No argument given");
            },

            Some("clear") => vga_buffer::clear_screen(),

            Some("set-name") => if let Some(new_name) = rd_line.next() {
                self.user_name = String::from(new_name);
            },

            Some("shutdown") =>
                println!("I can't let you do that, {}", self.user_name),

            None => {},
            _ => println!("Unrecognized command"),
        }
    }
}
