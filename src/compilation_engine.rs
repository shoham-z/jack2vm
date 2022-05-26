use std::fs;
use std::ops::{Index, Sub};

use regex::Regex;

use crate::xmlwriter::XmlWriter;

static CLASS_VAR_TYPES: [&str; 2] = ["static", "field"];
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

        let code = remove_comments.replace_all(&file_contents, "\n").to_string();

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
        let lines = code.lines();

        let count = lines.count();
        let mut lines = code.lines();

        for _index in 1..count {
            let line = lines.nth(0).unwrap();
            let trimmed_line = line.trim_start();
            let first_word = trimmed_line.split(" ").nth(0).unwrap();
            if first_word == "class" {
                self.output_file.write("keyword".to_string(), first_word.to_string());//class keyword

                self.output_file.write("identifier".to_string(), trimmed_line.split(" ").nth(1).unwrap().to_string());//class name

                self.output_file.write("symbol".to_string(), "{".to_string());//opening bracket
                break;
            }
        }


        let mut last_func = "";
        let mut class_contents = code.get(code.find("{").unwrap()..code.rfind("}").unwrap() + 1).unwrap().lines();
        //to get all the content of the class, such as fields, statics, constructors, methods and functions

        for line in class_contents {
            let mut words = line.split_whitespace();
            let tmp = words.nth(0);
            let mut first_word = "";
            match tmp {
                None => {}
                Some(value) => { first_word = value; }
            }
            if CLASS_VAR_TYPES.contains(&first_word) {
                self.compile_class_var_dec(line.to_string());
            }
            if CLASS_FUNC_TYPES.contains(&first_word) {
                let mut func_contents: &str = "";
                for cloned_line in lines.clone() {
                    let mut words = cloned_line.split_whitespace();
                    let tmp = words.nth(0);
                    let mut first_word = "";
                    match tmp {
                        None => {}
                        Some(value) => { first_word = value; }
                    }
                    if CLASS_FUNC_TYPES.contains(&first_word) && code.find(line).unwrap() < code.find(cloned_line).unwrap() {
                        func_contents = code.get(code.find(line).unwrap()..code.find(cloned_line).unwrap()).unwrap();
                        self.compile_subroutine_dec(func_contents.to_string());
                        last_func = func_contents;
                    } else if last_func != func_contents {
                        func_contents = code.get(code.find(line).unwrap()..code.len() - 5).unwrap();
                        self.compile_subroutine_dec(func_contents.to_string());
                        last_func = func_contents;
                    } else {
                        func_contents = code.get(code.find(line).unwrap()..code.len() - 1).unwrap();
                        self.compile_subroutine_dec(func_contents.to_string());
                        break;
                    }
                }
            }
        }


        self.output_file.write("symbol".to_string(), "}".to_string());//closing bracket

        self.output_file.close_tag("class".to_string());
    }

    /// Compiles a static variable declaration or field declaration.
    fn compile_class_var_dec(&mut self, line: String) {
        self.output_file.open_tag("classVarDec".to_string());

        let mut words = line.split_whitespace();

        self.output_file.write("keyword".to_string(), words.nth(0).unwrap().to_string());

        self.output_file.write("keyword".to_string(), words.nth(0).unwrap().to_string());

        let mut comma = line.find(",");

        if let Some(_value) = comma {

            // TO DO: handle more than one var name
            // EXAMPLE: "field int i, sum;"
            let mut var_name = words.next().unwrap();

            while let Some(_value) = comma {
                self.output_file.write("identifier".to_string(), var_name.get(0..var_name.find(',').unwrap()).unwrap().to_string());
                self.output_file.write("symbol".to_string(), ",".to_string());
                var_name = words.next().unwrap();

                comma = var_name.find(",");
            }
            self.output_file.write("identifier".to_string(), var_name.get(0..var_name.find(';').unwrap()).unwrap().to_string());
            self.output_file.write("symbol".to_string(), ";".to_string());
        } else {
            let var_name = words.nth(0).unwrap();

            self.output_file.write("identifier".to_string(), var_name.get(0..var_name.len() - 1).unwrap().to_string());

            self.output_file.write("symbol".to_string(), ";".to_string());

            //for word in content.split_whitespace(){println!("{}",word);}
        }

        let mut next: &str;

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
    pub fn compile_subroutine_dec(&mut self, content: String) {
        self.output_file.open_tag("subroutineDec".to_string());

        let func_dec = content.get(0..content.find("{").unwrap()).unwrap();

        let mut words = func_dec.split_whitespace();

        self.output_file.write("keyword".to_string(), words.nth(0).unwrap().to_string());//subroutine type - constructor/method/function keyword

        self.output_file.write("keyword".to_string(), words.nth(0).unwrap().to_string());//subroutine return type

        self.output_file.write("identifier".to_string(), words.nth(0).unwrap().split("(").nth(0).unwrap().to_string());//subroutine name

        self.output_file.write("symbol".to_string(), "(".to_string());

        let param_list = func_dec.get(content.find("(").unwrap() + 1..content.find(")").unwrap()).unwrap();

        self.compile_parameter_list(param_list.to_string());

        self.output_file.write("symbol".to_string(), ")".to_string());

        let func_body = content.get(content.find("{").unwrap()..content.rfind("}").unwrap() + 1).unwrap();

        self.compile_subroutine_body(func_body.to_string());

        self.output_file.close_tag("subroutineDec".to_string());
    }

    /// Compiles a (possibly empty) parameter list.
    /// Does not handle the enclosing "()".
    fn compile_parameter_list(&mut self, content: String) {
        self.output_file.open_tag("parameterList".to_string());
        let mut var_split;
        let mut first_time = true;
        if content.len() > 0 {
            let mut vars = content.split(",");

            let count = content.find(",");

            if count.is_some() {
                let times = count.unwrap() - 1; // one to get to the end of the collection
                let mut var_split;
                for _index in 1..times {
                    let temp = vars.next();

                    match temp {
                        None => { break; }
                        Some(value) => {
                            if !first_time {
                                self.output_file.write("symbol".to_string(), ",".to_string());
                            } else { first_time = false; }

                            var_split = value.split_whitespace();
                        }
                    }

                    self.output_file.write("keyword".to_string(), var_split.next().unwrap().to_string());

                    self.output_file.write("identifier".to_string(), var_split.next().unwrap().to_string());
                }
            }
            let temp = vars.next();

            match temp {
                None => {}
                Some(value) => {
                    var_split = value.split_whitespace();
                    self.output_file.write("keyword".to_string(), var_split.next().unwrap().to_string());

                    self.output_file.write("identifier".to_string(), var_split.next().unwrap().to_string());
                }
            }
        }
        self.output_file.close_tag("parameterList".to_string());
    }

    /// Compiles a subroutine's body.
    fn compile_subroutine_body(&mut self, content: String) {
        self.output_file.open_tag("subroutineBody".to_string());

        self.output_file.write("symbol".to_string(), "{".to_string());

        let body_content = content.get(content.find("{").unwrap() + 1..content.rfind("}").unwrap()).unwrap();
        //println!("{:?}", body_content);
        let mut stop_sign = "";
        let lines = body_content.lines();
        for line in lines {
            if line != "" {
                if let Some(value) = line.trim().split_whitespace().nth(0) {
                    if value == "var" {
                        self.compile_var_dec(line.trim().to_string());
                    } else {
                        stop_sign = value;
                        break;
                    }
                }
            }
        }

        self.compile_statements(body_content.to_string().get(body_content.find(stop_sign).unwrap()..body_content.len() - 1).unwrap().to_string());

        self.output_file.write("symbol".to_string(), "}".to_string());

        self.output_file.close_tag("subroutineBody".to_string());
    }

    /// Compiles a var declaration.
    fn compile_var_dec(&mut self, content: String) {
        self.output_file.open_tag("varDec".to_string());

        self.output_file.write("keyword".to_string(), "var".to_string());

        let mut words = content.split_whitespace();

        self.output_file.write("keyword".to_string(), words.nth(1).unwrap().to_string());

        let mut comma = content.find(",");

        if let Some(_value) = comma {

            // TO DO: handle more than one var name
            // EXAMPLE: "var int i, sum;"
            let mut var_name = words.next().unwrap();

            while let Some(_value) = comma {
                self.output_file.write("identifier".to_string(), var_name.get(0..var_name.find(',').unwrap()).unwrap().to_string());
                self.output_file.write("symbol".to_string(), ",".to_string());
                var_name = words.next().unwrap();

                comma = var_name.find(",");
            }
            self.output_file.write("identifier".to_string(), var_name.get(0..var_name.find(';').unwrap()).unwrap().to_string());
            self.output_file.write("symbol".to_string(), ";".to_string());
        } else {
            let var_name = words.nth(0).unwrap();

            self.output_file.write("identifier".to_string(), var_name.get(0..var_name.len() - 1).unwrap().to_string());

            self.output_file.write("symbol".to_string(), ";".to_string());

            //for word in content.split_whitespace(){println!("{}",word);}
        }

        //This is possible and should be handled: "var int i, sum;"

        self.output_file.close_tag("varDec".to_string());
    }

    /// Compiles a sequence of statements.
    /// Does not handle the enclosing "()".
    fn compile_statements(&mut self, content: String) {
        self.output_file.open_tag("statements".to_string());
        //println!("{}", content);

        let lines = content.lines();
        let mut first_word = "";
        let mut temp;
        for line in lines {
            temp = line.trim().split_whitespace().nth(0);
            match temp{
                None => {}
                Some(value) => {first_word = value;}
            }
            if first_word == "let" {
                self.compile_let(line.trim().to_string());
            } else if first_word == "do" {
                self.compile_do(line.trim().to_string());
            } else if first_word == "while" {
                self.compile_while(line.trim().to_string());
            } else if first_word == "if" {
                self.compile_if(line.trim().to_string());
            } else if ["return", "return;"].contains(&first_word) {
                self.compile_return(line.trim().to_string());
            }
            first_word = "";
        }


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

        self.output_file.write("keyword".to_string(), "do".to_string());

        let do_content = content.get(content.trim().find(" ").unwrap()+1..content.trim().len()-1).unwrap();

        let dot = do_content.find('.');

        if let Some(_value) = dot{
            self.output_file.write("identifier".to_string(), do_content.get(0..do_content.find(".").unwrap()).unwrap().to_string());

            self.output_file.write("symbol".to_string(), ".".to_string());

            self.output_file.write("identifier".to_string(), do_content.get(do_content.find(".").unwrap()+1..do_content.find("(").unwrap()).unwrap().to_string());
        }else{
            self.output_file.write("identifier".to_string(), do_content.get(0..do_content.find("(").unwrap()).unwrap().to_string());
        }

        self.output_file.write("symbol".to_string(), "(".to_string());

        self.compile_expression_list(do_content.get(do_content.find("(").unwrap()+1..do_content.find(")").unwrap()).unwrap().to_string());

        self.output_file.write("symbol".to_string(), ")".to_string());

        self.output_file.write("symbol".to_string(), ";".to_string());

        self.output_file.close_tag("doStatement".to_string());
    }

    /// Compiles a return statement.
    fn compile_return(&mut self, content: String) {
        self.output_file.open_tag("returnStatement".to_string());


        self.output_file.close_tag("returnStatement".to_string());
    }

    /// Compiles an expression.
    fn compile_expression(&mut self, expression: String) {
        self.output_file.open_tag("expression".to_string());

        self.compile_term(expression);

        self.output_file.close_tag("expression".to_string());
    }

    /// Compiles a term.
    /// If the current token is an identifier, the routine must distinguish between a variable,
    /// an array-entry, or a subroutine-call.
    /// A single look-ahead token, which may be one of "[" , "(" or ".",
    /// suffices to distinguish between the possibilities
    /// Any other token is not part of this term and should not be advanced over.
    fn compile_term(&mut self, term: String) {
        self.output_file.open_tag("term".to_string());

        if term=="this"{
            self.output_file.write("keyword".to_string(), term.trim().to_string());
        }else {
            self.output_file.write("identifier".to_string(), term.to_string());
        }
        self.output_file.close_tag("term".to_string());
    }

    /// Compiles a (possibly empty) comma-seperated list of expressions.
    fn compile_expression_list(&mut self, content: String) {
        self.output_file.open_tag("expressionList".to_string());

        if !content.is_empty() {
            let commas = content.matches(",").count();
            let mut current = 0;
            let mut expressions = content.split(",");
            for expression in expressions {
                println!("{:?}",expression.trim());
                self.compile_expression(expression.to_string());
                expressions = content.split(",");
                if current<commas{
                    current +=1;

                    self.output_file.write("symbol".to_string(), ",".to_string());
                }
                expressions = content.split(",");
            }
        }
        self.output_file.close_tag("expressionList".to_string());
    }
}