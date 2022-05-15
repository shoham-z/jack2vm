mod xmlwriter;
mod tokenizer;

// include the latest version of the regex crate in your Cargo.toml
use std::{env, fs};
use tokenizer::JackTokenizer;




fn main() {
    let args: Vec<String> = env::args().collect();
    let xml_file_path = search_jack_files(args[1].as_str());
    for file in xml_file_path{
        let mut tokenizer = JackTokenizer::new(&file);
        tokenizer.tokenize();
    }
}


fn search_jack_files(file_path: &str) -> Vec<String> {
    let mut jack_files: Vec<String> = Vec::new();
    let paths = fs::read_dir(file_path).unwrap();
    for path in paths {
        if path.as_ref().unwrap().path().display().to_string().contains(".jack") {
            jack_files.push(path.unwrap().path().display().to_string())
        }
    }
    jack_files
}