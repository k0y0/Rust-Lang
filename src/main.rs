use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;

extern crate whatlang;

fn main() {
    let filepath: String = get_file_path();

    run(&filepath);
}

fn get_file_path() -> String {
    let mut args = std::env::args();

    let url: String = args.nth(1).expect("Url");        
    
    url
}


fn run(filepath: &str) {
    let filename = Path::new(filepath);

    if !filename.exists() {
        println!("File not exists");
        return ;
    }

    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();

            file.read_to_string(&mut content).unwrap();

            println!("{}", content);
            
            let info = whatlang::detect(&content).unwrap();

            println!("{}", info.lang());
        },
        Err(error) => {
            println!("Error opening file {}", error);
        }
    }



}