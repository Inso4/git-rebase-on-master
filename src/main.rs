use std::io;
use std::process;
use std::process::Command;
use std::process::Output;

fn main() {
    let dev_branch = unwrap_or_exit(get_curr_branch_name());
    println!("current branch: {}", dev_branch);
    if dev_branch == "master" {
        println!("no need to rebase. on master")
    }

    println!("checking out master");
    let output = unwrap_or_exit(checkout_branch("master"));
    println!("{}", output);

    println!("pulling master");
    let output = unwrap_or_exit(pull());
    println!("{}", output);

    println!("checking out {}", dev_branch);
    let output = unwrap_or_exit(checkout_branch(&dev_branch));
    println!("{}", output);

    println!("rebasing {} on master", dev_branch);
    let output = unwrap_or_exit(rebase());
    println!("{}", output);
}

fn unwrap_or_exit(result: io::Result<Output>) -> String {
    let _output = result.unwrap();
    if _output.status.success() {
        String::from_utf8_lossy(&_output.stdout).trim().to_string()
    }
    else
    {
        let string_output = String::from_utf8_lossy(&_output.stderr).trim().to_string();
        eprintln!("{}", string_output);
        process::exit(1);
    }
}

fn get_curr_branch_name() -> io::Result<Output> {
    Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("--symbolic")
        .arg("HEAD")
        .output()
}

fn checkout_branch(branch_name: &str) -> io::Result<Output> {
    Command::new("git")
        .arg("checkout")
        .arg(branch_name)
        .output()
}

fn pull() -> io::Result<Output> {
    Command::new("git")
        .arg("pull")
        .output()
}

fn rebase() -> io::Result<Output> {
    Command::new("git")
        .arg("rebase")
        .arg("master")
        .output()
}