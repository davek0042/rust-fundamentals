use std::io::{self, Write};

fn main() {
    print!("Please enter a greeting: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    print!("Please enter your name: ");
    io::stdout().flush().unwrap();
    let mut fname = String::new();
    io::stdin().read_line(&mut fname).expect("Failed to read input");

    println!();
    // use of match expression to pattern match against variable "name"
    match name.trim() {
        "Good Bye" => println!("Sorry to see you go."),
        "Hello" => println!("Hi, nice to meet you, {}!", fname.trim()),
        _ => println!("I can't find a greeting, good bye."),
    }
}

