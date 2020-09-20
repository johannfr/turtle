#[macro_use]
extern crate lalrpop_util;

pub mod ast;

lalrpop_mod!(pub parser);

pub fn compile(input: &str) -> Result<ast::Program, String> {
    match parser::ProgramParser::new().parse(input) {
        Ok(s) => Ok(ast::Program::new(s)),
        Err(e) => Err(format!("{:?}", e)),
    }
}
