use std::process;
use std::process::Command;

fn main() {
    let current_branch = read_curr_branch_name();
    println!("current branch: {}", current_branch);
}

fn read_curr_branch_name() -> String {
    let branch_command = Command::new("git rev-parse")
        .arg("--abbrev-ref HEAD")
        .output()
        .unwrap_or_else(|err| {
            println!("error while getting the current branch name: {}", err);
            process::exit(1);
        });
    String::from_utf8_lossy(&branch_command.stdout).to_owned().to_string()
}