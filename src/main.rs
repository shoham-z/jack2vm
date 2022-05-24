mod xmlwriter;
mod tokenizer;
mod compilation_engine;

// include the latest version of the regex crate in your Cargo.toml
extern crate regex;
extern crate lazy_static;

use std::{env, fs};
use crate::tokenizer::tokenizer;
use compilation_engine::CompilationEngine;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{}", args[1]);

    let file_path = search_jack_files(args[1].as_str());
    for file in file_path {
        tokenizer(file.to_string());
        let mut compilation_engine: CompilationEngine = CompilationEngine::new(&file);
        compilation_engine.compile_class();
        break;
    }
}

fn maain() {
    let code = ["function void main() {
        var SquareGame game;
        let game = game;
        do game.run();
        do game.dispose();
        return;
        }"
,
        "function void more() {
        var boolean b;
        if (b) {
        }
        else {
        }
        return;
        }"];

    for block in code {
        println!("{}", block);
        let mut func = block.split("{");

        let func_dec = func.nth(0).unwrap();
        let mut func_body: String = String::new();
        let mut tmp = func.nth(0);
        while !tmp.is_none() {
            func_body += &*tmp.unwrap().to_string();
            tmp = func.nth(0);
            if tmp.is_some() {
                func_body += "{";
            }
        }
        func_body = func_body[0..func_body.len() - 1].parse().unwrap();

        println!("{:?}", func_dec);
        println!("\nfunc body: {:?}", func_body);
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