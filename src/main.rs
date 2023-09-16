use std::io::Write;
use std::process;
mod parser;

fn main() {
    let pid = process::id(); // PID (Process ID) is a unique number given to each process on an OS.
    // let vars: Vec<(String, String)> = std::env::vars().into_iter().collect(); // Environment variables

    let path = match std::env::var("PATH") {
        Ok(path) => path,
        Err(_) => "".to_string(),
    };
    println!("PID: {}\n", pid);

    loop {
        let mut input = String::new();
        print!("{}$ ", std::env::current_dir().unwrap().to_str().unwrap());
        let _ = std::io::stdout().flush();
        let _ = std::io::stdin().read_line(&mut input);
        let input = input.trim(); // Get rid of whitespace

        match parser::parse_command(&path, input) {
            Ok(()) => (),
            Err(parser::ParseError::InvalidCommandStructure) => println!("river-shell: parsing error"),
            Err(parser::ParseError::BinaryNotFound(command)) => println!("the binary for {} was not found", command),
        }
    }
}
