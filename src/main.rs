#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        // Initialize Shell and read input
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let command = command.trim();

        // Quit if command is quit
        if command == String::from("quit") {
            break;
        }

        // Future: Handle "valid" command condition
        //
        //
        // Print output
        eprintln!("{}: command not found", command);
        io::stdout().flush().unwrap();
    }
}
