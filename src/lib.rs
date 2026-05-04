use std::{fs, io};

mod ast;
mod converter;
mod lexer;
mod parser;

pub fn to_html(input: &str) -> String {
    let mut lexer = lexer::Lexer::new(input.into());
    let tokens = lexer.lex();

    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();

    converter::to_html(ast)
}

pub fn to_html_file(input: &str, path: &str) -> io::Result<()> {
    let out = to_html(input);
    fs::write(path, out)
}
