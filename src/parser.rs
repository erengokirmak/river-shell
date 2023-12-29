/// Describes the ways a parse attempt may fail
#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidCommandStructure,
}
/// Parses a command. Each element of the Split will be a command. If there are multiple elements,
/// the output of the earlier commands are fed into the later commands.
pub fn parse_command(command: &str) -> Result<&str, ParseError> {
    if !is_valid_command(command) {
        Err(ParseError::InvalidCommandStructure)
    } else {
        Ok(command.trim())
    }
}


#[derive(PartialEq, Eq)]
enum ParserState {
    Start,
    Command, // Parser must end on this if there is no '>'
    Pipe,
    Output, // Parser must end on this if there is a '>'
}

/// Checks if a given command is partially valid.
/// Partially valid is defined such that:
/// - The piping of commands is valid (the command doesn't start or end with a pipe).
/// - There are no empty commands between pipes.
///
/// # Examples
/// ```rust
/// let command = "ls | cat";
/// assert_eq!(true, is_valid_command(command)); // returns true
///
/// let command = " | cat";
/// assert_eq!(false, is_valid_command(command)); // returns false because no input as piped into cat
///
/// let command = "ls | cat | | grep word";
/// assert_eq!(false, is_valid_command(command)); // returns false as there is a pipe that is not valid
/// 
/// let command = "ls | cat | >"
/// ```
fn is_valid_command(command: &str) -> bool {
    // If there is not a pipe, the command can only fail if the binary is not found or the
    // arguments are invalid. This requires the parse_command to check.
    if !command.contains('|') {
        return true;
    }

    let command = command.trim().split('|');
    for cmd in command {
        if cmd.trim() == "" {
            return false;
        }
    }

    true
}

fn is_valid_command2(command: &str) -> bool {
    if !command.contains('|') && !command.contains('>') {
        return true;
    }

    let mut state = ParserState::Start;

    let commands = command.split('|');
    
    for cmd in commands {
        use ParserState as PS;
        match (cmd.trim(), state) {
            ("", PS::Start) => return true,
            (">", PS::Start) => return false,
            (_, PS::Start) => state = PS::Command,
            ("|", PS::Command) => state = PS::Pipe,
            (">", PS::Command) => state = PS::Output,
            (_, PS::Command) => state
        }
    }

    return !(state == ParserState::Start || state == ParserState::Pipe || state == ParserState::Output);
}

#[cfg(test)]
mod tests {

    #[test]
    fn command_validation_works() {
        use crate::parser::is_valid_command;

        let command = "sudo cd | cat | grep word";
        assert_eq!(true, is_valid_command(command));

        let command = "ls | cat";
        assert_eq!(true, is_valid_command(command)); // returns true

        let command = " | cat";
        assert_eq!(false, is_valid_command(command)); // returns false because no input as piped into cat

        let command = "ls | cat | | grep word";
        assert_eq!(false, is_valid_command(command)); // returns false as there is a pipe that is not valid

        let command = "ls |";
        assert_eq!(false, is_valid_command(command)); // returns false as there should not be a pipe there
    }
}
