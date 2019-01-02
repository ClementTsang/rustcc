#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(while_true)]

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
    pub list_of_st : Vec<Statement>,
    pub params : Vec<String>,
}

pub struct Statement {
    pub name : String,
    pub exp : Option<Assignment>,
    pub decl : Option<Declaration>,
}

pub struct Assignment {
    pub var : Option<Variable>,
    pub assign : Option<Box<Assignment>>,
    pub exp : Option<OrExpression>,
}

pub struct Declaration {
    pub var : Variable,
    pub exp : Assignment,
    pub var_type : String,
}

pub struct Variable {
    pub var_name : String,
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
    pub exp : Option<Box<Assignment>>,
    pub val : Option<i32>,
    pub var : Option<Variable>,
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
            list_of_st : Vec::new(),
            params : Vec::new(),
        }
    }
}

impl Statement {
    pub fn new () -> Statement {
        Statement {
            name : String::new(),
            exp : None,
            decl : None,
        }
    }
}

impl Assignment {
    pub fn new() -> Assignment {
        Assignment {
            var : None,
            assign : None,
            exp : None,
        }
    }

    // This is horrible.
    pub fn set_to_zero() -> Assignment {
        Assignment {
            var : None,
            assign : None,
            exp : Some(OrExpression {
                op : String::new(),
                left_exp : None,
                left_and_exp : Some(Box::new(AndExpression{
                    op : String::new(),
                    left_exp : None,
                    left_equal_exp : Some(Box::new(EqualityExp{
                        op : String::new(),
                        left_exp : None,
                        left_relation_exp : Some(Box::new(RelationalExp {
                            op : String::new(),
                            left_exp : None,
                            left_add_exp : Some(Box::new(AdditiveExp {
                                op : String::new(),
                                left_exp : None,
                                left_term : Some(Box::new(Term {
                                    op : String::new(),
                                    left_term : None,
                                    left_factor : Some(Box::new(Factor {
                                        op : String::new(),
                                        unary : None,
                                        exp : None,
                                        val : Some(0),
                                        var : None,
                                    })),
                                    right_factor : None,
                                })),
                                right_term : None,
                            })),
                            right_add_exp : None,
                        })),
                        right_relation_exp : None,
                    })),
                    right_equal_exp : None,
                })),
                right_and_exp : None,
            }),
        }
    }
}

impl Declaration {
    pub fn new() -> Declaration {
        Declaration {
            var : Variable::new(),
            exp : Assignment::new(),
            var_type : String::new(),
        }
    }
}

impl Variable {
    pub fn new() -> Variable {
        Variable {
            var_name : String::new(),
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
            var : None,
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

impl Clone for Variable {
    fn clone(&self) -> Self {
        Variable {
            var_name : self.var_name.clone(),
        }
    }
}

impl Clone for Declaration {
    fn clone (&self) -> Self {
        Declaration {
            var : self.var.clone(),
            exp : self.exp.clone(),
            var_type : self.var_type.clone(),
        }
    }
}

impl Clone for Assignment {
    fn clone(&self) -> Self {
        Assignment {
            var : self.var.clone(),
            assign : self.assign.clone(),
            exp : self.exp.clone(),
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
            var: self.var.clone(),
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
   
    for st in &input_prog.fnc.list_of_st {
        print!("          {} ", st.name);
        match st.exp.clone() {
            Some (x) => {
                print!("[ ");
                print_assignment(&x);
                println!(" ]");
            },
            None => {
                match st.decl.clone() {
                    Some (x) => {
                        print!("[ ");
                        print_declaration(&x);
                        println!(" ]");
                    },
                    None => (),
                }
            },
        }
    }
    
    println!("=====END AST PRINT=====");
}

pub fn print_declaration (decl : &Declaration) {
    print!("{} {} = ", decl.var_type, decl.var.var_name);
    print_assignment(&decl.exp);

}

pub fn print_assignment (exp : &Assignment) {

    match exp.var.clone() {
        Some(var) => {
            print!("(");
            print!("{} = ", var.var_name);
        },
        None => (),
    }

    match exp.assign.clone() {
        Some(assign) => {
            print_assignment(&*assign);
        },
        None => {
            match exp.exp.clone() {
                Some(exp) => {
                    print_or(&exp);
                    print!(")");
                },
                None => ()
            }
        },
    }
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
                    print_assignment(&*e);
                },
                None => {
                    match factor.val {
                        Some(v) => {
                            print!("{}", v);
                        },
                        None => {
                            match factor.var.clone() {
                                Some(v) => {
                                    print!("{}", v.var_name);
                                },
                                None => (),
                            }
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
    get_option_token(token_vec.first().cloned())
}

pub fn peek_two_tokens(token_vec : &Vec<lexer::Token>) -> lexer::Token {
    let mut tok_clone = token_vec.clone();
    tok_clone.remove(0);
    get_option_token(tok_clone.first().cloned())
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
    while (peek_two_tokens(token_vec).value != "EOF" && peek_two_tokens(token_vec).name != "EOF TOKEN") {
        result.list_of_st.push(parse_statement(token_vec));
    }

    tok = get_next_token(token_vec);
    assert!(tok.name == "Punc" && tok.value == "}", "Invalid punc. (\"}\")");

    result
}



pub fn parse_statement(token_vec : &mut Vec<lexer::Token>) -> Statement {
    let mut result : Statement = Statement::new();
    let mut tok : lexer::Token = peek_next_token(token_vec);
    
    // We must set this up to accept three cases: 
    // * Var dec
    // * Var assign
    // * Return
  
    // Get expression

    // Modify this statement based on expression/tok values
    if (tok.value == "return") {
        result.name = String::from("return");
        token_vec.remove(0);
        result.exp = Some(parse_assign(token_vec));
    }
    else if (tok.name == "Keyword") {
        result.name = String::from("declare");
        result.decl = Some(parse_declaration(token_vec));
    }
    else {
        result.name = String::from("exp");
        result.exp = Some(parse_assign(token_vec));
    }

    tok = get_next_token(token_vec);

    assert!(tok.value == ";", "Missing semicolon, saw {}", tok.value);
    
    result
}

pub fn parse_declaration(token_vec : &mut Vec<lexer::Token>) -> Declaration {
    let mut result : Declaration = Declaration::new();
    let mut tok : lexer::Token = peek_next_token(token_vec);

    assert!(tok.name == "Keyword" && 
            (tok.value == "int"), 
            "Invalid declaration, saw: {}", tok.value);

    result.var_type = tok.value;
    token_vec.remove(0);
    tok = get_next_token(token_vec);
    result.var = Variable {var_name : tok.value.clone()};

    tok = peek_next_token(token_vec);
    if (tok.value == "=" && tok.name == "AssignOp") {
        token_vec.remove(0);
        result.exp = parse_assign(token_vec);
    }
    else {
        // Default to zero.
        result.exp = Assignment::set_to_zero();
    }

    result
}

pub fn parse_assign(token_vec : &mut Vec<lexer::Token>) -> Assignment {
    let mut result : Assignment = Assignment::new();
    let tok : lexer::Token = peek_next_token(token_vec);

    assert!(tok.name == "Num" ||
           tok.value == "(" ||
           valid_unary(tok.value.clone()) ||
           tok.name == "Identifier",
        "Invalid assignment, saw: {}", tok.value);
    
    let mut next_tok = peek_two_tokens(&token_vec);

    if (tok.name == "Identifier" && next_tok.value == "=") {

        result.var = Some(Variable {var_name : tok.value.clone()});
        while (next_tok.value == "=") {

            token_vec.remove(0);
            token_vec.remove(0);

            result.exp = Some(parse_or_exp(token_vec));
            
            result.assign = Some(Box::new(Assignment {
                var : result.var.clone(),
                assign : result.assign.clone(),
                exp : result.exp.clone(),
            }));

            result.var = None;
            result.exp = None;

            next_tok = peek_next_token(token_vec);
        }
    }
    else {
        // Not an assignment, move on.
        result.exp = Some(parse_or_exp(token_vec));
    }

    result
}

pub fn valid_unary(s : String) -> bool {
    let op = vec!["!", "~", "-", "+"]; // Cheated, "+" is binary but can be used as unary (adds 0) 
    return op.contains(&(s.as_str()));
}

pub fn parse_or_exp(token_vec : &mut Vec<lexer::Token>) -> OrExpression {
    let mut result : OrExpression = OrExpression::new();
    let mut tok : lexer::Token = peek_next_token(token_vec);

    assert!(tok.name == "Num" ||
           tok.value == "(" ||
           valid_unary(tok.value.clone()) ||
           tok.name == "Identifier",
           "Invalid or_exp, saw \"{}\" : \"{}\"...", tok.name, tok.value);

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
           valid_unary(tok.value.clone()) ||
           tok.name == "Identifier", "Invalid and_exp, saw {}.", tok.value);

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
           valid_unary(tok.value.clone()) ||
           tok.name == "Identifier",
           "Invalid equal_exp: {}.", tok.value);

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
           valid_unary(tok.value.clone()) ||
           tok.name == "Identifier",
           "Invalid rel_exp: {}.", tok.value);

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
           valid_unary(tok.value.clone()) ||
           tok.name == "Identifier", 
           "Invalid add_exp: {}.", tok.value);

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
                                                var : None,
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
    let mut tok : lexer::Token = peek_next_token(token_vec);

    assert!(tok.name == "Num" ||
           tok.value == "(" ||
           valid_unary(tok.value.clone()) ||
           tok.name == "Identifier", "Invalid term: {}.", tok.value);

    result.left_factor = Some(Box::new(parse_factor(token_vec)));
    tok = peek_next_token(token_vec);

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
         
        result.left_term = Some(Box::new(Term {
            op : result.op.clone(),
            left_term : result.left_term.clone(),
            left_factor : result.left_factor.clone(),
            right_factor : result.right_factor.clone(),
        }));

        result.left_factor = None;
        result.right_factor = None;            
        result.op = String::new();
    }

    result
}


pub fn parse_factor(token_vec : &mut Vec<lexer::Token>) -> Factor {
    let mut result : Factor = Factor::new();
    let mut tok : lexer::Token = peek_next_token(token_vec);
    
    if (tok.value == "(") {
        token_vec.remove(0);
        result.exp = Some(Box::new(parse_assign(token_vec)));
        tok = get_next_token(token_vec);
        assert!(tok.value==")", "Missing closing paren, saw {}.", tok.value);
    }
    else if (String::from("~!-").contains(tok.value.as_str())) {
        result.unary = Some(Box::new(parse_unary(token_vec)));
    }
    else if (tok.name == "Num") {
        result.val = Some(tok.value.clone().parse::<i32>().unwrap());
        token_vec.remove(0);
        //println!("Found num: {}", tok.value);
    }
    else if (tok.name == "Identifier") {
        result.var = Some(Variable { var_name : tok.value.clone() });
        token_vec.remove(0);
        //println!("Found identifier: {}", tok.value);
    }

    result
}

pub fn parse_unary(token_vec : &mut Vec<lexer::Token>) -> Unary {
    let mut result : Unary = Unary::new();
    let tok : lexer::Token = get_option_token(token_vec.first().cloned());

    result.op = String::from(tok.value);
    token_vec.remove(0);
    result.right_fact = Some(Box::new(parse_factor(token_vec)));

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

