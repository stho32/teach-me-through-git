use std::fs;

pub fn get_absolute_path(relative_path: String) -> String {
    return fs::canonicalize(relative_path)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
}

pub fn get_path() -> String {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        return get_absolute_path(args[1].clone());
    } else {
        return get_absolute_path(String::from("./")); // default to the current directory
    }
}
