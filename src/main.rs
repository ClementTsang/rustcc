use std::env;
use std::fs;
use std::str;
use regex::Regex;

const RETURN_KEYWORD : &str = "return";
const SEMICOLON : &str  = ";";
const INT : &str  = "1234567890";

enum TokenType {
    KeyWord,
    Identifier,
    Punctuation,
    NumLiteral,
    StringLiteral,
}


struct Token {
    name : TokenType,
    value : String,
}


fn print_ast () {
}

fn print_assembly(){
    
}

fn parse_function() {
    
}

fn lexer(input : &String) -> Vec<Token> {
    let token_vec : Vec<Token> = Vec::new();

    for c in input.chars() {
        if (!c.is_whitespace()) {
            
        }
    }

    token_vec
}

fn parse_to_ast(filename : &String) {
    // Take in file.
    let file_contents = fs::read_to_string(filename).expect("Could not read file.");
    println!("=====Contents of file=====\n{}", file_contents);
    print!("=====End of file contents=====\n");
    let token_vec : Vec<Token> = lexer(&file_contents);

    parse_function(); 
    

    // Print out resulting AST (for debugging).
    print_ast();
}

fn convert_ast_to_assembly() -> String {
    let mut result = String::from("");

    // Print out resulting assembly (for debugging).
    print_assembly();

    result
}


fn main() {
    // Convert our .c file into an AST.
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    parse_to_ast(filename);

    // Convert AST into valid assembly.
    convert_ast_to_assembly();

}
