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
//! + `echo _WORD_`
//!     - Prints _WORD_
//!     - `eko` in Swedish
//! + `msr _REGISTER_`
//!     - Prints _REGISTER_, provided it is an integer
//!     - `msr` in Swedish
//! + `history`
//!     - Prints the previously entered commands
//!     - `historik` in Swedish
//! + `set-lang _LANG_`
//!     - Sets language to Swedish (sv), English (en) respectively
//!     - `utse-uttrycksmedel` in Swedish
//! + `set-prompt _ARG_`
//!     - Sets current prompt to _ARG_
//!     - `utse-kommandoprompt` in Swedish
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

const en_prompt: &'static str = "ultra_user@BanjOS: ";
const sv_prompt: &'static str = "ultra_anv@BanjOS: ";


/// Struct for the Shell, this could allow multiple TTY:s if one would like
pub struct Shell {
    /// Language currently set
    current_lang: Lang,
    /// The prompt (start of each new line)
    prompt: String,
    /// Vector containing previously entered lines
    history: Vec<String>,
    /// Index of current line
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
                                    for _ in 0..line.len() {
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
                                    for _ in 0..line.len() {
                                        vga_buffer::step_left();
                                        print!(" ");
                                        vga_buffer::step_left();
                                    }

                                    tmp_cur_line += 1;

                                    print!("{}", self.history[tmp_cur_line]);
                                    line = self.history[tmp_cur_line].clone();

                                } else if tmp_cur_line + 1 == self.cur_line {
                                    for _ in 0..line.len() {
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
                                print!("{}", current as char);

                                line.push(current as char);
                                tmp_line.push(current as char);

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

        self.prompt = String::from(match self.current_lang {
            Lang::en => en_prompt,
            Lang::sv => sv_prompt,
        });

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
                    println!("Var god skriv numeriskt argument!");
                }
            } else {
                println!("Inget argument givet");
            },

            Some("historik") =>
                for line in &self.history {
                    print!("{}", line);
                },

            Some("utse-kommandoprompt") => if let Some(new_prompt) = rd_line.next() {
                self.prompt = String::from(new_prompt);
                self.prompt.push_str(": ");
            } else {
                println!("Inget argument givet");
            },

            Some("utse-uttrycksmedel") => if let Some(new_lang) = rd_line.next() {
                self.set_lang(new_lang);
            },

            Some("rensa") => vga_buffer::clear_screen(),

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

            Some("set-prompt") => if let Some(new_prompt) = rd_line.next() {
                self.prompt = String::from(new_prompt);
                self.prompt.push_str(": ");
            } else {
                println!("No argument given");
            },

            Some("clear") => vga_buffer::clear_screen(),

            None => {},
            _ => println!("Unrecognized command"),
        }
    }
}
