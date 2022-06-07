extern crate lazy_static;
extern crate regex;
extern crate core;

use std::{env, fs};
use std::env::var;

use compilation_engine::CompilationEngine;

use crate::tokenizer::tokenizer;

mod xmlwriter;
mod tokenizer;
mod compilation_engine;
mod vm_writer;
mod symbol_table;
mod utility;

fn main() {

    //let args: Vec<String> = env::args().collect();

    let mut file_path;
    //let temp = args[1].to_string();
    let temp ="/home/shoham/nand2tetris/projects/11/Average".to_string();
    if !temp.contains(".jack"){file_path = search_jack_files(temp.as_str());} else{file_path= Vec::new(); file_path.push(temp.to_string())}
    for file in file_path {
        println!("file : {}",file);
        tokenizer(file.to_string());
        let mut compilation_engine: CompilationEngine = CompilationEngine::new(&file);
        compilation_engine.compile();
        //compilation_engine.class_name = "Main".to_string();
        /*compilation_engine.compile_subroutine_dec("function void convert(int value) {
    	var int mask, position;
    	var boolean loop;

    	let loop = true;
    	while (loop) {
    	    let position = position + 1;
    	    let mask = Main.nextMask(mask);

    	    if (~(position > 16)) {

    	        if (~((value & mask) = 0)) {
    	            do Memory.poke(8000 + position, 1);
       	        }
    	        else {
    	            do Memory.poke(8000 + position, 0);
      	        }
    	    }
    	    else {
    	        let loop = false;
    	    }
    	}
    	return;
    }".to_string());*/
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