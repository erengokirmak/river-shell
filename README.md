# River-Shell

A limited implementation of a linux shell written in Rust.

### Currently supports:
- Standard linux commands such as `ls` and `cd`,
- Running binaries that are in the current directory / in the PATH,
- Piping,

# Usage
1. Clone this repository to your machine.
2. Go to the directory where the directory was cloned,
3. Use `cargo run` to run the shell or `cargo test` to check if the current shell passes the automated tests.  
4. Use `q` or `exit` when inside the shell to quit it.  

# Goals
My primary goals for the project are as such:

- [x]  Implement a search function to find binaries of commands.
- [x] Implement a command execution for commands without pipes and with arguments.
- [x] Implement a verifier to check whether a command is valid.
- [x] Implement piping.
- [ ]  (Bonus) implement outputting to a file.
