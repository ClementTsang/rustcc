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


use std::env;
use std::fs;
use std::str;
use std::clone;
use std::fmt;
//use regex::Regex;


struct Program { 
    fnc : Function,
}

struct Function {
    name : String,
    return_type : String,
    st : Statement,
}

struct Statement {
    name : String,
    exp : Expression,
}

struct Expression {
    data_type : String,
    val : i32,
}

struct Token {
    name : String,
    value : String,
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token { name: self.name.clone(), value: self.value.clone()}
    }
}

impl fmt::Display for Token {
    fn fmt (&self, f: &mut fmt:: Formatter) -> fmt::Result {
        write!(f, "name: {}, value: {}", self.name, self.value)
    }
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
    let keywords = vec!["int", "return"];


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

    if (keywords.contains(&&*iden_name)) {
        Token {name : String::from("Keyword"), value : iden_name}
    }
    else {
        Token {name : String::from("Identifier"), value : iden_name}
    }
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



fn print_ast (input_prog : &Program) {
    println!("=====AST PRINT=====");

    println!("FUN {} {}:", input_prog.fnc.return_type, input_prog.fnc.name);
    println!("     params: (TO BE ADDED)");
    println!("     body:");
    println!("          {} {} <{}>", input_prog.fnc.st.name, input_prog.fnc.st.exp.data_type, input_prog.fnc.st.exp.val);
        
    println!("=====END AST PRINT=====");
}

fn print_assembly(){
    
}

fn parse_program(token_vec : &mut Vec<Token>) -> Program {
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

fn get_option_token( op : Option<Token> ) -> Token {
    match op {
        None => { 
            println!("Failed to get token from option");
            std::process::exit(1);
        },
        Some(x) => { x },
    }
}

fn get_next_token(token_vec : &mut Vec<Token>) -> Token {
    let tok : Token = get_option_token(token_vec.first().cloned());
    token_vec.remove(0);
    //println!("Token obtained: {}", tok);
    tok
}

fn parse_function(token_vec : &mut Vec<Token>) -> Function {
    let mut result : Function  = Function {
        name : String::from(""), 
        return_type : String::from(""),
        st : Statement { 
            name: String::from(""), 
            exp : Expression { 
                data_type: String::from(""), val : 0 }
        }
    };

    let mut tok : Token = get_next_token(token_vec);
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

fn parse_statement(token_vec : &mut Vec<Token>) -> Statement {
    let mut result : Statement = Statement {
        name: String::from(""), 
        exp : Expression { 
            data_type: String::from(""), val : 0 
        }
    }; 
    
    let mut tok : Token = get_next_token(token_vec);
    assert!(tok.name == "Keyword" && tok.value == "return", "Invalid keyword");
    result.name = tok.value;

    //Expression check
    result.exp = parse_expression(token_vec);

    tok = get_next_token(token_vec);
    assert!(tok.value == ";", "Missing semicolon");
    
    result
}

fn parse_expression(token_vec : &mut Vec<Token>) -> Expression {
    let mut result : Expression = Expression {
        data_type: String::from(""), val:0
    }; 
    
    let mut tok : Token = get_next_token(token_vec);
    assert!(tok.name == "Num", "Not a value.");
    result.val = tok.value.parse::<i32>().unwrap();

    if (tok.name == "Num") {
        result.data_type = String::from("Int");
    }

    result
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

    // Convert to tokens
    let mut token_vec : Vec<Token> = lexer(&mut file_contents);
    println!("=====Resulting tokens=====");
    for token in &token_vec {
        println!("Token: {}", token);
    }
    println!("=====End of tokens=====");

    // Parse tokens into AST
    let result_AST : Program = parse_program(&mut token_vec); 
    print_ast(&result_AST);

    result_AST;
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
