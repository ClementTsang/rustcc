#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]


use std::env;
use std::fs;
use std::str;
use std::clone;
//use regex::Regex;

const RETURN_KEYWORD : &str = "return";
const SEMICOLON : &str  = ";";
const INT : &str  = "1234567890";

struct Token {
    name : String,
    value : String,
}


fn is_number (c : char) -> bool {
   let nums = "0123456789";
   nums.contains(c)
}

fn is_letter (c : char) -> bool {
    let letters = "abcdefghijklmnopqrstuvwxyz";
    letters.contains(c)
}

fn is_punctuation (c : char) -> bool {
    let punc = "{}();";
    punc.contains(c)
}

fn read_identifier (input : &mut String) -> Token {
    let mut iden_name = String::new();
    let tmp = input.clone();
    for c in tmp.chars() {
        if (!c.is_whitespace() && !is_punctuation(c)) {
            iden_name.push(c);
            input.remove(0);
        }
        else {
            break;
        }
    }

    Token {name : String::from("Identifier"), value : iden_name}
}

fn read_number (input : &mut String) -> Token {
    let mut iden_name = String::new();
    let tmp = input.clone();
    for c in tmp.chars() {
        if (!c.is_whitespace() && !is_punctuation(c) && !is_letter(c)) {
            iden_name.push(c);
            input.remove(0);
        }
    }

    Token {name : String::from("Num"), value : iden_name}
}

fn read_punc (input : &mut String) -> Token {
    let ret_punc = (*input).chars().next().unwrap().to_string();
    input.remove(0); 
    Token {name : String::from("Punc"), value : ret_punc} 
}



fn print_ast () {
}

fn print_assembly(){
    
}

fn parse_function(token_vec : Vec<Token>) {
}

fn lexer(input : &mut String) -> Vec<Token> {
    let mut token_vec : Vec<Token> = Vec::new();
    let mut c : char;

    while (input.len() > 0) {
        c = input.chars().next().unwrap();

        if (!c.is_whitespace()) {
            //println!("Character: {}", c);
            if (is_letter(c)) {
                // Must be identifier, as no quotes (not supported yet).
                token_vec.push(read_identifier(input));
            }
                else if (is_number(c)) {
                token_vec.push(read_number(input));
            }
            else if (is_punctuation(c)) {
                token_vec.push(read_punc(input));
            }
            else {
                println!("Found a character that the lexer does not recognize: {}.", c);       
                std::process::exit(1);
            }
        }
        else {
            input.remove(0);
        }
    }

    token_vec
}

fn parse_to_ast(filename : &String) {
    // Take in file.
    let mut file_contents = fs::read_to_string(filename).expect("Could not read file.");
    print!("=====Contents of file=====\n{}", file_contents);
    print!("=====End of file contents=====\n");
    let token_vec : Vec<Token> = lexer(&mut file_contents);
    

    parse_function(token_vec); 
    

    // Print out resulting AST (for debugging).
    print_ast();
}

fn convert_ast_to_assembly() -> String {
    let mut result = String::new();

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
