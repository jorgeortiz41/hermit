#[allow(unused_imports)]
use std::io::{self, Write};

enum ShellAction {
    Continue,
    Exit,
}

// takes in user input
fn split_command(input: &str) -> (&str, &str) {
    match input.find(' ') {
        Some(index) => (&input[0..index], input[index + 1..].trim_start()), // command and args
        None => (input, ""),
    }
}

fn handle_command(command_args: (&str, &str)) {
    let (command, args) = command_args;
    if command == "type" {
        // Type command
        if args == "exit" || args == "echo" || args == "type" {
            println!("{} is a shell builtin", args);
        } else {
            eprintln!("{}: not found", args);
        }
    } else if command == "echo" {
        // Echo the rest of line after "echo" command
        println!("{}", args);
    } else {
        // Command not found error
        eprintln!("{}: command not found", command);
    }
}

fn main() {
    loop {
        // Read
        println!("remember to-do items!");
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");
        let command = command.trim();

        // Eval
        let parsed_command = split_command(&command);

        if parsed_command.0 == "exit" {
            // Exit if command is "exit"
            break;
        }

        // Eval - Handle parsed command
        handle_command(parsed_command);

        // Print
        io::stdout().flush().unwrap();
    }
}
