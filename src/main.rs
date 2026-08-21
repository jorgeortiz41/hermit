#[allow(unused_imports)]
use std::io::{self, Write};

fn detect_command(command: &str) -> &str {
    match command.find(' ') {
        Some(index) => &command[0..index],
        None => command, // no space? the whole thing is the command
    }
}

fn main() {
    loop {
        // Read
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let command = command.trim();

        // Eval
        let parsed_command = detect_command(&command);

        // Eval - Handle parsed command
        if parsed_command == String::from("exit") {
            // Exit if command is "exit"
            break;
        } else if parsed_command == String::from("type") {
            // Type command
            let type_command = &command[5..];
            if type_command == String::from("exit")
                || type_command == String::from("echo")
                || type_command == String::from("type")
            {
                println!("{} is a shell builtin", type_command);
            } else {
                eprintln!("{}: not found", type_command);
            }
        } else if parsed_command == String::from("echo") {
            // Echo the rest of line after "echo" command
            println!("{}", &command[5..]);
        } else {
            // Command not found error
            eprintln!("{}: command not found", command);
        }

        // Print
        io::stdout().flush().unwrap();
    }
}
