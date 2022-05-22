use std::borrow::Borrow;
use std::env::vars;
use std::fs;
use std::io::BufRead;
use regex::Regex;
use crate::xmlwriter::XmlWriter;

static CLASS_VAR_TYPES: [&str; 2] = ["static", "method"];
static OP: [&str; 5] = ["-", "=", "+", "<", ">"];
static DATA_TYPES: [&str; 6] = ["int", "boolean", "char", "String", "Array", "void"];
static CLASS_FUNC_TYPES: [&str; 3] = ["function", "method", "constructor"];
static SAVED_SYMBOLS: [&str; 19] = [";", "-", "=", "+", "/", ".", "{", "}", "(", ")", "[", "]", "<", ">", "&", "|", "*", ",", "~"];
static SAVED_KEYWORDS: [&str; 21] = ["class", "constructor", "function", "method", "field", "static", "var", "int", "char", "boolean", "void", "true", "false", "null", "this", "let", "do", "if", "else", "while", "return"];


pub struct CompilationEngine {
    output_file: XmlWriter,
    input_file: String,
}

impl CompilationEngine {
    /// Opens a jack file and gets ready to tokenize it
    ///
    /// # Arguments
    ///
    /// * `path` - A path to the jack file, including the file extension
    ///
    /// # Returns
    ///
    /// * The newly created CompilationEngine object
    pub fn new(path: &String) -> Self {
        let remove_comments: Regex = Regex::new(r#"/\*\*.*\*/|//.*\n|/\*.*\*/\n\*/"#).unwrap();
        let file_contents = fs::read_to_string(path).unwrap().as_str().to_owned();

        let code = remove_comments.replace_all(&file_contents, "").to_string();

        let lines = code.split("\n");
        for line in lines {
            if line.starts_with("//") { // | line.contains("//"){
                let _ = code.replace(line, "");
            } else if line.starts_with("/*") | line.starts_with("/**") {
                while !line.contains("*/") {
                    let _ = code.replace(line, "");
                }
            }
        }

        CompilationEngine {
            output_file: XmlWriter::new(&(path.to_owned().split(".jack").collect::<Vec<_>>()[0].to_owned() + "My.jack")),
            input_file: code,
        }
    }

    /// Compiles a complete class.
    pub fn compile_class(&mut self) {
        self.output_file.open_tag("class".to_string());

        let code = self.input_file.to_string();

        let lines = code.split("\n");

        for line in lines {
            //println!("{}", line.trim_start());
            let trimmed_line = line.trim_start();
            let first_word = trimmed_line.split(" ").nth(0).unwrap();
            if first_word == "class" {
                self.output_file.write("keyword".to_string(), first_word.to_string());//class keyword

                self.output_file.write("identifier".to_string(), trimmed_line.split(" ").nth(1).unwrap().to_string());//class name

                self.output_file.write("symbol".to_string(), "{".to_string());//opening bracket
            } else if CLASS_VAR_TYPES.contains(&first_word) {
                self.compile_class_var_dec(line.borrow().to_string());
            }
        }

        let mut lines = code.split("\n");

        let count = code.split("\n").count();
        println!("{}", count);

        let mut func_contents: String = "".to_string();
        for mut index in 1..count {
            let current_line = lines.next().unwrap();
            //println!("{}", index);
            //println!("{:?}", current_line);


            let cleared_carriage_return = current_line.replace("\r", "");
            let trimmed_line = cleared_carriage_return.trim_start();
            let mut first_word = trimmed_line.split(" ").nth(0).unwrap();
            let mut is_func = false;
            for item in CLASS_FUNC_TYPES{
                if item==first_word{
                    is_func = true;
                    break;
                }
            }
            if is_func {
                func_contents += trimmed_line;
                let mut current_line = "";
                index+=1;
                match lines.next() {
                    None => {}
                    Some(value) => { current_line = value.trim_start(); }
                }
                //println!("we're almost there");

                while !current_line.contains("}") && index<count {
                    index+=1;
                    if !current_line.is_empty(){
                        func_contents += current_line;
                        current_line = lines.next().unwrap();
                        //println!("good job shoham");
                    }

                }
                self.compile_subroutine_dec(func_contents.to_string());
            }
        }
        self.output_file.close_tag("class".to_string());
    }

    /// Compiles a static variable declaration or field declaration.
    fn compile_class_var_dec(&mut self, line: String) {
        self.output_file.open_tag("classVarDec".to_string());

        let mut words = line.split(" ");

        for word in words.to_owned() {
            if CLASS_VAR_TYPES.contains(&word) {
                self.output_file.write("keyword".to_string(), word.to_string());
            } else if DATA_TYPES.contains(&word) {
                self.output_file.write("keyword".to_string(), word.to_string());
            } else if word.contains(";") {
                let mut word2 = word.split(";");
                self.output_file.write("identifier".to_string(), word2.nth(0).unwrap().to_string());
                self.output_file.write("symbol".to_string(), ";".to_string());
            } else if word.contains(",") {
                let mut word2 = word.split(";");
                self.output_file.write("identifier".to_string(), word2.nth(0).unwrap().to_string());
                self.output_file.write("symbol".to_string(), ",".to_string());
            }
        }

        let mut next: &str;

        //let attempt = line.split(",");

        //println!("{:?}", attempt);
        //println!("{:?}", attempt.size_hint());

        let attempt = line.split(",");
        if attempt.size_hint().0 > 0 {
            loop {
                next = words.next().unwrap();
                if next == ";" {
                    self.output_file.write("symbol".to_string(), next.to_string());
                    break;
                } else if next == "," {
                    self.output_file.write("symbol".to_string(), next.to_string());
                } else {
                    self.output_file.write("identifier".to_string(), words.next().unwrap().to_string());
                }
            }
        }

        self.output_file.close_tag("classVarDec".to_string());
    }

    /// Compiles a complete method, function or constructor.
    fn compile_subroutine_dec(&mut self, content: String) {
        println!("{}",content);

        self.output_file.open_tag("subroutineDec".to_string());
        let mut lines = content.split("\n");
        let mut words = lines.next().unwrap().split(" ");
        for word in words {
            if !word.is_empty() {
                if !word.contains("{")
                {
                    if !word.contains("(") {
                        self.output_file.write("keyword".to_string(), word.to_string()); // type or return type of the function
                    } else {
                        let mut id = word.split("(");
                        self.output_file.write("identifier".to_string(), id.nth(0).unwrap().to_string()); // name of function
                        self.output_file.write("symbol".to_string(), "(".to_string());
                        let bruh = id.nth(1);
                        let dude;
                        let mut params = "";
                        match bruh {
                            None => {}
                            Some(value) => {
                                dude = value.split(")").nth(0);
                                match dude {
                                    None => {}
                                    Some(value) => { params = value }
                                };
                            }
                        };
                        self.compile_parameter_list(params.to_string());


                        self.output_file.write("symbol".to_string(), ")".to_string());
                    }
                }
            }
        }

        let body = content.split("{").nth(1).unwrap().split("}").nth(0).unwrap();

        println!("{}", body);

        self.compile_subroutine_body(body.to_string());

        self.output_file.close_tag("subroutineDec".to_string());
    }

    /// Compiles a (possibly empty) parameter list.
    /// Does not handle the enclosing "()".
    fn compile_parameter_list(&mut self, content: String) {
        self.output_file.open_tag("parameterList".to_string());

        if content.len() > 0 {
            let mut vars = content.split(",");

            let count = vars.count();

            vars = content.split(",");

            if count == 0 { return; }

            let times = count - 2; // one to get to the end of the collection, and another one to not write a ',' after the last parameter

            if count > 1 {
                let mut var_split;
                for _index in 1..times {
                    var_split = vars.next().unwrap().split(" ");

                    self.output_file.write("keyword".to_string(), var_split.next().unwrap().to_string());

                    self.output_file.write("identifier".to_string(), var_split.next().unwrap().to_string());

                    self.output_file.write("symbol".to_string(), ",".to_string());
                }
            }
            let mut var_split = vars.next().unwrap().split(" ");

            self.output_file.write("keyword".to_string(), var_split.next().unwrap().to_string());

            self.output_file.write("identifier".to_string(), var_split.next().unwrap().to_string());
        }
        self.output_file.close_tag("parameterList".to_string());
    }

    /// Compiles a subroutine's body.
    fn compile_subroutine_body(&mut self, content: String) {
        self.output_file.open_tag("subroutineBody".to_string());

        self.output_file.write("symbol".to_string(), "{".to_string());

        self.compile_statements(content.replace("{", "").replace("}", ""));

        self.output_file.write("symbol".to_string(), "}".to_string());

        self.output_file.close_tag("subroutineBody".to_string());
    }

    /// Compiles a var declaration.
    fn compile_var_dec(&mut self, content: String) {
        self.output_file.open_tag("varDec".to_string());


        self.output_file.close_tag("varDec".to_string());
    }

    /// Compiles a sequence of statements.
    /// Does not handle the enclosing "()".
    fn compile_statements(&mut self, content: String) {
        self.output_file.open_tag("statements".to_string());


        self.output_file.close_tag("statements".to_string());
    }

    /// Compiles a let statement.
    fn compile_let(&mut self, content: String) {
        self.output_file.open_tag("letStatement".to_string());


        self.output_file.close_tag("letStatement".to_string());
    }

    /// Compiles an if statement, possible with a trailing else clause.
    fn compile_if(&mut self, content: String) {
        self.output_file.open_tag("ifStatement".to_string());


        self.output_file.close_tag("ifStatement".to_string());
    }

    /// Compiles a while statement.
    fn compile_while(&mut self, content: String) {
        self.output_file.open_tag("whileStatement".to_string());


        self.output_file.close_tag("whileStatement".to_string());
    }

    /// Compiles a do statement.
    fn compile_do(&mut self, content: String) {
        self.output_file.open_tag("doStatement".to_string());


        self.output_file.close_tag("doStatement".to_string());
    }

    /// Compiles a return statement.
    fn compile_return(&mut self, content: String) {
        self.output_file.open_tag("returnStatement".to_string());


        self.output_file.close_tag("returnStatement".to_string());
    }

    /// Compiles an expression.
    fn compile_expression(&mut self, content: String) {
        self.output_file.open_tag("expression".to_string());


        self.output_file.close_tag("expression".to_string());
    }

    /// Compiles a term.
    /// If the current token is an identifier, the routine must distinguish between a variable,
    /// an array-entry, or a subroutine-call.
    /// A single look-ahead token, which may be one of "[" , "(" or ".",
    /// suffices to distinguish between the possibilities
    /// Any other token is not part of this term and should not be advanced over.
    fn compile_term(&mut self, content: String) {
        self.output_file.open_tag("term".to_string());


        self.output_file.close_tag("term".to_string());
    }

    /// Compiles a (possibly empty) comma-seperated list of expressions.
    fn compile_expression_list(&mut self, content: String) {
        self.output_file.open_tag("expressionList".to_string());

        let expressions = content.split(",");

        for expression in expressions {
            self.compile_expression(expression.to_string());
        }

        self.output_file.close_tag("expressionList".to_string());
    }
}