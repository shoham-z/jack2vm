use crate::xmlwriter::XmlWriter;

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
        CompilationEngine {
            output_file: XmlWriter::new(path),
            input_file: "".to_string(),
        }
    }

    /// Compiles a complete class.
    fn compile_class(){

    }

    /// Compiles a static variable declaration of field declaration.
    fn compile_class_var_dec(){

    }

    /// Compiles a complete method, function or constructor.
    fn compile_subroutine_dec(){

    }

    /// Compiles a (possibly empty) parameter list.
    /// Does not handle the enclosing "()".
    fn compile_parameter_list(){

    }

    /// Compiles a subroutine's body.
    fn compile_subroutine_body(){

    }

    /// Compiles a var declaration.
    fn compile_var_dec(){

    }

    /// Compiles a sequence of statements.
    /// Does not handle the enclosing "()".
    fn compile_statements(){

    }

    /// Compiles a let statement.
    fn compile_let(){

    }

    /// Compiles an if statement, possible with a trailing else clause.
    fn compile_if(){

    }

    /// Compiles a while statement.
    fn compile_while(){

    }

    /// Compiles a do statement.
    fn compile_do(){

    }

    /// Compiles a return statement.
    fn compile_return(){

    }

    /// Compiles an expression.
    fn compile_expression(){

    }

    /// Compiles a term.
    /// If the current token is an identifier, the routine must distinguish between a variable,
    /// an array-entry, or a subroutine-call.
    /// A single look-ahead token, which may be one of "[" , "(" or ".",
    /// suffices to distinguish between the possibilities
    /// Any other token is not part of this term and should not be advanced over.
    fn compile_term(){

    }

    /// Compiles a (possibly empty) comma-seperated list of expressions.
    fn compile_expression_list(){

    }

}