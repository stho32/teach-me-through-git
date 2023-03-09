use std::process::Command;

pub fn get_commit_count(path: &str) -> Option<u16> {
    let output = Command::new("git")
        .current_dir(path)
        .arg("rev-list")
        .arg("--count")
        .arg("HEAD")
        .output()
        .expect("Git command did not work.");

    let commit_count = String::from_utf8(output.stdout)
        .expect("Could not translate output from utf8");
    let commit_count = commit_count.trim();
    let commit_count = commit_count.parse::<u16>();

    match commit_count {
        Ok(commit_count) => { Some(commit_count) }
        Err(_) => { None }
    }
}
