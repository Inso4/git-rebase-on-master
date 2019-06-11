use std::env;
use std::process::Command;

fn main() {
    //repo
    let curr_dir = env::current_dir().unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
    println!("current dir: {}", curr_dir);

    let dev_branch = get_curr_branch_name();
    println!("current branch: {}", dev_branch);
    //todo: error on master

    println!("checking out master");
    let output = checkout_branch("master");
    println!("{}", output);

    println!("pulling master");
    let output = pull();
    println!("{}", output);

    println!("checking out {}", dev_branch);
    let output = checkout_branch(&dev_branch);
    println!("{}", output);

    println!("rebasing {} on master", dev_branch);
    let output = rebase();
    println!("{}", output);
}

fn get_curr_branch_name() -> String {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("--symbolic")
        .arg("HEAD")
        .output().unwrap();
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

//todo: add error printing and panic
fn checkout_branch(branch_name: &str) -> String {
    let output = Command::new("git")
        .arg("checkout")
        .arg(branch_name)
        .output().unwrap();
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn pull() -> String {
    let output = Command::new("git")
        .arg("pull")
        .output().unwrap();
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn rebase() -> String {
    let output = Command::new("git")
        .arg("rebase")
        .arg("master")
        .output().unwrap();
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}