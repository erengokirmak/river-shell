use colored::Colorize;
use std::io::{stdout, Write};

use parser::parse_command;

mod execute;
mod parser;

fn main() {
    println!("Welcome to river-shell!\n");
    println!("Started shell with process ID: {}", std::process::id());
    loop {
        print!(
            "{}$ ",
            std::env::current_dir()
                .expect("Current directory should be reachable")
                .to_str()
                .expect("Current directory should be convertable to a string slice")
                .truecolor(0, 250, 217)
        );
        let _ = stdout().flush();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(err) => {
                println!("river-shell: {}", err);
                input = "".to_string();
            }
        }

        match parse_command(&input) {
            Ok(parsed_command) => {
                if let Err(err) = execute::execute_command(parsed_command) {
                    println!("river-shell: {}", err);
                }
            }
            Err(parser::ParseError::InvalidCommandStructure) => {
                eprintln!("river-shell: command has invalid structure")
            }
        }
    }
}
