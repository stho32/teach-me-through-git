mod cmdargs;
mod gitcommands;

use cmdargs::get_path;

fn main() {
    let path = get_path();

    println!("path: {}", path);
    let number_of_commits = gitcommands::get_commit_count(&path).
        expect("The number of commits could not be detected. Is this path really pointing to a git repository?");
    println!("The repository contains {} commits.", number_of_commits)
}
