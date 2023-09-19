use std::{
    env,
    fmt::Display,
    path::Path,
    path::PathBuf,
    process::{Child, Command, Stdio},
};

#[derive(Debug)]
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

pub fn execute_command(command: std::str::Split<'_, &str>) -> Result<(), ExecutionError> {
    let mut peekable_commands = command.peekable();
    let mut previous_command: Option<Child> = None;
    while let Some(command) = peekable_commands.next() {
        let mut args = command.trim().split(' ');
        let binary = args.next().unwrap().trim();

        if binary == "exit" || binary == "q" {
            std::process::exit(0);
        } else if binary == "cd" {
            let _ = match args.next() {
                Some(dir) => std::env::set_current_dir(dir),
                None => std::env::set_current_dir("/"),
            };
            return Ok(());
        } else if binary == "" {
            return Ok(());
        }

        let binary_path: PathBuf = match find_binary(binary) {
            Ok(path) => path,
            Err(err) => return Err(err),
        };

        let stdout = if peekable_commands.peek().is_none() {
            Stdio::inherit()
        } else {
            Stdio::piped()
        };

        let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
            Stdio::from(output.stdout.unwrap())
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

pub fn find_binary(binary_name: &str) -> Result<PathBuf, ExecutionError> {
    fn is_valid_binary(path: &Path) -> bool {
        !path.is_symlink() && path.is_file() && !path.is_dir()
    }
    let paths = match env::var("PATH") {
        Ok(path) => path,
        Err(_) => "".to_string(),
    };

    let mut first_candidate = env::current_dir().unwrap();
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
    use std::path::PathBuf;

    use crate::execute::find_binary;

    #[test]
    fn finding_binaries_work() {
        assert_eq!(PathBuf::from("/bin/ls"), find_binary("ls").unwrap());
    }
}
