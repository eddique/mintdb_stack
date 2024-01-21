use std::io;
use std::io::Write;

pub fn start() {
    loop {
        print!("\x1b[38;5;50mmintdb:\x1b[0m ");
        io::stdout().flush().expect("Failed to flush stdo");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        println!("input: {input}");
        if input == "exit" {
            println!("shutting down");
            break;
        }
    }
}