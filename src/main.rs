mod cmdargs;
mod gitcommands;
mod screen;

use std::process::exit;

use cmdargs::get_path;
use screen::{clear_screen, read_line};

#[derive(Clone, Copy)]
struct CommitStatus {
    number_of_commits: u16,
    current_position: u16,
}

fn print_status(status: CommitStatus) {
    println!(
        "Your position: {}/{}",
        status.current_position, status.number_of_commits
    );
}

fn get_next_status(status: CommitStatus) -> CommitStatus {
    let buffer = read_line("Choose your next step: (+, -, e)", "");

    if buffer == String::from("+") {
        return next_commit(status);
    }

    if buffer == String::from("-") {
        return previous_commit(status);
    }

    if buffer == String::from("e") {
        exit(0);
    }

    return status;
}

fn previous_commit(status: CommitStatus) -> CommitStatus {
    return CommitStatus {
        number_of_commits: status.number_of_commits,
        current_position: status.current_position - 1,
    };
}

fn next_commit(status: CommitStatus) -> CommitStatus {
    return CommitStatus {
        number_of_commits: status.number_of_commits,
        current_position: status.current_position + 1,
    };
}

fn main() {
    let path = get_path();

    let number_of_commits = gitcommands::get_commit_count(&path).
        expect("The number of commits could not be detected. Is this path really pointing to a git repository?");

    let mut status = CommitStatus {
        number_of_commits,
        current_position: 1,
    };

    loop {
        clear_screen();

        println!("Path: {}", path);
        println!("number of commits : {}", number_of_commits);

        print_status(status);

        let commithash = gitcommands::get_nth_commithash(
            status.number_of_commits - status.current_position,
            &path,
        )
        .expect("Could not get commit hash.");

        println!(
            "commithash of the : {}th commit = {}",
            status.current_position, commithash
        );

        if (commithash != String::from("")) {
            println!("checking it out...");
            gitcommands::checkout_commit(&commithash, &path);

            let info = gitcommands::get_commit_info(&commithash, &path)
                .expect("Could not get commit info.");

            println!("\n\n{}", info);
        }

        status = get_next_status(status);
    }
}
