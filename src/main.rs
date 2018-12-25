#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

/**
 * To do:
 * Rewrite datatypes to use builder format for ease
 * Format resulting .s files better (Spacing is WHACK)
 **/

pub mod parser;
use std::env;
use std::fs;
use std::str;
use std::clone;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
//use regex::Regex;



fn print_assembly(input : &String){
   println!("=====Resulting assembly=====\n{}", input);
   println!("=====End assembly=====");
}

fn generate_function(func : &parser::Function) -> String {
    let mut result : String = String::from(func.name.clone().as_str());
    result.push_str(":\n");
    result.push_str(generate_statement(&func.st).as_str());
    result
}

fn generate_statement(st : &parser::Statement) -> String {
    let mut result = String::new();
    if (st.name == "return") {
        result = String::from("    movl");
        result.push_str(generate_expr(&st.exp).as_str());
        result.push_str("    ret");
    }
    result
}

fn generate_expr(exp : &parser::Expression) -> String {
    let mut result = String::new();
    
    if (exp.data_type.clone().as_str() != "Unary_Op") {
        result.push_str("    $");
        result.push_str(exp.val.to_string().as_str());
        result.push_str(", %eax\n");
    }
    else {
        match exp.inner_exp.clone() {
            None => {
                println!("Failed to process an expression - unary op missing next exp!");
                std::process::exit(1);
            },
            Some(expr) => {
                result.push_str(generate_expr(&(*expr)).as_str());
                match exp.op.as_str() {
                    "-" => {
                        result.push_str("    neg    %eax\n");
                    },
                    "~" => {
                        result.push_str("    not    %eax\n");
                    },
                    "!" => {
                        result.push_str(
                        "    cmpl    $0, %eax\n    movl    $0, %eax\n    sete   %al\n");
                    },
                    _ => {
                        println!("Found an unsupported unary expression!");
                        std::process::exit(1);
                    },
                }
            },
        }
    }

    
    result
}

fn generate_assembly(prog : &parser::Program, filename : String) -> String {
    let mut result = String::from(
    "    .globl    main\n    .type main, @function\n");

    result.push_str(generate_function(&prog.fnc).as_str());
    result.push_str("\n");

    // Print out resulting assembly (for debugging).
    print_assembly(&result);

    // Write to file:
    let mut assembly_file_name : String = String::from(&filename[..filename.len()-2]);
    assembly_file_name.push_str(".s");
    let mut file = File::create(assembly_file_name).expect("Failed to create file.");
    file.write_all(result.as_str().as_bytes()).expect("Failed to write data.");

    result
}


fn main() {
    // Convert our .c file into an AST.
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let parsed_prog : parser::Program = parser::parse_to_ast(filename);

    // Convert AST into valid assembly.
    generate_assembly(&parsed_prog, filename.clone());

}
