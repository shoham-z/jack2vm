use std::fs::File;
use std::io::Write;

use crate::utility::{ADD, AND, EQ, GT, Kind, LT, MEMORY_AREAS, NEG, NOT, OR, SUB};

pub struct VMWriter {
    pub vm_file: File,
}


impl VMWriter {
    /// Creates a new vm file and prepares it for writing
    ///
    /// # Arguments
    ///
    /// * `path` - A path to the jack file, including the file extension
    ///
    /// # Returns
    ///
    /// * This self vmwriter object
    pub fn new(path: &String) -> Self {
        VMWriter {
            vm_file: File::create(path.to_owned().split(".jack").collect::<Vec<_>>()[0].to_owned() + ".vm").unwrap(),
        }
    }

    /// Writes a VM push command
    pub fn write_push(&mut self, segment: Kind, segmen: String, index: usize) {
        if segmen.is_empty() {
            let seg = match segment {
                Kind::STATIC => { "static" }
                Kind::FIELD => { "this" }
                Kind::ARG => { "argument" }
                Kind::VAR => { "local" }
                Kind::NONE => { "constant" }
            };
            self.vm_file.write(format!("push {} {}\n", seg, index).as_ref()).expect("write push failed");
        } else {
            if MEMORY_AREAS.contains(&&*segmen) {
                self.vm_file.write(format!("push {} {}\n", segmen, index).as_ref()).expect("write push failed");
            } else {
                self.vm_file.write(format!("push {}\n", segmen).as_ref()).expect("write push failed");
            }
        }
    }

    /// Writes a VM pop command - receives a constant in segment
    pub fn write_pop(&mut self, segment: Kind, segmen: String, index: usize) {
        if segmen.is_empty() {
            let seg = match segment {
                Kind::STATIC => { "static" }
                Kind::FIELD => { "this" }
                Kind::ARG => { "argument" }
                Kind::VAR => { "local" }
                Kind::NONE => { "constant" }
            };
            self.vm_file.write(format!("pop {} {}\n", seg, index).as_ref()).expect("write pop failed");
        } else { self.vm_file.write(format!("pop {} {}\n", segmen, index).as_ref()).expect("write pop failed"); }
    }

    /// Writes a VM arithmetic-logical command
    pub fn write_arithmetic(&mut self, command: usize) {
        match command {
            ADD => { self.vm_file.write("add\n".as_ref()).expect("TODO: panic message"); }
            SUB => { self.vm_file.write("sub\n".as_ref()).expect("TODO: panic message"); }
            NEG => { self.vm_file.write("neg\n".as_ref()).expect("TODO: panic message"); }
            EQ => { self.vm_file.write("eq\n".as_ref()).expect("TODO: panic message"); }
            GT => { self.vm_file.write("gt\n".as_ref()).expect("TODO: panic message"); }
            LT => { self.vm_file.write("lt\n".as_ref()).expect("TODO: panic message"); }
            AND => { self.vm_file.write("and\n".as_ref()).expect("TODO: panic message"); }
            OR => { self.vm_file.write("or\n".as_ref()).expect("TODO: panic message"); }
            NOT => { self.vm_file.write("not\n".as_ref()).expect("TODO: panic message"); }
            _ => {}
        };
    }

    /// Writes a VM label command
    pub fn write_label(&mut self, label: String) {
        self.vm_file.write(format!("label {}\n", label).as_ref()).expect("write label failed");
    }

    /// Writes a VM goto command
    pub fn write_goto(&mut self, label: String) {
        self.vm_file.write(format!("goto {}\n", label).as_ref()).expect("write goto failed");
    }

    /// Writes a VM if-goto command
    pub fn write_if(&mut self, label: String) {
        self.vm_file.write(format!("if-goto {}\n", label).as_ref()).expect("write if-goto failed");
    }

    /// Writes a VM call command
    pub fn write_call(&mut self, name: String, n_args: usize) {
        self.vm_file.write(format!("call {} {}\n", name, n_args).as_ref()).expect("write if-goto failed");
    }

    /// Writes a VM function command
    pub fn write_function(&mut self, name: String, n_args: usize) {
        self.vm_file.write(format!("function {} {}\n", name, n_args).as_ref()).expect("write function failed");
    }

    /// Writes a VM return command
    pub fn write_return(&mut self) {
        self.vm_file.write("return\n".as_ref()).expect("write return failed");
    }
}