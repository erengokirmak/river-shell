use std::{
    env,
    fmt::Display,
    path::Path,
    path::PathBuf,
    process::{Child, Command, Stdio},
};

#[derive(Debug, PartialEq)]
pub enum ExecutionError {
    BinaryNotFound(String),
    ProcessErrorFound,
}

impl Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProcessErrorFound => write!(f, "command could not be executed correctly"),
            Self::BinaryNotFound(binary) => write!(f, "command not found: {binary}"),
        }
    }
}

/// Executes a command given as a string.
pub fn execute_command(command: &str)-> Result<(), ExecutionError> {
    let mut commands = command.split("|").peekable();
    let mut previous_command: Option<Child> = None;
    while let Some(command) = commands.next() {
        let mut args = command.trim().split(' ');
        let binary = args.next()
                        .expect("the args iterator should have at least one item")
                        .trim();

        // Built-in commands
        match binary {
            "exit" | "q" => std::process::exit(0),
            "cd" => {
                let _ = match args.next() {
                    Some(dir) => std::env::set_current_dir(dir),
                    None => std::env::set_current_dir("/"),
                };
            }
            "" => return Ok(()),
            _ => (),
        }

        let binary_path: PathBuf = match find_binary(binary) {
            Ok(path) => path,
            Err(err) => return Err(err),
        };

        // 
        let stdout = match commands.peek() {
            Some(_) => Stdio::piped(),
            None => Stdio::inherit(),
        };

        // Specifying the input based on 
        let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
            Stdio::from(output.stdout.expect("standard output should be reachable"))
        });

        let output = Command::new(binary_path)
            .args(args)
            .stdout(stdout)
            .stdin(stdin)
            .spawn();

        match output {
            Ok(output) => previous_command = Some(output),
            Err(_) => return Err(ExecutionError::ProcessErrorFound),
        }
    }

    if let Some(mut final_command) = previous_command {
        let _ = final_command.wait();
    }
    Ok(())
}

/// Searches the current directory and the PATH variable
/// and finds a binary matching with the given name
pub fn find_binary(binary_name: &str) -> Result<PathBuf, ExecutionError> {
    fn is_valid_binary(path: &Path) -> bool {
        !path.is_symlink() && path.is_file() && !path.is_dir()
    }
    let paths = match env::var("PATH") {
        Ok(path) => path,
        Err(_) => "".to_string(),
    };

    let mut first_candidate = env::current_dir().expect("current directory should be reachable");
    first_candidate.push(binary_name);

    if is_valid_binary(&first_candidate) {
        return Ok(first_candidate);
    }

    for potential_path in paths.split(':') {
        let binary_candidate = PathBuf::from(potential_path.to_owned() + "/" + binary_name);
        if is_valid_binary(&binary_candidate) {
            return Ok(binary_candidate);
        }
    }
    Err(ExecutionError::BinaryNotFound(binary_name.to_string()))
}

#[cfg(test)]
mod tests {
    use crate::execute::{execute_command, find_binary};
    use std::path::PathBuf;

    /// If the test is run on a linux system, this is expected behavior
    #[test]
    fn finding_binaries_work() {
        assert_eq!(PathBuf::from("/bin/ls"), find_binary("ls").expect("hardcoded binary should be findable"));
    }

    #[test]
    fn empty_command_returns_ok() {
        assert_eq!(Ok(()), execute_command(""));
    }

    #[test]
    fn changing_directories_works() {
        let _ = execute_command("cd");
        assert_eq!(std::env::current_dir().expect("current directory should be reachable"), PathBuf::from("/"));
    }
}
