use std::fs;
use regex::Regex;
use crate::xmlwriter::XmlWriter;


pub struct JackTokenizer {
    buffer:Vec<char>,
    token_content:String,
    token_type:usize,
    index:usize,
    xml_writer:XmlWriter
}

const KEYWORD: usize = 1;

const SYMBOL: usize = 2;

const IDENTIFIER: usize = 3;

const INT_CONST: usize = 4;

const STRING_CONST: usize = 5;

impl JackTokenizer {
    /// Opens a jack file and gets ready to tokenize it
    ///
    /// # Arguments
    ///
    /// * `path` - A path to the jack file, including the file extension
    ///
    /// # Returns
    ///
    /// * The newly created JackTokenizer object
    pub fn new(path: &String) -> Self {
        let regex_no_comments:Regex = Regex::new(r#"/\*\*.*\*/|//.*\n|/\*.*\*/\n\*/"#).unwrap();
        //reading the data *it had to be owned otherwise regex will not be able to use it*:
        let file_raw_data =fs::read_to_string(path).unwrap().as_str().to_owned();
        //non readable data that's way next line i transferred it to chars:
        let after_no_comments = regex_no_comments.replace_all(&file_raw_data, "");
        //a vector for all the chars:
        let mut buffer:Vec<char> =vec![];
        for text in after_no_comments.chars() {
            print!("{}", text);
            if text != '\n' || text != ' ' {
                buffer.push(text);
            }
        }
        let tokenizer = JackTokenizer {
            buffer,
            token_content: "".to_string(),
            token_type: 0,
            index: 0,
            xml_writer: XmlWriter::new(path)
        };
        tokenizer
    }

    /// Are there any more tokens in the input?
    ///
    /// # Returns
    ///
    /// * True if there are more tokens in the input, false otherwise
    pub fn has_more_tokens(&self) -> bool {
        self.index < self.buffer.to_owned().len()
    }

    /// Gets the next token from the input, and makes it the current token.
    /// This method should only be called only if has_more_tokens is true.
    /// Initially there is no current token
    pub fn advance(&mut self) {
        let saved_key_words:Vec<&str> = vec!["class", "constructor", "function", "method", "field", "static", "var", "int", "char", "boolean", "void", "true", "false", "null", "this", "let", "do", "if", "else", "while", "return"];
        let saved_symbols:Vec<&str> = vec![";", "-", "=", "+", "/", ".", "{", "}", "(", ")", "[", "]", "<", ">", "&", "|", "*", ",", "~"];
        //checking if the index is oversize the array:
        if self.has_more_tokens(){
            if self.buffer[self.index] == '/' && self.buffer[self.index+1] == '*'{
                while !(self.buffer[self.index+1] == '/'){
                    self.buffer.remove(self.index);
                }
                self.buffer.remove(self.index);
                self.buffer.remove(self.index);
            }
            self.token_content.push(self.buffer[self.index]); //combining all the chars until we get a valid word\symbol\identifier
            if !(self.token_content.contains("\t") || self.token_content.contains("\n")) {
                //checking if the word is a Key word
                if saved_key_words.contains(&&*self.token_content) && (saved_symbols.contains(&&*self.buffer[self.index+1].to_string())  || (self.buffer[self.index+1] == ' ')) {
                    self.token_type = KEYWORD;
                    self.token_content.clear();
                }
                else if self.token_content == " " {
                    self.token_content.clear()
                }
                //checking if the word is a string:
                else if self.buffer[self.index] == '"' { //checking if the word is a string
                    self.token_content.clear();
                    self.buffer.remove(self.index);
                    while self.buffer[self.index] != '"' {
                        self.token_content.push(self.buffer[self.index]);
                        self.buffer.remove(self.index);
                    }
                    self.token_type = STRING_CONST;
                    self.token_content.clear();
                }
                //checking if its a symbol:
                //******note!!! -> the check for symbols most come before the check for identifier*****
                else if saved_symbols.contains(&&*self.token_content) { //checking if its a symbol
                    self.token_type = SYMBOL;
                }
                // checking if its identifier:
                //******note!!! -> the check for symbols most come before the check for identifier*****
                else if saved_symbols.contains(&&*self.buffer[self.index + 1].to_string()) || self.buffer[self.index+1] ==' ' {
                    if self.token_content.parse::<i32>().is_ok() {
                        self.token_type = INT_CONST;
                    }
                    else {
                        self.token_type = IDENTIFIER;
                    }
                    self.token_content.clear();
                }
            }
            else {
                self.token_content.clear();
            }//if the word is \n or \t or white space it removes it
        }
    }


    /// Returns the type of the current token as a constant
    ///
    /// # Returns
    ///
    /// The token as constant (KEYWORD/SYMBOL/IDENTIFIER/INT_CONST/STRING_CONST)
    pub fn token_type(&mut self) -> usize {
        return self.token_type;
    }

    /// Returns the KEYWORD which is the current token, as a constant.
    /// This method should only be called if tokenType is KEYWORD
    ///
    /// # Returns
    ///
    /// The current token
    pub fn keyword(&self) -> String {
        let mut value: &str = "";

        if value == "<" {
            value = "&lt;"
        } else if value == ">" {
            value = "&gt;"
        } else if value == "\"" {
            value = "&quet;"
        } else if value == "&" {
            value = "&amp;"
        }else{
            value = self.token_content.as_str();
        }
        value.to_string()
    }


    /// Returns the symbol which is the current token, as a constant.
    /// This method should only be called if tokenType is SYMBOL
    ///
    /// # Returns
    ///
    /// The current token
    pub fn symbol(&self) -> String {
        return self.token_content.to_string();
    }


    /// Returns the identifier which is the current token, as a constant.
    /// This method should only be called if tokenType is identifier
    ///
    /// # Returns
    ///
    /// The current token
    pub fn identifier(&self) -> String {
        return self.token_content.to_string();
    }


    /// Returns the integer value of the current token.
    /// This method should only be called if tokenType is INT_CONST
    ///
    /// # Returns
    ///
    /// The integer value of the current token
    pub fn int_val(&self) -> String {
        return self.token_content.parse::<i32>().unwrap().to_string();
    }


    /// Returns the string value of the current token, without hte two enclosing double quotes.
    /// This method should only be called if tokenType is STRING_CONST
    ///
    /// # Returns
    ///
    /// The string value of the current token
    pub fn string_val(&self) -> String {
        return self.token_content[1.. self.token_content.len() - 1].to_string();
    }

    /// Iterates over all characters in the jack file and tokenize them into xml file
    pub fn tokenize(&mut self) {
        let mut content = String::new();
        let mut tag = "";
        while self.has_more_tokens() {
            self.advance();
            if self.token_type == KEYWORD {
                tag = "keyword";
                content = self.keyword();
            } else if self.token_type == SYMBOL {
                tag = "symbol";
                content = self.symbol();
            } else if self.token_type == STRING_CONST {
                tag = "stringConstant";
                content = self.string_val();
            } else if self.token_type == INT_CONST {
                tag = "integerConstant";
                content = self.int_val();
            } else if self.token_type == IDENTIFIER {
                tag = "identifier";
                content = self.identifier();
            } else { panic!("ERROR IN tokenizer.token_type()!"); }
            self.xml_writer.write(tag.to_string(), content);
        }
        self.xml_writer.write_last();
    }
}

