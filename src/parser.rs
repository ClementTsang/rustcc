#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]

pub mod lexer;

use std::env;
use std::fs;
use std::str;
use std::clone;
use std::fmt;

pub struct Program { 
    pub fnc : Function,
}

pub struct Function {
    pub name : String,
    pub return_type : String,
    pub st : Statement,
}

pub struct Statement {
    pub name : String,
    pub exp : Expression,
}

pub struct Expression {
    pub data_type : String,
    pub val : i32,
}

pub fn print_ast (input_prog : &Program) {
    println!("=====AST PRINT=====");

    println!("FUN {} {}:", input_prog.fnc.return_type, input_prog.fnc.name);
    println!("     params: (TO BE ADDED)");
    println!("     body:");
    println!("          {} {} <{}>", input_prog.fnc.st.name, input_prog.fnc.st.exp.data_type, input_prog.fnc.st.exp.val);
        
    println!("=====END AST PRINT=====");
}

pub fn parse_program(token_vec : &mut Vec<lexer::Token>) -> Program {
    let mut result : Program = Program { fnc : Function { 
        name : String::from(""), 
        return_type : String::from(""), 
        st : Statement { 
            name : String::from(""),
            exp : Expression { 
                data_type: String::from(""), val : 0 
            }
        }
    }};
    
    // A program, by our grammar rules, MUST contain a function from the tokens passed in.
    // We thus look for main().
    
    result.fnc = parse_function(token_vec);


    result
}

pub fn get_option_token( op : Option<lexer::Token> ) -> lexer::Token {
    match op {
        None => { 
            println!("Failed to get token from option");
            std::process::exit(1);
        },
        Some(x) => { x },
    }
}

pub fn get_next_token(token_vec : &mut Vec<lexer::Token>) -> lexer::Token {
    let tok : lexer::Token = get_option_token(token_vec.first().cloned());
    token_vec.remove(0);
    //println!("Token obtained: {}", tok);
    tok
}

pub fn parse_function(token_vec : &mut Vec<lexer::Token>) -> Function {
    let mut result : Function  = Function {
        name : String::from(""), 
        return_type : String::from(""),
        st : Statement { 
            name: String::from(""), 
            exp : Expression { 
                data_type: String::from(""), val : 0 }
        }
    };

    let mut tok : lexer::Token = get_next_token(token_vec);
    assert!(tok.name == "Keyword", "Invalid keyword");
    result.return_type = tok.value;
    
    tok = get_next_token(token_vec);
    assert!(tok.name == "Identifier", "Invalid identifier");
    result.name = tok.value;

    tok = get_next_token(token_vec);
    assert!(tok.name == "Punc" && tok.value == "(", "Invalid punc. (\"(\")");
    // Param names go in between here!
    tok = get_next_token(token_vec);
    assert!(tok.name == "Punc" && tok.value == ")", "Invalid punc. (\")\")");

    tok = get_next_token(token_vec);
    assert!(tok.name == "Punc" && tok.value == "{", "Invalid punc. (\"{\")");

    // Statement check
    result.st = parse_statement(token_vec);

    tok = get_next_token(token_vec);
    assert!(tok.name == "Punc" && tok.value == "}", "Invalid punc. (\"}\")");


    result
}

pub fn parse_statement(token_vec : &mut Vec<lexer::Token>) -> Statement {
    let mut result : Statement = Statement {
        name: String::from(""), 
        exp : Expression { 
            data_type: String::from(""), val : 0 
        }
    }; 
    
    let mut tok : lexer::Token = get_next_token(token_vec);
    assert!(tok.name == "Keyword" && tok.value == "return", "Invalid keyword");
    result.name = tok.value;

    //Expression check
    result.exp = parse_expression(token_vec);

    tok = get_next_token(token_vec);
    assert!(tok.value == ";", "Missing semicolon");
    
    result
}

pub fn parse_expression(token_vec : &mut Vec<lexer::Token>) -> Expression {
    let mut result : Expression = Expression {
        data_type: String::from(""), val:0
    }; 
    
    let tok : lexer::Token = get_next_token(token_vec);
    assert!(tok.name == "Num", "Not a value.");
    result.val = tok.value.parse::<i32>().unwrap();

    if (tok.name == "Num") {
        result.data_type = String::from("Int");
    }

    result
}

pub fn parse_to_ast(filename : &String) -> Program {
    // Take in file.
    let mut file_contents = fs::read_to_string(filename).expect("Could not read file.");
    print!("=====Contents of file=====\n{}", file_contents);
    print!("=====End of file contents=====\n");

    // Convert to tokens
    let mut token_vec : Vec<lexer::Token> = lexer::lexer(&mut file_contents);
    println!("=====Resulting tokens=====");
    for token in &token_vec {
        println!("Token: {}", token);
    }
    println!("=====End of tokens=====");

    // Parse tokens into AST
    let result_ast : Program = parse_program(&mut token_vec); 
    print_ast(&result_ast);

    result_ast
}

