extern crate lazy_static;
// include the latest version of the regex crate in your Cargo.toml
extern crate regex;
extern crate core;

use std::{env, fs};
use std::env::args;

use compilation_engine::CompilationEngine;

use crate::tokenizer::tokenizer;

mod xmlwriter;
mod tokenizer;
mod compilation_engine;

fn main() {
    //let args: Vec<String> = env::args().collect();
    //println!("{}", args[1]);

    let mut file_path;
    //let mut temp = args[1];
    let temp ="/home/shoham/nand2tetris/projects/10/ExpressionLessSquare/Square.jack".to_string();
    if !temp.contains(".jack"){file_path = search_jack_files(temp.as_str());} else{file_path= Vec::new(); file_path.push(temp.to_string())}
    for file in file_path {
        tokenizer(file.to_string());
        let mut compilation_engine: CompilationEngine = CompilationEngine::new(&file);
        compilation_engine.compile();
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