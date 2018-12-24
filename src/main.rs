#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]

/**
 * To do:
 * Reorganize this mess
 * Finish assembly
 * THEN move on
 **/

pub mod parser;
use std::env;
use std::fs;
use std::str;
use std::clone;
use std::fmt;
//use regex::Regex;



fn print_assembly(){
    
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
    parser::parse_to_ast(filename);

    // Convert AST into valid assembly.
    convert_ast_to_assembly();

}
