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

pub fn get_nth_commithash(n : u16) -> Option<String> {
    // Get the commit hash of the nth commit
    let output = Command::new("git")
        .args(&["rev-list", "--max-parents=0", "--skip", &(n.to_string()), "--max-count=1"])
        .output()
        .expect("Failed to get commit hash");

    if !output.status.success() {
        eprintln!("{:?}", output);
        return None;
    }

    let commit_hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
    return Some(commit_hash);
}

pub fn checkout_commit(commit_hash: &String) {
    let output = Command::new("git")
        .args(&["checkout", &commit_hash])
        .output()
        .expect("Failed to checkout commit");

    if !output.status.success() {
        eprintln!("Failed to checkout commit: {:?}", output);
        return;
    }
}

pub fn get_commit_info(commit_hash: &str) -> Result<String, String> {
    let output = Command::new("git")
        .args(&["show", commit_hash])
        .output()
        .map_err(|e| format!("Failed to get commit information: {}", e))?;

    if !output.status.success() {
        return Err(format!("Failed to get commit information: {:?}", output));
    }

    let commit_info = String::from_utf8_lossy(&output.stdout).to_string();

    Ok(commit_info)
}
