use crate::utility::{Kind, Symbol};

pub struct SymbolTable {
    table: Vec<Symbol>,
    static_index: usize,
    field_index: usize,
    arg_index: usize,
    var_index: usize,
}

impl SymbolTable {
    /// Creates a new Symbol Table
    pub fn new() -> Self {
        SymbolTable {
            table: vec![],
            static_index: 0,
            field_index: 0,
            arg_index: 0,
            var_index: 0,
        }
    }

    /// Starts a new subroutine scope
    /// A.K.A. resets the table
    pub fn start_subroutine(&mut self) {
        self.table = Vec::new();
        self.static_index = 0;
        self.field_index = 0;
        self.arg_index = 0;
        self.var_index = 0;
    }

    /// Defines a new identifier og hte given name, type, and kind, and assigns it an index.
    /// STATIC and FIELD identifiers have a class scope, while ARG and VAR identifiers have a subroutine scope
    pub fn define(&mut self, name: String, data_type: String, kind: Kind) {
        let index: usize;
        match kind {
            Kind::STATIC => {
                index = self.static_index;
                self.static_index += 1
            }
            Kind::FIELD => {
                index = self.field_index;
                self.field_index += 1
            }
            Kind::ARG => {
                index = self.arg_index;
                self.arg_index += 1
            }
            Kind::VAR => {
                index = self.var_index;
                self.var_index += 1
            }
            Kind::NONE => { panic!("invalid Kind of variable!!") }
        };
        self.table.push(Symbol::new(name, data_type, kind, index));
    }

    /// Returns the number of variable of the given kind already defined in current scope
    pub fn var_count(&self, kind: Kind) -> usize {
        match kind {
            Kind::STATIC => { self.static_index }
            Kind::FIELD => { self.field_index }
            Kind::ARG => { self.arg_index }
            Kind::VAR => { self.var_index }
            Kind::NONE => { panic!("invalid Kind of variable!!") }
        }
    }

    /// Returns the kind of the names identifier in the current scope.
    /// If the identifier is unknown in the current scope, returns NONE.
    pub fn kind_of(&self, name: String) -> Kind {
        let mut kind: Kind = Kind::NONE;
        for symbol in self.table.iter() {
            if symbol.get_name() == name {
                kind = symbol.get_kind();
                break;
            }
        }
        kind
    }

    /// Returns the type of the names identifier in the scope.
    pub fn type_of(&self, name: String) -> String {
        let mut data = "".to_string();
        for symbol in self.table.iter() {
            if symbol.get_name() == name {
                data = symbol.get_data_type();
                break;
            }
        }
        data
    }

    /// Returns the index assigned to the named identifier.
    pub fn index_of(&self, name: String) -> usize {
        let mut index = usize::MAX;
        for symbol in self.table.iter() {
            if symbol.get_name() == name {
                index = symbol.get_index();
                break;
            }
        }
        index
    }
}