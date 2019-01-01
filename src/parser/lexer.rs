#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]

use std::env;
use std::fs;
use std::str;
use std::clone;
use std::fmt;


pub struct Token {
    pub name : String,
    pub value : String,
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


pub fn is_number (c : char) -> bool {
   let nums = "0123456789";
   nums.contains(c)
}

pub fn is_letter (c : char) -> bool {
    let letters = "abcdefghijklmnopqrstuvwxyz";
    letters.contains(c)
}

pub fn is_punctuation (c : char) -> bool {
    let punc = "{}();";
    punc.contains(c)
}

pub fn is_op (c : char) -> bool {
    let op = "-~!+*/";
    op.contains(c)
}

pub fn is_unary (c : char) -> bool {
    let op = "-~!";
    op.contains(c)
}

pub fn is_binary (c : char) -> bool {
    let op = "+*/-";
    op.contains(c)
}

pub fn is_multi_check (c : char) -> bool {
    let op = "=|&";
    op.contains(c)
}

pub fn is_multi_op (val : &str) -> bool {
    let op = vec!["<", ">", "<=", ">=", "==", "!=", "||", "&&"];
    op.contains(&val)
}

pub fn is_assign_op (c : char) -> bool {
    c == '='
}

pub fn read_identifier (input : &mut String) -> Token {
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

pub fn read_number (input : &mut String) -> Token {
    let mut iden_name = String::new();
    let tmp = input.clone();
    for c in tmp.chars() {
        if (!c.is_whitespace() && is_number(c)) {
            iden_name.push(c);
            input.remove(0);
        }
        else {
            break;
        }
    }

    Token {name : String::from("Num"), value : iden_name}
}

pub fn read_punc (input : &mut String) -> Token {
    let ret_punc = input.chars().next().unwrap().to_string();
    input.remove(0); 
    Token {name : String::from("Punc"), value : ret_punc} 
}

pub fn read_op (input : &mut String) -> Token {
    let ret_op = input.chars().next().unwrap().to_string();
    input.remove(0);
    Token{name : String::from("Op"), value : ret_op}
}

pub fn read_assign_op(input : &mut String) -> Token {
    input.remove(0);
    Token{name : String::from("Assign"), value : String::from("=")}
}

pub fn read_multi_op(input : &mut String, ret_op : String) -> Token {
    for x in 0..ret_op.len() {
        input.remove(0);
    }
    Token{name : String::from("Op"), value : ret_op}
}

pub fn lexer(input : &mut String) -> Vec<Token> {
    let mut token_vec : Vec<Token> = Vec::new();
    let mut c : char;

    while (input.len() > 0) {
        c = input.chars().next().unwrap();

        if (!c.is_whitespace()) {
            if (input.len() > 1) {
                let mut tmp_input = input.clone();
                tmp_input.remove(0);
                let tmp_char : char = tmp_input.chars().next().unwrap();

                let test_val : String = if (tmp_char.is_whitespace() || !is_multi_check(tmp_char)) {
                    c.to_string()
                } 
                else { 
                    c.to_string() + tmp_char.to_string().as_str()
                };

                if (is_multi_op(test_val.as_str())) {
                    token_vec.push(read_multi_op(input, test_val.clone()));      
                    continue;
                }
            }
            if (is_letter(c)) {
                // Must be identifier, as no quotes (not supported yet).
                token_vec.push(read_identifier(input));
            }
            else if (is_assign_op(c)) {
                token_vec.push(read_assign_op(input));
            }
            else if (is_number(c)) {
                token_vec.push(read_number(input));
            }
            else if (is_punctuation(c)) {
                token_vec.push(read_punc(input));
            }
            else if (is_op(c)) {
                token_vec.push(read_op(input));
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

