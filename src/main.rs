mod cmdargs;
mod gitcommands;

use std::io;
use std::process::{Command, exit};
use cmdargs::get_path;

#[derive(Clone, Copy)]
struct CommitStatus {
    number_of_commits : u16,
    current_position : u16
}

fn print_status(status : CommitStatus) {
    println!("Your position: {}/{}", status.current_position, status.number_of_commits);
}

fn get_next_status(status: CommitStatus) -> CommitStatus {
    let mut buffer = String::new();
    let stdin = io::stdin();

    print!("(+) next commit, (-) prev commit, (Strg+C) exit: ");
    stdin.read_line(&mut buffer)
        .expect("Reading from terminal failed :(");

    let buffer = buffer.trim();

    if buffer == String::from("+") {
        return next_commit(status);
    }

    if buffer == String::from("-") {
        return previous_commit(status);
    }

    return status;
}

fn previous_commit(status: CommitStatus) -> CommitStatus {
    return CommitStatus {
        number_of_commits: status.number_of_commits,
        current_position: status.current_position - 1
    }
}

fn next_commit(status: CommitStatus) -> CommitStatus {
    return CommitStatus {
        number_of_commits: status.number_of_commits,
        current_position: status.current_position + 1
    }
}

fn enter_directory(path: &String) -> bool {
    let output = Command::new("cd")
        .arg(path)
        .output()
        .expect("Failed to change directory");

    return output.status.success();
}

fn main() {
    let path = get_path();

    let number_of_commits = gitcommands::get_commit_count(&path).
        expect("The number of commits could not be detected. Is this path really pointing to a git repository?");

    let mut status = CommitStatus {
        number_of_commits,
        current_position: 1
    };

    if !enter_directory(&path) {
        println!("Could not change into repository directory.");
        exit(0);
    }

    loop {
        print_status(status);

        let commithash = gitcommands::get_nth_commithash(status.current_position)
            .expect("Could not get commit hash.");

        gitcommands::checkout_commit(&commithash);
        let info = gitcommands::get_commit_info(&commithash);

        print!("{:?}", info);

        status = get_next_status(status);
    }
}

