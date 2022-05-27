extern crate lazy_static;
// include the latest version of the regex crate in your Cargo.toml
extern crate regex;
extern crate core;

use std::{env, fs};

use compilation_engine::{CompilationEngine, OP,KEYWORD_CONSTANT,UNARY_OP};

use crate::tokenizer::tokenizer;

mod xmlwriter;
mod tokenizer;
mod compilation_engine;

fn maian() {
    let args: Vec<String> = env::args().collect();
    //println!("{}", args[1]);

    let file_path = search_jack_files(args[1].as_str());
    for file in file_path {
        tokenizer(file.to_string());
        let mut compilation_engine: CompilationEngine = CompilationEngine::new(&file);
        compilation_engine.compile_class();
    }
}

fn ex(expression:String){
    println!("<expression>");
    let mut index =usize::MAX;
    let mut tmp;
    let mut arr:Vec<usize> = Vec::new();
    for op in OP{
        tmp = expression.find(op);
        match tmp {
            None => {}
            Some(value) => {index = value;}

        }
        if index!=usize::MAX{

            arr.push(index);
        }
    }
    index=usize::MAX;

    for val in arr{
        if val<index{
            index = val;
        }
    }

    //println!("{:?}",index);
    if index==usize::MAX {

            deal_term(expression.trim().to_string());


    }
    else {
        if expression.get(0..index).unwrap().find("(").is_some() && expression.get(0..index).unwrap().find("(").unwrap() == 0 {
            deal_term(expression.get(expression.find("(").unwrap()..expression.rfind(")").unwrap() + 1).unwrap().trim().to_string());
        } else {
            deal_term(expression.get(0..index).unwrap().trim().to_string());

            let symbol = expression.get(index..index + 1).unwrap();

            match symbol{
                "<" => println!("<symbol>{}</symbol>", "&lt;"),
                ">" => println!("<symbol>{}</symbol>", "&gt;"),
                "&" => println!("<symbol>{}</symbol>", "&amp;"),
                &_ => println!("<symbol>{}</symbol>", symbol)
            }

            ex(expression.get(index + 1..expression.len()).unwrap().to_string());
        }
    }
    println!("</expression>");
}

fn deal_term(term:String){
    println!("<term>");
    //println!("current term: {}", term);
    for keyword in KEYWORD_CONSTANT{
        if term == keyword.to_string(){
            println!("<keyword>{}</keyword>", term);
            break;
        }
    }
    if term.find("(")==Some(0) && term.rfind(")")==Some(term.len()-1){
        println!("<symbol>{}</symbol>", "(");
        if term.find("-")==Some(1){
            println!("<symbol>{}</symbol>", "-");
            deal_term(term.get(2..term.len()-1).unwrap().to_string())
        } else if term.find("~")==Some(1) {
            println!("<symbol>{}</symbol>","~");
        } else{
            ex(term.get(term.find("(").unwrap()+1..term.rfind(")").unwrap()).unwrap().to_string());
        }
        println!("<symbol>{}</symbol>", ")");
    } else if term.to_ascii_lowercase().find("\"")==Some(0) && term.to_ascii_lowercase().find("\"")==Some(term.len()-1) {
        print!("<stringConstant>");
        ex(term.get(term.find("\"").unwrap() + 1..term.find("\"").unwrap()).unwrap().to_string());
        print!("</stringConstant>\n");
    } else if term.chars().all(char::is_numeric) { // check for integer constant
        print!("<integerConstant>");
        print!("{}", term.to_string());
        print!("</integerConstant>\n");
    } else if UNARY_OP.contains(&term.chars().next().unwrap().to_string().as_str()){
        println!("FUCKKKKKKK");
    } else if term.find(".").is_some(){
        println!("<identifier>{}</identifier>", term.get(0..term.find(".").unwrap()).unwrap());

        println!("<symbol>{}</symbol>", ".");

        println!("<identifier>{}</identifier>", term.get(term.find(".").unwrap()+1..term.find("(").unwrap()+1).unwrap());

        println!("<symbol>{}</symbol>", "(");

        println!("<expressionList>{}</expressionList>",term.get(term.find("(").unwrap()+1..term.find(")").unwrap()).unwrap());

        println!("<symbol>{}</symbol>", ")");
    } else if term.find("[").is_some() {
        println!("<identifier>{}</identifier>", term.get(0..term.find("[").unwrap()).unwrap());

        println!("<symbol>{}</symbol>", "[");

        ex(term.get(term.find("[").unwrap()+1..term.find("]").unwrap()).unwrap().to_string());

        println!("<symbol>{}</symbol>", "]");

    } else{
        println!("<identifier>{}</identifier>", term);
    }
    println!("</term>");

}

fn main() {
    //ex("5*(a[i] - square.run())".to_string());

    let code = "while (x) {
         do erase();
         let size = size;
         do draw();
      }
      return;
   }";

    let mut code = "var char key;
      var boolean exit;

      let exit = key;
      while (exit) {
         while (key) {
            let key = key;
            do moveSquare();
         }

         if (key) { let exit = exit; }
         if (key) { do square.decSize(); }
         if (key) { do square.incSize(); }
         if (key) { let direction = exit; }
         if (key) { let direction = key; }
         if (key) { let direction = square; }
         if (key) { let direction = direction; }

         while (key) {
            let key = key;
            do moveSquare();
         }
      }
      return;
      }";


    //println!("{}", code);
    let mut start_statement= "";
    for line in code.lines(){ if line.contains("while"){start_statement = line; println!("{}",line);break;}}

    let mut open_count = code.matches("{").count();
    let mut close_count = code.matches("}").count();

    let mut lines = code.get(code.find(start_statement).unwrap()..code.len()).unwrap().lines();
    let mut current_line = lines.nth(0).unwrap();
    let mut block = code.get(code.find(start_statement).unwrap()..code.find(current_line).unwrap() + current_line.len()).unwrap();
    loop {

        current_line = lines.nth(0).unwrap();
        if !current_line.is_empty() {
            open_count = block.matches("{").count();
            close_count = block.matches("}").count();
            println!("\nopen: {},close: {}  {}..{} : {}\n",open_count,close_count,code.find(start_statement).unwrap(),code.find(current_line).unwrap() + current_line.len(), current_line);
            if open_count==close_count{
                break;
            }else if open_count<close_count{
                panic!("bad code")
            }
            //code = code.get(0..code.find(current_line).unwrap()) + code.get(code.find(current_line).unwrap() + current_line.le)
            block = code.get(code.find(start_statement).unwrap()..code.find(current_line).unwrap() + current_line.len()).unwrap();
        }
    }

    println!("{}",block);

    //for line in code.get(code.find(start_statement).unwrap()..code.len()).unwrap().lines(){ println!("{}",line);}

    //println!("{}",start_statement);

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