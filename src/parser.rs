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
    pub params : Vec<String>,
}

pub struct Statement {
    pub name : String,
    pub exp : Expression,
}

pub struct Expression {
    pub data_type : String,
    pub val : i32,
    pub op : String,
    pub inner_exp : Option<Box<Expression>>,
}

impl Program { 
    pub fn new () -> Program {
        Program {
            fnc : Function::new(),
        }
    }
}

impl Function {
    pub fn new () -> Function {
        Function {
            name : String::new(),
            return_type : String::new(),
            st : Statement::new(),
            params : Vec::new(),
        }
    }
}

impl Statement {
    pub fn new () -> Statement {
        Statement {
            name : String::new(),
            exp : Expression::new(),
        }
    }
}

impl Expression {
    pub fn new() -> Expression {
        Expression {
            data_type: String::new(),
            val : 0,
            op : String::new(),
            inner_exp : None,
        }
    }
}

impl Clone for Expression {
    fn clone(&self) -> Self {
        Expression { 
            data_type : self.data_type.clone(),
            val : self.val,
            op : self.op.clone(),
            inner_exp : self.inner_exp.clone(),
        }
    }
}

pub fn print_ast (input_prog : &Program) {
    println!("=====AST PRINT=====");

    println!("FUN {} {}:", input_prog.fnc.return_type, input_prog.fnc.name);
    print!("     params: ( ");
    for p in &input_prog.fnc.params {
        print!("{} ", p);
    }
    println!(")");
    println!("     body:");
    
    print!("          {} ", input_prog.fnc.st.name);
    print_exp(&input_prog.fnc.st.exp);
    
    //println!("          {} {} <{}>", input_prog.fnc.st.name, input_prog.fnc.st.exp.data_type, input_prog.fnc.st.exp.val);
        
    println!("=====END AST PRINT=====");
}

pub fn print_exp (exp : &Expression) {
    match exp.data_type.as_str() {
        "Unary_Op" => {
            print!("{}", exp.op);
            match exp.inner_exp.clone() {
                None => println!("{}", exp.val.clone()),
                Some(x) => print_exp(&(*x)),
            }
        },
        _ => println!("{}", exp.val.clone()),
    }

}

pub fn parse_program(token_vec : &mut Vec<lexer::Token>) -> Program {
    let mut result : Program = Program::new(); 
   
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
    let mut result : Function  = Function::new();

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
    let mut result : Statement = Statement::new();

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
    let mut result : Expression = Expression::new();
    
    let tok : lexer::Token = get_next_token(token_vec);
    assert!(tok.name == "Num" || tok.name == "Unary_Op", "Not an expression.");

    if (tok.name == "Num") {
        result.val = tok.value.parse::<i32>().unwrap();
        result.data_type = String::from("Int");
    }
    else if (tok.name == "Unary_Op") {
        result.data_type = tok.name.clone();
        result.op = tok.value.clone();
        result.inner_exp = Some(Box::new(parse_expression(token_vec)));
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

