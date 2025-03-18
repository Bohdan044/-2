use std::process::Command;

fn run_command(command: &str, args: &[&str]) {
    let status = Command::new(command)
        .args(args)
        .status()
        .expect("Failed to execute command");

    if !status.success() {
        eprintln!("Command failed: {} {:?}", command, args);
    }
}

fn main() {
    println!("Cloning repository...");
    run_command("git", &["clone", "git@github.com:твій-юзернейм/rust-practice.git"]);

    println!("Adding changes...");
    run_command("git", &["add", "."]);

    println!("Committing changes...");
    run_command("git", &["commit", "-m", "Initial commit"]);

    println!("Pushing changes...");
    run_command("git", &["push", "origin", "main"]);

    println!("Pulling latest changes...");
    run_command("git", &["pull", "origin", "main"]);

    println!("All done!");
}
