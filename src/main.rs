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

fn handle_command(command: &str, args: &str) -> ShellAction {
    match command {
        "exit" => ShellAction::Exit,
        "type" => {
            if args == "exit" || args == "echo" || args == "type" {
                println!("{} is a shell builtin", args);
            } else {
                eprintln!("{}: not found", args);
            }
            ShellAction::Continue
        }
        "echo" => {
            println!("{}", args);
            ShellAction::Continue
        }
        _ => {
            eprintln!("{}: command not found", command);
            ShellAction::Continue
        }
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
        let (command, args) = split_command(command);
        if let ShellAction::Exit = handle_command(command, args) {
            break;
        }

        // Print
        io::stdout().flush().unwrap();
    }
}
