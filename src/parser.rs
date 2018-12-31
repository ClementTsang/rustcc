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
    pub op : String,
    pub left_exp : Option<Box<Expression>>,
    pub left_term : Option<Box<Term>>,
    pub right_term : Option<Box<Term>>,
}

pub struct Term {
    pub op : String,
    pub left_term : Option<Box<Term>>,
    pub left_factor : Option<Box<Factor>>,
    pub right_factor : Option<Box<Factor>>,
}

pub struct Factor {
    pub op : String,
    pub unary : Option<Box<Unary>>,
    pub exp : Option<Box<Expression>>,
    pub val : Option<i32>,
}

pub struct Unary {
    pub op : String,
    pub right_fact : Option<Box<Factor>>,
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
            op : String::new(),
            left_exp : None,
            left_term : None,
            right_term : None,
        }
    }
}

impl Term {
    pub fn new() -> Term {
        Term {
            op: String::new(),
            left_term : None,
            left_factor : None,
            right_factor : None,
        }
    }
}

impl Factor { 
    pub fn new() -> Factor {
        Factor {
            op : String::new(),
            unary : None,
            exp: None,
            val : None,
        }
    }
}

impl Unary { 
    pub fn new() -> Unary {
        Unary {
            op : String::new(),
            right_fact : None,
        }
    }
}

impl Clone for Expression {
    fn clone(&self) -> Self {
        Expression { 
            op : self.op.clone(),
            left_exp : self.left_exp.clone(),
            left_term : self.left_term.clone(),
            right_term : self.right_term.clone(),
        }
    }
}

impl Clone for Term {
    fn clone(&self) -> Self {
        Term {
            op : self.op.clone(),
            left_term : self.left_term.clone(),
            left_factor : self.left_factor.clone(),
            right_factor : self.right_factor.clone(),
        }
    }
}

impl Clone for Factor {
    fn clone(&self) -> Self {
        Factor {
            op : self.op.clone(),
            unary : self.unary.clone(),
            exp : self.exp.clone(),
            val : self.val,
        }
    }
}

impl Clone for Unary { 
    fn clone(&self) -> Self {
        Unary {
            op : self.op.clone(),
            right_fact : self.right_fact.clone(),
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
    
    println!("\n=====END AST PRINT=====");
}


pub fn print_exp (exp : &Expression) {
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_term.clone() {
                Some(rterm) => {
                    print!("(");
                    print_exp(&*lexp);
                    print!(" {} ", exp.op);
                    print_term(&(*rterm));
                    print!(")");
                },
                None => {
                    print_exp(&*lexp);
                },
            }
        },
        None => {
            match exp.left_term.clone() {
                    Some(lterm) => {
                        match exp.right_term.clone() {
                            Some(rterm) => {
                                print!("(");
                                print_term(&(*lterm));
                                print!(" {} ", exp.op);
                                print_term(&(*rterm));
                                print!(")");
                            },
                            None => {
                                print_term(&(*lterm));
                            },
                        }
                    },
                    None => {
                    },
                }               
        },
    }
}




pub fn print_term (term : &Term) {
    match term.left_term.clone() {
        Some(lterm) => {
            match term.right_factor.clone() {
                Some(rfactor) => {
                    print!("(");
                    print_term(&*lterm);
                    print!(" {} ", term.op);
                    print_factor(&(*rfactor));
                    print!(")");
                },
                None => {
                    print_term(&*lterm);
                },
            }
        },
        None => {
            match term.left_factor.clone() {
                    Some(lfactor) => {
                        match term.right_factor.clone() {
                            Some(rfactor) => {
                                print!("(");
                                print_factor(&(*lfactor));
                                print!(" {} ", term.op);
                                print_factor(&(*rfactor));
                                print!(")");
                            },
                            None => {
                                print_factor(&(*lfactor));
                            },
                        }
                    },
                    None => {
                    },
                }               
        },
    }
}


pub fn print_factor (factor : &Factor) {
    match factor.unary.clone() {
        Some(un) => {
            print_unary(&*un);
        },
        None => {
            match factor.exp.clone() {
                Some(e) => {
                    print_exp(&*e);
                },
                None => {
                    match factor.val {
                        Some(v) => {
                            print!("{}", v);
                        },
                        None => {
                        },
                    }
                },
            }
        }
    }
}

pub fn print_unary (unary : &Unary) {
   print!("{}(", unary.op);
   match unary.right_fact.clone() {
        Some(fact) => {
            print_factor(&(*fact));
            print!(")");
        },
        None => {
        },
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

pub fn peek_next_token(token_vec : &Vec<lexer::Token>) -> lexer::Token {
    let tok : lexer::Token = get_option_token(token_vec.first().cloned());
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
    assert!(tok.value == ";", "Missing semicolon, saw {}", tok.value);
    
    result
}

pub fn parse_expression(token_vec : &mut Vec<lexer::Token>) -> Expression {
    let mut result : Expression = Expression::new();

    //println!("GENERATING EXP");
   
    let mut tok : lexer::Token = peek_next_token(token_vec);
    assert!(tok.name == "Num" ||
           tok.value == "(" ||
           tok.name == "Op", "Invalid term.");

    //println!("LEFT TERM TOKEN: {}", tok.value);
    result.left_term = Some(Box::new(parse_term(token_vec)));
    
    tok = peek_next_token(token_vec);
    //println!("TOK PEEK IN EXP: {}", tok.value);

    while (tok.value == "-" || tok.value == "+") {
        result.op = String::from(tok.value.clone());
        //println!("Set expr: {}", result.op);
        tok = get_next_token(token_vec);

        //println!("RIGHT TERM TOKEN: {}", tok.value.clone());
        result.right_term = Some(Box::new(parse_term(token_vec)));
        tok = peek_next_token(token_vec);
        //println!("IN +-, NEXT IS: {}", tok.value);
        
        result.left_exp = Some(Box::new(Expression {
            op : result.op.clone(),
            left_exp : result.left_exp.clone(),
            left_term : result.left_term.clone(),
            right_term : result.right_term.clone(),
        }));

        result.left_term = None;
        result.right_term = None;            
        result.op = String::new();
        //print!("RESULTING EXP: ");
        //print_exp(&result);
       // println!("");
    }
    result
}

pub fn parse_term(token_vec : &mut Vec<lexer::Token>) -> Term {
    let mut result : Term = Term::new();

   // println!("GENERATING TERM");
   
    let mut tok : lexer::Token = peek_next_token(token_vec);
    assert!(tok.name == "Num" ||
           tok.value == "(" ||
           tok.name == "Op", "Invalid factor.");

    //println!("LEFT FACTOR TOKEN: {}", tok.value);
    result.left_factor = Some(Box::new(parse_factor(token_vec)));
    
    tok = peek_next_token(token_vec);
    //println!("TOK PEEK IN TERM: {}", tok.value);

    while (tok.value == "*" || tok.value == "/") {
        result.op = String::from(tok.value.clone());
        //println!("Set expr: {}", result.op);
        tok = get_next_token(token_vec);
       
         if (tok.value == "/" && peek_next_token(token_vec).name == "Num") {
            if (peek_next_token(token_vec).value.parse::<i32>().unwrap() == 0) {
                println!("Tried dividing by zero.");
                std::process::exit(1);
            }
        }
        

        //println!("RIGHT FACTOR TOKEN: {}", tok.value.clone());
        result.right_factor = Some(Box::new(parse_factor(token_vec)));

       tok = peek_next_token(token_vec);
         
        //println!("IN */, NEXT IS: {}", tok.value);
        
        result.left_term = Some(Box::new(Term {
            op : result.op.clone(),
            left_term : result.left_term.clone(),
            left_factor : result.left_factor.clone(),
            right_factor : result.right_factor.clone(),
        }));

        result.left_factor = None;
        result.right_factor = None;            
        result.op = String::new();

        //print!("RESULTING TERM: ");
        //print_term(&result);
        //println!("");
    }

    result
}


pub fn parse_factor(token_vec : &mut Vec<lexer::Token>) -> Factor {
    let mut result : Factor = Factor::new();

    //println!("GENERATING FACTOR.");

    let mut tok : lexer::Token = peek_next_token(token_vec);
    
    if (tok.value == "(") {
        //println!("FOUND (");
        token_vec.remove(0);
        result.exp = Some(Box::new(parse_expression(token_vec)));
        tok = get_next_token(token_vec);
        assert!(tok.value==")", "Missing closing paren, saw {}.", tok.value);
        //println!("FOUND )");
    }
    else if (String::from("~!-").contains(tok.value.as_str())) {
        result.unary = Some(Box::new(parse_unary(token_vec)));
    }
    else if (tok.name == "Num") {
        //println!("GENERATED NUM: {}", tok.value);  
        result.val = Some(tok.value.parse::<i32>().unwrap());
        token_vec.remove(0);
    }

    result
}

pub fn parse_unary(token_vec : &mut Vec<lexer::Token>) -> Unary {
    let mut result : Unary = Unary::new();

    //println!("GENERATING UNARY");
    
    let mut tok : lexer::Token = get_option_token(token_vec.first().cloned());
    result.op = String::from(tok.value);
    token_vec.remove(0);
    result.right_fact = Some(Box::new(parse_factor(token_vec)));

    //println!("Generated unary: {}", result.op);

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

