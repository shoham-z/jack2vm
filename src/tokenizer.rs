use std::fs;
use std::str::Chars;
use lazy_static::lazy_static;
use regex::Regex;
use throw::throw;
use crate::xmlwriter::Xmlwriter;

lazy_static! { // This was the only possible way to have global regex, without recompiling every time
    pub static ref COMMENT_REGEX: Regex = Regex::new(r"/\*\*.*\*/\n|//.*\n|/\*.*\*/\n").unwrap();
    pub static ref KEYWORD_REGEX: Regex = Regex::new(r"class\b|\bconstructor\b|\bfunction\b|\bmethod\b|\bfield\b|\bstatic\b|\bvar\b|\bint\b|\bchar\b|\bboolean\b|\bvoid\b|\btrue\b|\bfalse\b|\bnull\b|\bthis\b|\blet\b|\bdo\b|\bif\b|\belse\b|\bwhile\b|\breturn\b").unwrap();
    pub static ref SYMBOL_REGEX: Regex = Regex::new(r"\{|}|\(|\)|\b\[\b|\b]\b|\b\.\b|,|\b;|\+|-|\*|/|&|\b\|\b|<|>|=|\b~\b").unwrap();
    pub static ref IDENTIFIER_REGEX: Regex = Regex::new(r"\D\w*").unwrap();
    pub static ref INT_CONST_REGEX: Regex = Regex::new(r"(?m)\b\d{1,5}\b").unwrap(); // any 5 digit number
    pub static ref STRING_CONST_REGEX: Regex = Regex::new(r#"\"[^\"\n]*\""#).unwrap(); // any string literal (starts and ends with ")
}
//constant for type
pub const KEYWORD: i32 = 1;

pub const SYMBOL: i32 = 2;

pub const IDENTIFIER: i32 = 3;

pub const INT_CONST: i32 = 4;

pub const STRING_CONST: i32 = 5;

//constant for keyword
const CLASS: i32 = 10;

const METHOD: i32 = 11;

const FUNCTION: i32 = 12;

const CONSTRUCTOR: i32 = 13;

const INT: i32 = 14;

const BOOLEAN: i32 = 15;

const CHAR: i32 = 16;

const VOID: i32 = 17;

const VAR: i32 = 18;

const STATIC: i32 = 19;

const FIELD: i32 = 20;

const LET: i32 = 21;

const DO: i32 = 22;

const IF: i32 = 23;

const ELSE: i32 = 24;

const WHILE: i32 = 25;

const RETURN: i32 = 26;

const TRUE: i32 = 27;

const FALSE: i32 = 28;

const NULL: i32 = 29;

const THIS: i32 = 30;


pub struct JackTokenizer {
    xml_writer: Xmlwriter,
    pub current_token: String,
    pub current_token_type: i32,
    pub current_content: String,
    jack_code: Vec<char>,
    ptr: usize,
}

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
        let s = fs::read_to_string(path).unwrap();
        let mut tokenizer = JackTokenizer {
            xml_writer: Xmlwriter::new(path),
            current_token: "".to_string(),
            current_token_type: -1,
            current_content: String::new(),
            jack_code: COMMENT_REGEX.replace_all(s.as_str(), "").parse::<String>().unwrap().chars().collect(),
            ptr: s.len(),
        };
        tokenizer
    }

    /// Are there any more tokens in the input?
    ///
    /// # Returns
    ///
    /// * True if there are more tokens in the input, false otherwise
    pub fn has_more_tokens(&self) -> bool {
        self.ptr < self.jack_code.to_owned().len()
    }

    /// Gets the next token from the input, and makes it the current token.
    /// This method should only be called only if has_more_tokens is true.
    /// Initially there is no current token
    pub fn advance(&mut self) {
        if self.has_more_tokens() {
            self.current_content += self.jack_code[self.ptr].to_string().as_str();
            self.ptr += 1;
        } else {}
        if KEYWORD_REGEX.is_match(self.current_content.as_str()) {
            self.current_token_type = KEYWORD;
            self.current_token = "keyword".to_string();
        } else if SYMBOL_REGEX.is_match(self.current_content.as_str()) {
            self.current_token_type = SYMBOL;
            self.current_token = "symbol".to_string();
        } else if INT_CONST_REGEX.is_match(self.current_content.as_str()) {
            self.current_token_type = INT_CONST;
            self.current_token = "integerConstant".to_string();
        } else if STRING_CONST_REGEX.is_match(self.current_content.as_str()) {
            self.current_token_type = STRING_CONST;
            self.current_token = "stringConstant".to_string();
        } else if IDENTIFIER_REGEX.is_match(self.current_content.as_str()) {
            self.current_token_type = IDENTIFIER;
            self.current_token = "identifier".to_string();
        } else {
            println!("{}", (format!("Unknown token:{}", self.current_content)));
        }
        println!("{}", self.current_content);
    }

    /// Returns the type of the current token as a constant
    ///
    /// # Returns
    ///
    /// The token as constant (KEYWORD/SYMBOL/IDENTIFIER/INT_CONST/STRING_CONST)
    pub fn token_type(&mut self) -> i32 {
        return self.current_token_type;
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
            value = self.current_content.as_str();
        }
        value.to_string()
    }


    /// Returns the symbol which is the current token, as a constant.
    /// This method should only be called if tokenType is SYMBOL
    ///
    /// # Returns
    ///
    /// The current token
    pub fn symbol(&self) -> char {
        return self.current_content.chars().next().unwrap();
    }


    /// Returns the identifier which is the current token, as a constant.
    /// This method should only be called if tokenType is identifier
    ///
    /// # Returns
    ///
    /// The current token
    pub fn identifier(&self) -> String {
        return self.current_token.to_string();
    }


    /// Returns the integer value of the current token.
    /// This method should only be called if tokenType is INT_CONST
    ///
    /// # Returns
    ///
    /// The integer value of the current token
    pub fn int_val(&self) -> usize {
        return self.current_content.parse().unwrap();
    }


    /// Returns the string value of the current token, without hte two enclosing double quotes.
    /// This method should only be called if tokenType is STRING_CONST
    ///
    /// # Returns
    ///
    /// The string value of the current token
    pub fn string_val(&self) -> String {
        return self.current_token[1.. self.current_token.len() - 1].to_string();
    }

    /// Iterates over all characters in the jack file and tokenize them into xml file
    pub fn tokenize(&mut self) {
        while self.has_more_tokens() {
            self.advance();
            if self.current_token_type == KEYWORD {
                self.keyword();
            } else if self.current_token_type == SYMBOL {
                self.symbol();
            } else if self.current_token_type == STRING_CONST {
                self.string_val();
            } else if self.current_token_type == INT_CONST {
                self.int_val();
            } else if self.current_token_type == IDENTIFIER {
                self.identifier();
            } else { panic!("ERROR IN tokenizer.token_type()!"); }
            self.xml_writer.write(self.current_token.to_string(), self.current_content.to_string());
        }
        self.xml_writer.write_last();
    }
}
