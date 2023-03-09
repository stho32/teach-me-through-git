pub fn get_path() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        args[1].clone()
    } else {
        String::from("./") // default to the current directory
    }
}
