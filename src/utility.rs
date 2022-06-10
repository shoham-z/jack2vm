/// This file contains all the constants/data types that i have defined, along the proposed implementation
#[derive(Clone, Copy, PartialEq)]
pub enum Kind {
    STATIC,
    FIELD,
    ARG,
    VAR,
    NONE,
}

pub const ADD: usize = 1;
pub const SUB: usize = 2;
pub const NEG: usize = 3;
pub const EQ: usize = 4;
pub const GT: usize = 5;
pub const LT: usize = 6;
pub const AND: usize = 7;
pub const OR: usize = 8;
pub const NOT: usize = 9;

pub static BUILT_IN_CLASSES: [&str; 8] = ["Array", "Keyboard", "Math", "Memory", "Output", "Screen", "String", "Sys"];
pub static MEMORY_AREAS: [&str; 8] = ["static", "this", "that", "local", "argument", "constant", "pointer", "temp"];
pub static KEYWORD_CONSTANT: [&str; 4] = ["true", "false", "null", "this"];
pub static CLASS_VAR_TYPES: [&str; 2] = ["static", "field"];
pub static DATA_TYPES: [&str; 4] = ["int", "boolean", "char", "void"];
pub static OP: [&str; 9] = ["+", "-", "*", "/", "&", "|", "<", ">", "="];
pub static UNARY_OP: [&str; 2] = ["-", "~"];
pub static CLASS_FUNC_TYPES: [&str; 3] = ["function", "method", "constructor"];


/// Struct for cleaner code.
pub struct Symbol {
    name: String,
    data_type: String,
    kind: Kind,
    index: usize,
}

impl Symbol {
    /// Constructor for Symbol. For cleaner code.
    pub fn new(name: String, data_type: String, kind: Kind, index: usize) -> Self {
        Symbol {
            name,
            data_type,
            kind,
            index,
        }
    }

    /// Getter for name of the symbol
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    /// Getter for kind of the symbol
    pub fn get_kind(&self) -> Kind {
        match self.kind {
            Kind::STATIC => { Kind::STATIC }
            Kind::FIELD => { Kind::FIELD }
            Kind::ARG => { Kind::ARG }
            Kind::VAR => { Kind::VAR }
            Kind::NONE => { Kind::NONE }
        }
    }

    /// Getter for data type of the symbol
    pub fn get_data_type(&self) -> String {
        self.data_type.to_string()
    }

    /// Getter for index of the symbol
    pub fn get_index(&self) -> usize {
        self.index
    }
}