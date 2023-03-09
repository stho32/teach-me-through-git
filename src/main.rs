mod cmdargs;
mod gitcommands;

use std::io;
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
        return CommitStatus {
            number_of_commits : status.number_of_commits,
            current_position : status.current_position + 1
        }
    }

    if buffer == String::from("-") {
        return CommitStatus {
            number_of_commits : status.number_of_commits,
            current_position : status.current_position - 1
        }
    }

    return CommitStatus {
        number_of_commits : status.number_of_commits,
        current_position : status.current_position
    };
}

fn main() {
    let path = get_path();

    let number_of_commits = gitcommands::get_commit_count(&path).
        expect("The number of commits could not be detected. Is this path really pointing to a git repository?");

    let mut status = CommitStatus {
        number_of_commits,
        current_position: 1
    };

    loop {
        print_status(status);
        status = get_next_status(status);
    }
}
