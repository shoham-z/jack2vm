use std::borrow::{Borrow, Cow};
use std::fs;
use regex::Regex;
use crate::XmlWriter;

lazy_static!{

}




pub fn tokenizer(xml_file_path: String) {

    let saved_key_words:Vec<&str> = vec!["class", "constructor", "function", "method", "field", "static", "var", "int", "char", "boolean", "void", "true", "false", "null", "this", "let", "do", "if", "else", "while", "return"];
    let saved_symbols:Vec<&str> = vec![";", "-", "=", "+", "/", ".", "{", "}", "(", ")", "[", "]", "<", ">", "&", "|", "*", ",", "~"];

    //this way we kick out all the comments:
    let regex_no_comments:Regex = Regex::new(r#"/\*\*.*\*/|//.*\n|/\*.*\*/\n\*/"#).unwrap();

    let mut xml_writer:XmlWriter = XmlWriter::new(&xml_file_path);

    //reading the data and *it has to be owned other ways regex will not be able to use it*:
    let file_raw_data =fs::read_to_string(xml_file_path).unwrap().as_str().to_owned();
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
    let saved_symbols = vec![";","-","=","+", "*", "/",".","{","}","(",")","[","]","<",">","&","|", ",", "~"];
    let mut word =String::new();
    for mut index in 0..(chars_vec.len()-1)  {
        //checking if the index is oversize the array:
        if index <chars_vec.len(){
            if chars_vec[index] == '/' && chars_vec[index+1] == '*'{
                while !(chars_vec[index+1] == '/'){
                    chars_vec.remove(index);
                }
                chars_vec.remove(index);
                chars_vec.remove(index);
            }
            word.push(chars_vec[index]); //combining all the chars until we get a valid word\symbol\identifier
            if !(word.contains("\t") || word.contains("\n")) {
                //checking if the word is a Key word
                if saved_key_words.contains(&&*word) && (saved_symbols.contains(&&*chars_vec[index+1].to_string())  || (chars_vec[index+1] == ' ')) {
                    xml_writer.write("keyword".to_string(),word.to_string());
                    word.clear();
                }
                else if word == " " {
                    word.clear()
                }
                //checking if the word is a string:
                else if chars_vec[index] == '"' { //checking if the word is a string
                    word.clear();
                    chars_vec.remove(index);
                    while chars_vec[index] != '"' {
                        word.push(chars_vec[index]);
                        chars_vec.remove(index);
                    }
                    xml_writer.write("stringConstant".to_string(),word.to_string());
                    word.clear();
                }
                //checking if its a symbol:
                //******note!!! -> the check for symbols most come before the check for identifier*****
                else if saved_symbols.contains(&&*word) { //checking if its a symbol
                    if word == "<"{
                        xml_writer.write("symbol".to_string(), "&lt;".to_string());
                    }
                    else if word ==">"{
                        xml_writer.write("symbol".to_string(), "&gt;".to_string());
                    }
                    else if word == "&" {
                        xml_writer.write("symbol".to_string(), "&amp;".to_string());
                    }
                    else if word == "\"" {
                        xml_writer.write("symbol".to_string(), "&quet;".to_string());
                    }
                    else {
                        xml_writer.write("symbol".to_string(), word.to_string());
                    }
                    word.clear()
                }
                // checking if its identifier:
                //******note!!! -> the check for symbols most come before the check for identifier*****
                else if saved_symbols.contains(&&*chars_vec[index + 1].to_string()) || chars_vec[index+1] ==' ' {
                    if word.parse::<i32>().is_ok() {
                        xml_writer.write("integerConstant".to_string(), word.to_string());
                    }
                    else {
                        xml_writer.write("identifier".to_string(), word.to_string());
                    }
                    word.clear();
                }
            }
            else {
                word.clear();
            }//if the word is \n or \t or white space it removes it
        }
    }

    xml_writer.write_last();
}