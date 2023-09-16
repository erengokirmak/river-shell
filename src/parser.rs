use std::{path::PathBuf, process::Command};
use dirs;

pub enum ParseError {
    BinaryNotFound(String),
    InvalidCommandStructure,
}

pub fn parse_command(path: &String, command: &str) -> Result<(), ParseError> {
    /// Checks if a given command is partially valid.
    /// Partially valid, in this case, is defined such that:
    /// - The piping of commands is valid (there command doesn't end with a pipe followed by no
    /// commands.
    /// - There are no empty commands between pipes.
    ///
    /// # Examples
    /// ```rust
    /// let command = "ls | cat";
    /// assert_eq!(true, validate_command(command)); // returns true
    ///
    /// let command = " | cat";
    /// assert_eq!(false, validate_command(command)); // returns false because no input as piped into cat
    ///
    /// let command = "ls | cat | | ";
    /// assert_eq!(false, validate_command(command)); // returns false as there is a pipe that is not valid
    /// ```
    fn validate_command(command: &str) -> bool {
        // If there is not a pipe, the command can only fail if the binary is not found or the
        // arguments are invalid. This requires the parse_command to check.
        if !command.contains("|") {
            return true;
        }

        let mut command = command.trim().split("|");

        while let Some(command) = command.next() {
            if command.trim() == "" {
                return false;
            }
        }

        true
    }

    let command = command.trim();
    
    if !validate_command(command) {
        return Err(ParseError::InvalidCommandStructure);
    }

    if command == "exit" || command == "q" {
        std::process::exit(0);
    } else if command == "" {
        println!("");
        return Ok(());
    }

    let mut command_groups = command.split("|").peekable();

    while let Some(command) = command_groups.next() {
        let mut command_split = command.split(" ");
        let binary = command_split.next().unwrap();
        match binary.trim() {
            "exit" => std::process::exit(0),
            "cd" => {
                match command_split.next() {
                    Some(path) => {
                        let _ = std::env::set_current_dir(path);
                        return Ok(());
                    },
                    None => {
                        if let Some(home_dir) = dirs::home_dir() {
                            let _ = std::env::set_current_dir(home_dir);
                        }
                        return Ok(());
                    },
                };
            }
            other => {
                if let Ok(path) = find_binary(path, other) {
                    Command::new(path)
                        .args(command_split)
                        .status()
                        .expect("The binary should be able to start");
                    return Ok(());
                } else {
                    return Err(ParseError::BinaryNotFound(other.to_string()));
                }
            }
        }
    }
    Ok(())
}

/// Searches for a particular binary in the current working directory and all directories in a
/// given path variable. Binaries in the current working directory are prioritized in the search.
fn find_binary(path: &str, command: &str) -> Result<PathBuf, std::io::Error> {
    fn is_valid_binary(path: &PathBuf) -> bool {
        path.exists() && !path.is_symlink() && path.is_file()
    }

    let mut potential_binary_location = std::env::current_dir().unwrap();
    potential_binary_location.push(command);
    if is_valid_binary(&potential_binary_location) {
        return Ok(potential_binary_location);
    }

    let paths = path.split(":");
    for path in paths {
        let mut candidate_file = PathBuf::from(path);
        candidate_file.push(command);
        if is_valid_binary(&candidate_file) {
            return Ok(candidate_file);
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Command not found",
    ))
}
