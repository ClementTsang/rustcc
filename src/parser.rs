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
    pub exp : OrExpression,
}

pub struct OrExpression {
    pub op : String,
    pub left_exp : Option<Box<OrExpression>>,
    pub left_and_exp : Option<Box<AndExpression>>,
    pub right_and_exp : Option<Box<AndExpression>>,
}

pub struct AndExpression {
    pub op : String,
    pub left_exp : Option<Box<AndExpression>>,
    pub left_equal_exp : Option<Box<EqualityExp>>,
    pub right_equal_exp : Option<Box<EqualityExp>>,
}

pub struct EqualityExp {
    pub op : String,
    pub left_exp : Option<Box<EqualityExp>>,
    pub left_relation_exp : Option<Box<RelationalExp>>,
    pub right_relation_exp : Option<Box<RelationalExp>>,
}

pub struct RelationalExp {
    pub op : String,
    pub left_exp : Option<Box<RelationalExp>>,
    pub left_add_exp : Option<Box<AdditiveExp>>,
    pub right_add_exp : Option<Box<AdditiveExp>>,
}

pub struct AdditiveExp {
    pub op : String,
    pub left_exp : Option<Box<AdditiveExp>>,
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
    pub exp : Option<Box<OrExpression>>,
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
            exp : OrExpression::new(),
        }
    }
}

impl OrExpression {
    pub fn new() -> OrExpression {
        OrExpression {
            op : String::new(),
            left_exp : None,
            left_and_exp : None,
            right_and_exp : None,
        }
    }
}

impl AndExpression {
    pub fn new() -> AndExpression {
        AndExpression {
            op : String::new(),
            left_exp : None,
            left_equal_exp : None,
            right_equal_exp : None,
        }
    }
}

impl EqualityExp {
    pub fn new() -> EqualityExp {
        EqualityExp {
            op : String::new(),
            left_exp : None,
            left_relation_exp : None,
            right_relation_exp : None,
        }
    }
}

impl RelationalExp {
    pub fn new() -> RelationalExp {
        RelationalExp {
            op : String::new(),
            left_exp : None,
            left_add_exp : None,
            right_add_exp : None,
        }
    }
}

impl AdditiveExp {
    pub fn new() -> AdditiveExp {
        AdditiveExp {
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

impl Clone for OrExpression {
    fn clone(&self) -> Self {
        OrExpression { 
            op : self.op.clone(),
            left_exp : self.left_exp.clone(),
            left_and_exp : self.left_and_exp.clone(),
            right_and_exp : self.right_and_exp.clone(),
        }
    }
}

impl Clone for AndExpression {
    fn clone(&self) -> Self {
        AndExpression {
            op : self.op.clone(),
            left_exp : self.left_exp.clone(),
            left_equal_exp : self.left_equal_exp.clone(),
            right_equal_exp : self.right_equal_exp.clone(),
        }
    }
}

impl Clone for EqualityExp {
    fn clone(&self) -> Self {
        EqualityExp {
            op : self.op.clone(),
            left_exp : self.left_exp.clone(),
            left_relation_exp : self.left_relation_exp.clone(),
            right_relation_exp : self.right_relation_exp.clone(),
        }
    }
}


impl Clone for RelationalExp {
    fn clone(&self) -> Self {
        RelationalExp {
            op : self.op.clone(),
            left_exp : self.left_exp.clone(),
            left_add_exp : self.left_add_exp.clone(),
            right_add_exp : self.right_add_exp.clone(),
        }
    }
}

impl Clone for AdditiveExp {
    fn clone(&self) -> Self {
        AdditiveExp {
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
    print_or(&input_prog.fnc.st.exp);
    
    println!("\n=====END AST PRINT=====");
}


pub fn print_or (exp : &OrExpression) {
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_and_exp.clone() {
                Some(rand_exp) => {
                    print!("(");
                    print_or(&*lexp);
                    print!(" {} ", exp.op);
                    print_and(&*rand_exp);
                    print!(")");
                },
                None => {
                    print_or(&*lexp);
                },
            }
        },
        None => {
            match exp.left_and_exp.clone() {
                    Some(land_exp) => {
                        match exp.right_and_exp.clone() {
                            Some(rand_exp) => {
                                print!("(");
                                print_and(&*land_exp);
                                print!(" {} ", exp.op);
                                print_and(&(*rand_exp));
                                print!(")");
                            },
                            None => {
                                print_and(&*land_exp);
                            },
                        }
                    },
                    None => {
                    },
                }               
        },
    }
}

pub fn print_and (exp : &AndExpression) {
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_equal_exp.clone() {
                Some(r_child) => {
                    print!("(");
                    print_and(&*lexp);
                    print!(" {} ", exp.op);
                    print_eq(&*r_child);
                    print!(")");
                },
                None => {
                    print_and(&*lexp);
                },
            }
        },
        None => {
            match exp.left_equal_exp.clone() {
                    Some(lchild) => {
                        match exp.right_equal_exp.clone() {
                            Some(rchild) => {
                                print!("(");
                                print_eq(&*lchild);
                                print!(" {} ", exp.op);
                                print_eq(&(*rchild));
                                print!(")");
                            },
                            None => {
                                print_eq(&*lchild);
                            },
                        }
                    },
                    None => {
                    },
                }               
        },
    }
}

pub fn print_eq(exp : &EqualityExp) {
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_relation_exp.clone() {
                Some(r_child) => {
                    print!("(");
                    print_eq(&*lexp);
                    print!(" {} ", exp.op);
                    print_rel(&*r_child);
                    print!(")");
                },
                None => {
                    print_eq(&*lexp);
                },
            }
        },
        None => {
            match exp.left_relation_exp.clone() {
                    Some(lchild) => {
                        match exp.right_relation_exp.clone() {
                            Some(rchild) => {
                                print!("(");
                                print_rel(&*lchild);
                                print!(" {} ", exp.op);
                                print_rel(&(*rchild));
                                print!(")");
                            },
                            None => {
                                print_rel(&*lchild);
                            },
                        }
                    },
                    None => {
                    },
                }               
        },
    }
}

pub fn print_rel(exp : &RelationalExp) {
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_add_exp.clone() {
                Some(r_child) => {
                    print!("(");
                    print_rel(&*lexp);
                    print!(" {} ", exp.op);
                    print_add(&*r_child);
                    print!(")");
                },
                None => {
                    print_rel(&*lexp);
                },
            }
        },
        None => {
            match exp.left_add_exp.clone() {
                    Some(lchild) => {
                        match exp.right_add_exp.clone() {
                            Some(rchild) => {
                                print!("(");
                                print_add(&*lchild);
                                print!(" {} ", exp.op);
                                print_add(&(*rchild));
                                print!(")");
                            },
                            None => {
                                print_add(&*lchild);
                            },
                        }
                    },
                    None => {
                    },
                }               
        },
    }
}

pub fn print_add(exp : &AdditiveExp) {
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_term.clone() {
                Some(r_child) => {
                    print!("(");
                    print_add(&*lexp);
                    print!(" {} ", exp.op);
                    print_term(&*r_child);
                    print!(")");
                },
                None => {
                    print_add(&*lexp);
                },
            }
        },
        None => {
            match exp.left_term.clone() {
                    Some(lchild) => {
                        match exp.right_term.clone() {
                            Some(rchild) => {
                                print!("(");
                                print_term(&*lchild);
                                print!(" {} ", exp.op);
                                print_term(&(*rchild));
                                print!(")");
                            },
                            None => {
                                print_term(&*lchild);
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
                    print_or(&*e);
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
    result.exp = parse_or_exp(token_vec);

    tok = get_next_token(token_vec);
    assert!(tok.value == ";", "Missing semicolon, saw {}", tok.value);
    
    result
}

pub fn valid_unary(s : String) -> bool {
    let op = vec!["!", "~", "-", "+"];
    return op.contains(&(s.as_str()));
}

pub fn parse_or_exp(token_vec : &mut Vec<lexer::Token>) -> OrExpression {
    let mut result : OrExpression = OrExpression::new();

    let mut tok : lexer::Token = peek_next_token(token_vec);
    assert!(tok.name == "Num" ||
           tok.value == "(" ||
           valid_unary(tok.value), "Invalid or_exp.");

    result.left_and_exp = Some(Box::new(parse_and_exp(token_vec)));
    
    tok = peek_next_token(token_vec);

    while (tok.value == "||") {
        result.op = String::from(tok.value.clone());
        get_next_token(token_vec);

        result.right_and_exp = Some(Box::new(parse_and_exp(token_vec)));
        tok = peek_next_token(token_vec);

        result.left_exp = Some(Box::new(OrExpression {
            op : result.op.clone(),
            left_exp : result.left_exp.clone(),
            left_and_exp : result.left_and_exp.clone(),
            right_and_exp : result.right_and_exp.clone(),
        }));

        result.left_and_exp = None;
        result.right_and_exp = None;            
        result.op = String::new();
    }
    result
}

pub fn parse_and_exp(token_vec : &mut Vec<lexer::Token>) -> AndExpression {
    let mut result : AndExpression = AndExpression::new();

   
    let mut tok : lexer::Token = peek_next_token(token_vec);
    assert!(tok.name == "Num" ||
           tok.value == "(" ||
           valid_unary(tok.value), "Invalid and_exp.");

    result.left_equal_exp = Some(Box::new(parse_equal_exp(token_vec)));
    
    tok = peek_next_token(token_vec);

    while (tok.value == "&&") {
        result.op = String::from(tok.value.clone());
        get_next_token(token_vec);

        result.right_equal_exp = Some(Box::new(parse_equal_exp(token_vec)));
        tok = peek_next_token(token_vec);
        
        result.left_exp = Some(Box::new(AndExpression {
            op : result.op.clone(),
            left_exp : result.left_exp.clone(),
            left_equal_exp : result.left_equal_exp.clone(),
            right_equal_exp : result.right_equal_exp.clone(),
        }));

        result.left_equal_exp = None;
        result.right_equal_exp = None;            
        result.op = String::new();
    }
    result
}

pub fn parse_equal_exp(token_vec : &mut Vec<lexer::Token>) -> EqualityExp {
    let mut result : EqualityExp = EqualityExp::new();

    let mut tok : lexer::Token = peek_next_token(token_vec);
    assert!(tok.name == "Num" ||
           tok.value == "(" ||
           valid_unary(tok.value), "Invalid equal_exp.");

    result.left_relation_exp = Some(Box::new(parse_rel_exp(token_vec)));
    
    tok = peek_next_token(token_vec);

    while (tok.value == "==" || tok.value == "!=") {
        result.op = String::from(tok.value.clone());
        get_next_token(token_vec);

        result.right_relation_exp = Some(Box::new(parse_rel_exp(token_vec)));
        tok = peek_next_token(token_vec);
        
        result.left_exp = Some(Box::new(EqualityExp {
            op : result.op.clone(),
            left_exp : result.left_exp.clone(),
            left_relation_exp : result.left_relation_exp.clone(),
            right_relation_exp : result.right_relation_exp.clone(),
        }));

        result.left_relation_exp = None;
        result.right_relation_exp = None;            
        result.op = String::new();
    }
    result
}


pub fn parse_rel_exp(token_vec : &mut Vec<lexer::Token>) -> RelationalExp {
    let mut result : RelationalExp = RelationalExp::new();

    let mut tok : lexer::Token = peek_next_token(token_vec);
    assert!(tok.name == "Num" ||
           tok.value == "(" ||
           valid_unary(tok.value), "Invalid rel_exp.");

    result.left_add_exp = Some(Box::new(parse_add_exp(token_vec)));
    
    tok = peek_next_token(token_vec);

    while (tok.value == ">" || tok.value == ">=" || tok.value == "<" || tok.value == "<=") {
        result.op = String::from(tok.value.clone());
        get_next_token(token_vec);

        result.right_add_exp = Some(Box::new(parse_add_exp(token_vec)));
        tok = peek_next_token(token_vec);
        
        result.left_exp = Some(Box::new(RelationalExp {
            op : result.op.clone(),
            left_exp : result.left_exp.clone(),
            left_add_exp : result.left_add_exp.clone(),
            right_add_exp : result.right_add_exp.clone(),
        }));

        result.left_add_exp = None;
        result.right_add_exp = None;            
        result.op = String::new();
    }


    result
}


pub fn parse_add_exp(token_vec : &mut Vec<lexer::Token>) -> AdditiveExp {
    let mut result : AdditiveExp = AdditiveExp::new();

    let mut tok : lexer::Token = peek_next_token(token_vec);
    assert!(tok.name == "Num" ||
           tok.value == "(" ||
           valid_unary(tok.value.clone()), "Invalid add_exp.");

    // Edge case for if there is no value to the right of the addition:
    result.left_term = Some(Box::new(parse_term(token_vec)));
    if (tok.value == "+") {
        match result.left_term.clone() {
            Some(x) => {
                match (&*x).left_term.clone() {
                    Some(y) => (),
                    None => {
                        match (&*x).left_factor.clone() {
                            None => (),
                            Some (y) => {
                                match (&*x).right_factor.clone() {
                                    Some (y) => (),
                                    None => {
                                        result.left_term = Some(Box::new(Term{
                                            op : String::new(),
                                            left_term : None,
                                            left_factor: Some(Box::new(Factor {
                                                op : String::new(),
                                                unary : None,
                                                exp : None,
                                                val : Some(0),
                                            })),
                                            right_factor : None,
                                        }));
                                    }
                                }
                            },
                        }
                    },
                }
            },
            None => (),
        }
    }
    
    tok = peek_next_token(token_vec);

    while (tok.value == "-" || tok.value == "+") {
        result.op = String::from(tok.value.clone());
        get_next_token(token_vec);

        result.right_term = Some(Box::new(parse_term(token_vec)));
        tok = peek_next_token(token_vec);
        
        result.left_exp = Some(Box::new(AdditiveExp {
            op : result.op.clone(),
            left_exp : result.left_exp.clone(),
            left_term: result.left_term.clone(),
            right_term : result.right_term.clone(),
        }));

        result.left_term= None;
        result.right_term= None;            
        result.op = String::new();
    }


    result
}

pub fn parse_term(token_vec : &mut Vec<lexer::Token>) -> Term {
    let mut result : Term = Term::new();

   // println!("GENERATING TERM");
   
    let mut tok : lexer::Token = peek_next_token(token_vec);
    assert!(tok.name == "Num" ||
           tok.value == "(" ||
           valid_unary(tok.value), "Invalid term.");

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
        result.exp = Some(Box::new(parse_or_exp(token_vec)));
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
    
    let tok : lexer::Token = get_option_token(token_vec.first().cloned());
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

