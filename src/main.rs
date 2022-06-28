use std::fs;
use std::env;

fn main() {
    let filepath: String = get_file_path();

    run(&filepath);
}

fn get_file_path() -> String {
    env::args().nth(1).expect("Url")      
}


fn run(filepath: &str) {
    let content = fs::read_to_string(filepath).unwrap();

    println!("file content \"{}...", &content[..50]);

    let info = whatlang::detect(&content).unwrap();

    println!("detected language: {}", info.lang());
}