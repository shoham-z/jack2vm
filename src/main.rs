// include the latest version of the regex crate in your Cargo.toml
extern crate regex;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::{fs, io};
use std::borrow::Borrow;
use std::path::Path;
use std::io::BufRead;
use std::net::SocketAddr;

fn main() {
    let file_path = r"C:\nandtot\nand2tetris\projects\10\Square\square.jack";
    //this way we kick out all the comments:
    let regex_no_comments = Regex::new(r#"\*\*.*\*/\n|//.*\n|/\*.*\*/\n\*/"#).unwrap();
    //reading the data and *it has to be owned other ways regex will not be able to use it*:
    let file_raw_data =fs::read_to_string(file_path).unwrap().as_str().to_owned();
    //none readable data that's way next line i transferred it to chars:
    let mut after_no_comments = regex_no_comments.replace_all(&file_raw_data, "");
    //a vectors for all the chars:
    let mut chars_vec =vec![];
    for text in after_no_comments.chars() {
        print!("{}", text);
        if text != '\n' || text != ' ' {
            chars_vec.push(text);
        }
    }
    let saved_key_words = vec!["class", "constructor", "function", "method", "field", "static", "var", "int", "char", "boolean", "void", "true", "false", "null", "this", "let", "do", "if", "else", "while", "return"];
    let saved_symbols = vec![";","-","=","+","/",".","{","}","(",")","[","]","<",">","&","|"];
    let mut word =String::new();
    for mut index in 0..(chars_vec.len()-1)  {
        //checks if the index is oversize the array:
        if index <chars_vec.len(){
            word.push(chars_vec[index]); //combining all the chars until we get a valid word\symbol\identifier
            if !(word.contains("\t") || word.contains("\n") || word.contains(" ")) {
                //checks if the word is a Key word
                if saved_key_words.contains(&&*word) {
                    key_words(word.borrow());
                    word.clear();
                }
                //checks if the word is a string:
                else if chars_vec[index] == '"' { //checking if the word is a string
                    word.clear();
                    while chars_vec[index + 1] != '"' {
                        word.push(chars_vec[index + 1]);
                        chars_vec.remove(index);
                    }
                    string_constant(word.borrow())
                }
                //checks if it is a symbol:
                //******note!!! -> the check for symbols must come before the check for identifier*****
                else if saved_symbols.contains(&&*word){ //checking if its a symbol
                    if word == "<"{
                        saved_symbol("&lt")
                    }
                    else if word ==">"{
                        saved_symbol("&gt")
                    }
                    else {
                        saved_symbol(word.borrow());
                    }
                    word.clear()
                }
                // checks if it is identifier:
                //******note!!! -> the check for symbols must come before the check for identifier*****
                else if saved_symbols.contains(&&*chars_vec[index + 1].to_string()) || chars_vec[index+1] ==' ' {
                    if word.parse::<i32>().is_ok() {
                        integer_constant(word.borrow())
                    }
                    else {
                        identifier(word.borrow());
                    }
                    word.clear();
                }
            }
            else {
                word.clear();
            }//if the word is \n or \t or white space it removes it
        }
    }
}
//I think by the name its obvious what dose functions do:
fn key_words(word: &str){
    println!("<Keyword> {} </Keyword>", word)
}

fn saved_symbol(symbol: &str){
    println!("<Symbol> {} </Symbol>", symbol)
}

fn identifier(identifier: &str){
    println!("<identifier> {} </identifier>", identifier)
}

fn string_constant(stringConstant: &str){
    println!("<stringConstant> {} </stringConstant>", stringConstant)
}

fn integer_constant(integerConstant: &str){
    println!("<integerConstant> {} </integerConstant>",integerConstant)
}