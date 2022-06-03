use std::env::var;
use std::fs;

use regex::Regex;
use crate::symbol_table::SymbolTable;

use crate::xmlwriter::XmlWriter;
use crate::utility::{CLASS_FUNC_TYPES, CLASS_VAR_TYPES, DATA_TYPES, KEYWORD_CONSTANT, Kind, OP, UNARY_OP};
use crate::vm_writer::VMWriter;

pub struct CompilationEngine {
    class_name: String,
    xml_file: XmlWriter,
    vm_file: VMWriter,
    input_file: String,
    class_symbol_table : SymbolTable,
    subroutine_symbol_table : SymbolTable
}

impl CompilationEngine {
    /// Compiles the entire directory
    pub fn compile(&mut self) {
        self.compile_class();
    }

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
        let remove_comments: Regex = Regex::new(r#"//.*\n|/\*.*\*/"#).unwrap();
        let file_contents = fs::read_to_string(path).unwrap().as_str().to_owned();

        let code = remove_comments.replace_all(&file_contents, "\n").to_string();

        CompilationEngine {
            class_name: code.split_whitespace().nth(1).unwrap().to_string(),
            xml_file: XmlWriter::new(&(path.to_owned().split(".jack").collect::<Vec<_>>()[0].to_owned() + "My.jack")),
            vm_file: VMWriter::new(&(path.to_owned().split(".jack").collect::<Vec<_>>()[0].to_owned() + "My.jack")),
            input_file: code,
            class_symbol_table: SymbolTable::new(),
            subroutine_symbol_table: SymbolTable::new()
        }
    }

    /// Compiles a complete class.
    fn compile_class(&mut self) {
        //self.output_file.open_tag("class".to_string());
        let code = self.input_file.to_string();

        let lines = code.lines();
        let count = lines.count();
        let mut lines = code.lines();

        self.class_symbol_table  = SymbolTable::new();

        for _index in 1..count {
            let line = lines.nth(0).unwrap();
            let trimmed_line = line.trim_start();
            let first_word = trimmed_line.split(" ").nth(0).unwrap();
            if first_word == "class" {

                //self.output_file.write("keyword".to_string(), first_word.to_string());//class keyword
                //self.output_file.write("identifier".to_string(), trimmed_line.split(" ").nth(1).unwrap().to_string());//class name
                //self.output_file.write("symbol".to_string(), "{".to_string());//opening bracket
                break;
            }
        }
        let mut class_contents = code.get(code.find("{").unwrap()..code.rfind("}").unwrap() + 1).unwrap().lines();
        //to get all the content of the class, such as fields, statics, constructors, methods and functions

        for _index in 1..class_contents.clone().count() {
            let line = class_contents.nth(0).unwrap();
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
                // This is the source of the problem
                // collect the lines one by one the same way as in while and if statement
                let mut subroutine_code = Vec::new();
                let new_code_source = code.get(code.find(line).unwrap()..code.len()).unwrap().lines();
                let mut open_count = 0;
                let mut close_count = 0;
                for new_line in new_code_source {
                    open_count += new_line.matches("{").count();
                    close_count += new_line.matches("}").count();
                    subroutine_code.push(new_line);

                    if open_count == close_count && open_count != 0 {
                        break;
                    } else if open_count < close_count {
                        let tmp = "{";
                        panic!("ERROR IN JACK CODE: Missing \'{}\'", tmp)
                    }
                }
                self.compile_subroutine_dec(subroutine_code.join("\n"));
            }
        }


        //self.output_file.write("symbol".to_string(), "}".to_string());//closing bracket
        //self.output_file.close_tag("class".to_string());
    }

    /// Compiles a static variable declaration or field declaration.
    fn compile_class_var_dec(&mut self, line: String) {
        let mut words = line.trim().get(0..line.trim().len()-1).unwrap().split_whitespace();

        //self.output_file.open_tag("classVarDec".to_string());
        //self.output_file.write("keyword".to_string(), words.nth(0).unwrap().to_string());

        let kind =  match words.nth(0).unwrap() {
            "field" => Kind::FIELD,
            "static" => Kind::STATIC,
            &_ => {Kind::NONE}
        };
        let data_type = words.nth(0).unwrap();
        /*if DATA_TYPES.contains(&data_type) {
            self.output_file.write("keyword".to_string(), data_type.to_string());
        } else {
            self.output_file.write("identifier".to_string(), data_type.to_string());
        }*/

        let mut comma = line.find(",");

        if let Some(_value) = comma {

            // TO DO: handle more than one var name
            // EXAMPLE: "field int i, sum;"
            let mut var_name = words.next().unwrap();

            while let Some(_value) = comma {
                //self.output_file.write("identifier".to_string(), var_name.get(0..var_name.find(',').unwrap()).unwrap().to_string());
                //self.output_file.write("symbol".to_string(), ",".to_string());
                if var_name.find(",") == Some(0) {
                    var_name = var_name.get(1..var_name.len()).unwrap();
                }
                else if var_name.find(",") == Some(var_name.len()-1) {
                    var_name = var_name.get(0..var_name.len()-1).unwrap();
                }

                self.class_symbol_table.define(var_name.to_string(),data_type.to_string(), kind);

                var_name = words.next().unwrap();

                comma = var_name.find(",");
            }
            self.class_symbol_table.define(var_name.to_string(),data_type.to_string(), kind);
            //self.output_file.write("identifier".to_string(), var_name.get(0..var_name.find(';').unwrap()).unwrap().to_string());
            //self.output_file.write("symbol".to_string(), ";".to_string());
        } else {
            let var_name = words.nth(0).unwrap();
            self.class_symbol_table.define(var_name.get(0..var_name.len()).unwrap().to_string(),data_type.to_string(), kind);

            //self.output_file.write("identifier".to_string(), var_name.get(0..var_name.len() - 1).unwrap().to_string());
            //self.output_file.write("symbol".to_string(), ";".to_string());
        }

        let mut next: &str;

        let attempt = line.split(",");
        if attempt.size_hint().0 > 0 {
            loop {
                next = words.next().unwrap();
                if next == ";" {
                    //self.output_file.write("symbol".to_string(), next.to_string());
                    break;
                } else if next == "," {
                    //self.output_file.write("symbol".to_string(), next.to_string());
                } else {
                    self.class_symbol_table.define(words.next().unwrap().to_string(),data_type.to_string(), kind);

                    //self.output_file.write("identifier".to_string(), words.next().unwrap().to_string());
                }
            }
        }

        //self.output_file.close_tag("classVarDec".to_string());
    }

    /// Compiles a complete method, function or constructor.
    pub fn compile_subroutine_dec(&mut self, content: String) {
        //self.xml_file.open_tag("subroutineDec".to_string());

        let func_dec = content.get(0..content.find("{").unwrap()).unwrap();

        let mut words = func_dec.split_whitespace();

        //self.xml_file.write("keyword".to_string(), words.nth(0).unwrap().to_string());//subroutine type - constructor/method/function keyword

        let data_type = words.nth(0).unwrap();//subroutine return type
        if DATA_TYPES.contains(&data_type) {
            //self.xml_file.write("keyword".to_string(), data_type.to_string());
        } else {
            //self.xml_file.write("identifier".to_string(), data_type.to_string());
        }

        //self.xml_file.write("identifier".to_string(), words.nth(0).unwrap().split("(").nth(0).unwrap().to_string());//subroutine name

        //self.xml_file.write("symbol".to_string(), "(".to_string());

        let param_list = func_dec.get(content.find("(").unwrap() + 1..content.find(")").unwrap()).unwrap();

        self.subroutine_symbol_table.define("this".to_string(), self.class_name.to_string(), Kind::ARG);

        self.compile_parameter_list(param_list.to_string());

        //self.xml_file.write("symbol".to_string(), ")".to_string());

        let func_body = content.get(content.find("{").unwrap()..content.rfind("}").unwrap() + 1).unwrap();

        self.compile_subroutine_body(func_body.to_string());

        //self.xml_file.close_tag("subroutineDec".to_string());
    }

    /// Compiles a (possibly empty) parameter list.
    /// Does not handle the enclosing "()".
    fn compile_parameter_list(&mut self, content: String) {
        self.xml_file.open_tag("parameterList".to_string());
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
                                //self.output_file.write("symbol".to_string(), ",".to_string());
                            } else { first_time = false; }

                            var_split = value.split_whitespace();
                        }
                    }
                    let data_type = var_split.next().unwrap().to_string();
                    let var_name = var_split.next().unwrap().to_string();

                    self.subroutine_symbol_table.define(var_name, data_type, Kind::ARG);

                    //self.output_file.write("keyword".to_string(), var_split.next().unwrap().to_string());
                    //self.output_file.write("identifier".to_string(), var_split.next().unwrap().to_string());
                }
            }
            let temp = vars.next();

            match temp {
                None => {}
                Some(value) => {
                    var_split = value.split_whitespace();
                    let data_type = var_split.next().unwrap().to_string();
                    let var_name = var_split.next().unwrap().to_string();

                    self.subroutine_symbol_table.define(var_name, data_type, Kind::ARG);

                    //self.output_file.write("keyword".to_string(), var_split.next().unwrap().to_string());
                    //self.output_file.write("identifier".to_string(), var_split.next().unwrap().to_string());
                }
            }
        }
        //self.output_file.close_tag("parameterList".to_string());
    }

    /// Compiles a subroutine's body.
    fn compile_subroutine_body(&mut self, content: String) {
        self.xml_file.open_tag("subroutineBody".to_string());

        self.xml_file.write("symbol".to_string(), "{".to_string());

        let body_content = content.get(content.find("{").unwrap() + 1..content.rfind("}").unwrap()).unwrap();
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

        self.xml_file.write("symbol".to_string(), "}".to_string());

        self.xml_file.close_tag("subroutineBody".to_string());
    }

    /// Compiles a var declaration.
    fn compile_var_dec(&mut self, content: String) {
        //self.output_file.open_tag("varDec".to_string());
        //self.output_file.write("keyword".to_string(), "var".to_string());

        let mut words = content.split_whitespace();

        let data_type = words.nth(1).unwrap();
        if DATA_TYPES.contains(&data_type) {
            //self.output_file.write("keyword".to_string(), data_type.to_string());
        } else {
            //self.output_file.write("identifier".to_string(), data_type.to_string());
        }

        let mut comma = content.find(",");

        if let Some(_value) = comma {

            // TO DO: handle more than one var name
            // EXAMPLE: "var int i, sum;"
            let mut var_name = words.next().unwrap();

            while let Some(_value) = comma {
                //self.output_file.write("identifier".to_string(), var_name.get(0..var_name.find(',').unwrap()).unwrap().to_string());
                //self.output_file.write("symbol".to_string(), ",".to_string());

                self.subroutine_symbol_table.define(var_name.get(0..var_name.find(',').unwrap()).unwrap().to_string(), data_type.to_string(), Kind::VAR);
                var_name = words.next().unwrap();

                comma = var_name.find(",");
            }
            self.subroutine_symbol_table.define(var_name.get(0..var_name.find(',').unwrap()).unwrap().to_string(), data_type.to_string(), Kind::VAR);

            //self.output_file.write("identifier".to_string(), var_name.get(0..var_name.find(';').unwrap()).unwrap().to_string());
            //self.output_file.write("symbol".to_string(), ";".to_string());
        } else {
            let var_name = words.nth(0).unwrap();
            self.subroutine_symbol_table.define(var_name.get(0..var_name.find(',').unwrap()).unwrap().to_string(), data_type.to_string(), Kind::VAR);

            //self.output_file.write("identifier".to_string(), var_name.get(0..var_name.len() - 1).unwrap().to_string());

            //self.output_file.write("symbol".to_string(), ";".to_string());
        }

        //self.output_file.close_tag("varDec".to_string());
    }

    /// Compiles a sequence of statements.
    /// Does not handle the enclosing "()".
    fn compile_statements(&mut self, content: String) {
        self.xml_file.open_tag("statements".to_string());

        let mut previous_statements = Vec::new();
        let temp = content.clone();
        let mut lines = temp.lines();
        let mut first_word = "";
        let mut tmp;
        let mut advance = true;
        let mut current_line = "";
        if lines.clone().count() == 1 {
            let line = content.to_string();

            tmp = line.trim().split_whitespace().nth(0);
            match tmp {
                None => {}
                Some(value) => { first_word = value; }
            }
            if first_word == "let"
            {
                self.compile_let(line.trim().to_string());
            } else if first_word == "do"
            {
                self.compile_do(line.trim().to_string());
            } else if line.contains("return")
            {
                self.compile_return(line.trim().to_string());
            }
        } else {
            for _index in 1..lines.clone().count() {
                if advance {
                    let tmp = lines.next();
                    if tmp.is_some() {
                        current_line = tmp.unwrap().trim();
                    }
                }
                advance = true;
                tmp = current_line.trim().split_whitespace().nth(0);
                match tmp {
                    None => {}
                    Some(value) => { first_word = value; }
                }
                if first_word == "while"
                {
                    let mut start_statement = "";
                    let mut lines_clone = temp.lines();
                    for index_clone in 1..lines_clone.clone().count() {
                        let line_clone = lines_clone.next().unwrap();
                        if line_clone.contains("while") && !previous_statements.contains(&index_clone) {
                            start_statement = line_clone;
                            previous_statements.push(index_clone);
                            break;
                        }
                    }

                    let mut open_count = 0;
                    let mut close_count = 0;

                    let mut while_lines = Vec::new();
                    while_lines.push(start_statement);
                    for line in lines.clone() {
                        while_lines.push(line);
                    }

                    let mut while_statement = "".to_string();
                    for while_line in while_lines {
                        if !while_line.is_empty() {
                            current_line = lines.next().unwrap();
                            while_statement.push_str(while_line);
                            while_statement.push_str("\n");
                            open_count += while_line.matches("{").count();
                            close_count += while_line.matches("}").count();

                            if open_count == close_count && open_count != 0 {
                                advance = false;
                                break;
                            } else if open_count < close_count {
                                panic!("ERROR IN THE JACK CODE!")
                            }
                        }
                    }
                    if while_statement.trim().chars().last() != Some('}') {
                        while_statement.push_str("}");
                    }
                    self.compile_while(while_statement);
                }
                if first_word == "if"
                {
                    let mut start_statement = current_line;
                    let mut lines_clone = temp.lines();
                    for index_clone in 1..lines_clone.clone().count() {
                        let line_clone = lines_clone.next().unwrap();
                        if line_clone.contains("if") && !previous_statements.contains(&index_clone) {
                            start_statement = line_clone;
                            previous_statements.push(index_clone);
                            break;
                        }
                    }

                    let mut if_lines = Vec::new();
                    if_lines.push(start_statement);
                    for line in lines.clone() {
                        if_lines.push(line);
                    }

                    let mut if_statement = "".to_string();

                    let mut open_count = 0;
                    let mut close_count = 0;
                    let mut if_line = if_lines.get(0).unwrap();
                    for index in 1..if_lines.clone().len() - 1 {
                        if !if_line.is_empty() {
                            current_line = lines.next().unwrap();
                            if_statement.push_str(if_line);
                            if_statement.push_str("\n");
                            open_count += if_line.matches("{").count();
                            close_count += if_line.matches("}").count();

                            if_line = if_lines.get(index).unwrap();

                            if open_count == close_count && open_count != 0 && !if_line.contains("else") {
                                advance = false;
                                break;
                            } else if open_count < close_count {
                                panic!("ERROR IN THE JACK CODE!")
                            }
                        }
                    }

                    self.compile_if(if_statement);
                }
                tmp = current_line.trim().split_whitespace().nth(0);
                match tmp {
                    None => {}
                    Some(value) => { first_word = value; }
                }
                if first_word == "let"
                {
                    self.compile_let(current_line.trim().to_string());
                    advance = true;
                }
                if first_word == "do"
                {
                    self.compile_do(current_line.trim().to_string());
                    advance = true;
                }

                if current_line.contains("return")
                {
                    self.compile_return(current_line.trim().to_string());
                    advance = true;
                }
                first_word = "";
            }
        }

        self.xml_file.close_tag("statements".to_string());
    }

    /// Compiles a let statement.
    fn compile_let(&mut self, content: String) {
        self.xml_file.open_tag("letStatement".to_string());

        self.xml_file.write("keyword".to_string(), "let".to_string());

        let assign_to = content.get(content.find(" ").unwrap()..content.find("=").unwrap()).unwrap();

        if assign_to.contains("[") {
            self.xml_file.write("identifier".to_string(), assign_to.split("[").nth(0).unwrap().trim().to_string());


            self.xml_file.write("symbol".to_string(), "[".to_string());

            self.compile_expression(content.get(content.find("[").unwrap() + 1..content.find("]").unwrap()).unwrap().trim().to_string());

            self.xml_file.write("symbol".to_string(), "]".to_string());
        } else {
            self.xml_file.write("identifier".to_string(), assign_to.trim().to_string());
        }
        self.xml_file.write("symbol".to_string(), "=".to_string());

        self.compile_expression(content.get(content.find("=").unwrap() + 1..content.find(";").unwrap()).unwrap().trim().to_string());

        self.xml_file.write("symbol".to_string(), ";".to_string());


        self.xml_file.close_tag("letStatement".to_string());
    }

    /// Compiles an if statement, possible with a trailing else clause.
    fn compile_if(&mut self, content: String) {
        self.xml_file.open_tag("ifStatement".to_string());

        let temp = content.get(content.find("(").unwrap() + 1..content.find("{").unwrap()).unwrap();



        let expression = temp.get(0..temp.rfind(")").unwrap()).unwrap();


        if let Some(value) = content.find("else") {
            self.xml_file.write("keyword".to_string(), "if".to_string());

            self.xml_file.write("symbol".to_string(), "(".to_string());

            self.compile_expression(expression.to_string());

            self.xml_file.write("symbol".to_string(), ")".to_string());

            self.xml_file.write("symbol".to_string(), "{".to_string());

            self.compile_statements(content.get(content.find("{").unwrap() + 1..value - 1).unwrap().to_string());

            self.xml_file.write("symbol".to_string(), "}".to_string());

            self.xml_file.write("keyword".to_string(), "else".to_string());

            self.xml_file.write("symbol".to_string(), "{".to_string());

            self.compile_statements(content.get(value + 1..content.rfind("}").unwrap()).unwrap().to_string());

            self.xml_file.write("symbol".to_string(), "}".to_string());
        } else {
            self.xml_file.write("keyword".to_string(), "if".to_string());

            self.xml_file.write("symbol".to_string(), "(".to_string());

            self.compile_expression(expression.to_string());

            self.xml_file.write("symbol".to_string(), ")".to_string());

            self.xml_file.write("symbol".to_string(), "{".to_string());

            self.compile_statements(content.get(content.find("{").unwrap() + 1..content.rfind("}").unwrap()).unwrap().to_string());

            self.xml_file.write("symbol".to_string(), "}".to_string());
        }

        self.xml_file.close_tag("ifStatement".to_string());
    }

    /// Compiles a while statement.
    fn compile_while(&mut self, content: String) {
        self.xml_file.open_tag("whileStatement".to_string());

        let temp = content.get(content.find("(").unwrap() + 1..content.find("{").unwrap()).unwrap();

        let expression = temp.get(0..temp.rfind(")").unwrap()).unwrap();

        self.xml_file.write("keyword".to_string(), "while".to_string());

        self.xml_file.write("symbol".to_string(), "(".to_string());

        self.compile_expression(expression.to_string());

        self.xml_file.write("symbol".to_string(), ")".to_string());

        self.xml_file.write("symbol".to_string(), "{".to_string());

        self.compile_statements(content.get(content.find("{").unwrap() + 1..content.rfind("}").unwrap()).unwrap().to_string());

        self.xml_file.write("symbol".to_string(), "}".to_string());

        self.xml_file.close_tag("whileStatement".to_string());
    }

    /// Compiles a do statement.
    fn compile_do(&mut self, content: String) {
        self.xml_file.open_tag("doStatement".to_string());

        self.xml_file.write("keyword".to_string(), "do".to_string());

        let do_content = content.get(content.trim().find(" ").unwrap() + 1..content.trim().len() - 1).unwrap();

        let dot = do_content.find('.');

        if let Some(_value) = dot {
            self.xml_file.write("identifier".to_string(), do_content.get(0..do_content.find(".").unwrap()).unwrap().to_string());

            self.xml_file.write("symbol".to_string(), ".".to_string());

            self.xml_file.write("identifier".to_string(), do_content.get(do_content.find(".").unwrap() + 1..do_content.find("(").unwrap()).unwrap().to_string());
        } else {
            self.xml_file.write("identifier".to_string(), do_content.get(0..do_content.find("(").unwrap()).unwrap().to_string());
        }

        self.xml_file.write("symbol".to_string(), "(".to_string());

        self.compile_expression_list(do_content.get(do_content.find("(").unwrap() + 1..do_content.rfind(")").unwrap()).unwrap().to_string());

        self.xml_file.write("symbol".to_string(), ")".to_string());

        self.xml_file.write("symbol".to_string(), ";".to_string());

        self.xml_file.close_tag("doStatement".to_string());
    }

    /// Compiles a return statement.
    fn compile_return(&mut self, content: String) {
        self.xml_file.open_tag("returnStatement".to_string());

        self.xml_file.write("keyword".to_string(), "return".to_string());

        if !content.contains("return;") {
            self.compile_expression(content.get(content.trim().find(" ").unwrap()..content.find(";").unwrap()).unwrap().trim().to_string());
        }

        self.xml_file.write("symbol".to_string(), ";".to_string());

        self.xml_file.close_tag("returnStatement".to_string());
    }

    /// Compiles an expression.
    pub fn compile_expression(&mut self, expression: String) {

        self.xml_file.open_tag("expression".to_string());

        let mut index = usize::MAX;
        let mut tmp;
        let mut arr: Vec<usize> = Vec::new();
        for op in OP {
            tmp = expression.trim().find(op);
            match tmp {
                None => {}
                Some(value) => {
                    if value != usize::MAX {
                        arr.push(value);
                    }
                }
            }
        }
        index = usize::MAX;

        for val in arr {
            let start = expression.get(0..val).unwrap().trim();
            let end = expression.get(val + 1..expression.len()).unwrap().trim();

            let start_open = start.find("(");
            let start_end = start.rfind(")");
            let end_open = end.find("(");
            let end_end = end.rfind(")");
            if (start_open == Some(0) && start_end == Some(start.len() - 1)) || (end_open == Some(0) && end_end == Some(end.len() - 1)) {
                index = if start_end.is_some() {start_end.unwrap()+ 1} else if  end_open.is_some() {val} else {0} ;
                let mut s = expression.get(val..index + 1);
                while s == Some(" ") && s.is_some() {
                    index = index + 1;
                    s = expression.get(index..index + 1);
                }
                break;
            }
            if val < index {
                index = val;
            }
        }
        let reg = Regex::new(r"(?m)\(.*\) ([+\-*/&|<>=]) \(.*\)|\(.*\)([+\-*/&|<>=])\(.*\)").unwrap();
        if reg.is_match(expression.trim()) {
            for op in OP {
                let checks = expression.split(op);

                if checks.clone().count() > 1 {
                    for item in checks.clone() {
                        let exp = item.trim();
                        if exp.find("(") == Some(0) && exp.rfind(")") == Some(exp.len() - 1) {
                            self.compile_term(item.trim().to_string());
                            if item != checks.clone().last().unwrap() {
                                match op {
                                    "<" => self.xml_file.write("symbol".to_string(), "&lt;".to_string()),
                                    ">" => self.xml_file.write("symbol".to_string(), "&gt;".to_string()),
                                    "&" => self.xml_file.write("symbol".to_string(), "&amp;".to_string()),
                                    &_ => self.xml_file.write("symbol".to_string(), op.trim().to_string())
                                }
                            } else {
                                self.xml_file.close_tag("expression".to_string());
                                return;
                            }
                        }
                    }
                }
            }
        } else if index == usize::MAX {
            self.compile_term(expression.trim().to_string());
        } else if index == 0 {
            self.compile_term(expression.trim().to_string());
        } else if expression.get(0..1).unwrap() == "~" || expression.get(0..1).unwrap() == "-" {
            self.compile_term(expression.trim().to_string());
        } else {
            if expression.trim().get(0..index).unwrap().find("(") == Some(0) && expression.trim().get(0..index).unwrap().find(")") == Some(expression.len() - 1) {
                self.compile_term(expression.get(expression.find("(").unwrap()..expression.rfind(")").unwrap() + 1).unwrap().trim().to_string());
            } else {
                self.compile_term(expression.get(0..index).unwrap().trim().to_string());

                let mut symbol = expression.get(index..index + 1).unwrap().trim();
                if symbol == "" {
                    symbol = expression.get(index + 1..index + 2).unwrap().trim();
                    if symbol == "" {
                        symbol = expression.get(index + 2..index + 3).unwrap().trim();
                    }
                }

                match symbol {
                    "<" => self.xml_file.write("symbol".to_string(), "&lt;".to_string()),
                    ">" => self.xml_file.write("symbol".to_string(), "&gt;".to_string()),
                    "&" => self.xml_file.write("symbol".to_string(), "&amp;".to_string()),
                    &_ => self.xml_file.write("symbol".to_string(), symbol.trim().to_string())
                }
                let rest_of_exp = expression.get(index + 2..expression.len()).unwrap().trim();
                let mut rest_of_exp_has_op = false;
                for op in OP {
                    tmp = rest_of_exp.trim().find(op);
                    match tmp {
                        None => {}
                        Some(_) => {
                            rest_of_exp_has_op = true;
                            break;
                        }
                    }
                }

                if rest_of_exp_has_op {
                    self.compile_term(rest_of_exp.to_string());
                } else {
                    self.compile_term(rest_of_exp.trim().to_string());
                }
            }
        }
        self.xml_file.close_tag("expression".to_string());
    }

    /// Compiles a term.
    /// If the current token is an identifier, the routine must distinguish between a variable,
    /// an array-entry, or a subroutine-call.
    /// A single look-ahead token, which may be one of "[" , "(" or ".",
    /// suffices to distinguish between the possibilities
    /// Any other token is not part of this term and should not be advanced over.
    fn compile_term(&mut self, term: String) {
        self.xml_file.open_tag("term".to_string());
        for keyword in KEYWORD_CONSTANT {
            if term == keyword.to_string() {
                self.xml_file.write("keyword".to_string(), term.to_string());
                self.xml_file.close_tag("term".to_string());
                return;
            }
        }
        let mut unary_op = false;
        for op in UNARY_OP {
            if term.find(op) == Some(0) { unary_op = true; }
        }
        if unary_op {
            self.xml_file.write("symbol".to_string(), term.get(0..1).unwrap().to_string());

            self.compile_term(term.get(1..term.len()).unwrap().to_string())
        } else if term.find("(") == Some(0) && term.rfind(")") == Some(term.len() - 1) {
            self.xml_file.write("symbol".to_string(), "(".to_string());
            if term.find("-") == Some(1) {
                self.compile_expression(term.get(1..term.len() - 1).unwrap().to_string());
            } else if term.find("~") == Some(1) {
                self.compile_expression(term.get(1..term.len() - 1).unwrap().to_string());
            } else {
                self.compile_expression(term.get(1..term.len() - 1).unwrap().to_string());
            }
            self.xml_file.write("symbol".to_string(), ")".to_string());
        } else if term.find("\"") == Some(0) && term.rfind("\"") == Some(term.len() - 1) {
            self.xml_file.write("stringConstant".to_string(), term.get(term.find("\"").unwrap() + 1..term.rfind("\"").unwrap()).unwrap().to_string());
        } else if term.chars().all(char::is_numeric) { // check for integer constant

            self.xml_file.write("integerConstant".to_string(), term.to_string());
        } else if UNARY_OP.contains(&term.chars().next().unwrap().to_string().as_str()) {} else if term.find(".").is_some() {
            self.xml_file.write("identifier".to_string(), term.get(0..term.find(".").unwrap()).unwrap().to_string());

            self.xml_file.write("symbol".to_string(), ".".to_string());

            self.xml_file.write("identifier".to_string(), term.get(term.find(".").unwrap() + 1..term.find("(").unwrap()).unwrap().to_string());

            self.xml_file.write("symbol".to_string(), "(".to_string());
            self.compile_expression_list(term.get(term.find("(").unwrap() + 1..term.find(")").unwrap()).unwrap().to_string());

            self.xml_file.write("symbol".to_string(), ")".to_string());
        } else if term.find("[").is_some() {
            self.xml_file.write("identifier".to_string(), term.get(0..term.find("[").unwrap()).unwrap().to_string());

            self.xml_file.write("symbol".to_string(), "[".to_string());

            self.compile_expression(term.get(term.find("[").unwrap() + 1..term.find("]").unwrap()).unwrap().to_string());

            self.xml_file.write("symbol".to_string(), "]".to_string());
        } else {
            self.xml_file.write("identifier".to_string(), term.to_string());
        }
        self.xml_file.close_tag("term".to_string());
    }

    /// Compiles a (possibly empty) comma-seperated list of expressions.
    fn compile_expression_list(&mut self, content: String) {
        self.xml_file.open_tag("expressionList".to_string());

        if !content.is_empty() {
            let commas = content.matches(",").count();
            let mut current = 0;
            let expressions = content.split(",");
            for expression in expressions {
                self.compile_expression(expression.trim().to_string());
                if current < commas {
                    current += 1;

                    self.xml_file.write("symbol".to_string(), ",".to_string());
                }
            }
        }
        self.xml_file.close_tag("expressionList".to_string());
    }
}