//! # Shell
//! _Very_ barebones shell system


use pipe::Buffer;

pub fn main(input: &mut Buffer) {
    let mut prompt = "User@Banjos";
    
    loop {
        println!("\n{}", prompt);
        //while !input.is_empty() {
            println!("{}", input.read());
        //}

    }
}
