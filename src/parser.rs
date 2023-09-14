use std::path::PathBuf;

pub enum ParseError {
    BinaryNotFound,
    InvalidArguments,
    InsufficientPermissions,
    EmptyCommand,
}

pub fn parse_command(path: &String, command: &str) -> Result<(), ParseError> {
    // This if block is here for testing purposes. It will go away soon.
    match command {
        "exit" | "q" => std::process::exit(0),
        _ => (),
    }
    if command == "" {
        return Err(ParseError::EmptyCommand);
    }

    // std::process::Command::new(find_binary(&path, command).unwrap().to_str().unwrap()).spawn().expect("The program should be able to run");

    match find_binary(&path, command) {
        Ok(path) => println!("{}", path.to_str().unwrap()),

        Err(err) => println!("{err}"),
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
