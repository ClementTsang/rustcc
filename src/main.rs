#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

/**
 * To do:
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
        result.push_str(generate_or_expr(&st.exp).as_str());
        result.push_str("    ret");
    }
    result
}

fn generate_or_rchild(expr : &parser::OrExpression, rchild : &parser::AndExpression) -> String {
    let mut result = String::new();
    result.push_str("    push    %eax\n");
    result.push_str(generate_and_expr(&*rchild).as_str());
    result.push_str("    pop     %ecx\n");
    result.push_str("    orl     %ecx, %eax\n");
    result.push_str("    movl    $0, %eax\n");
    result.push_str("    setne   %al\n");

    result
}

fn generate_or_expr(exp : &parser::OrExpression) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_and_exp.clone() {
                Some(rchild) => {
                    result.push_str(generate_or_expr(&*lexp).as_str());
                    result.push_str(generate_or_rchild(exp, &*rchild).as_str());
                },
                None => {
                    result.push_str(generate_or_expr(&*lexp).as_str());
                },
            }
        },
        None => {
            match exp.left_and_exp.clone() {
                    Some(lchild) => {
                        match exp.right_and_exp.clone() {
                            Some(rchild) => {
                                result.push_str(generate_and_expr(&*lchild).as_str());
                                result.push_str(generate_or_rchild(exp, &*rchild).as_str());
                            },
                            None => {
                                result.push_str(generate_and_expr(&*lchild).as_str());
                            },
                        }
                    },
                    None => {
                    },
                }               
        },
    }

    result
}

fn generate_and_rchild(expr : &parser::AndExpression, rchild : &parser::EqualityExp) -> String {
    let mut result = String::new();

    result.push_str("    pushl   %eax\n");
    result.push_str(generate_eq_expr(rchild).as_str());
    result.push_str("    popl    %ecx\n");
    result.push_str("    cmpl    $0, %ecx\n");
    result.push_str("    setne   %cl\n");
    result.push_str("    cmpl    $0, %eax\n");
    result.push_str("    movl    $0, %eax\n");
    result.push_str("    setne   %al\n");
    result.push_str("    andb    %cl, %al\n");

    result
}

fn generate_and_expr(exp : &parser::AndExpression) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_equal_exp.clone() {
                Some(rchild) => {
                    result.push_str(generate_and_expr(&*lexp).as_str());
                    result.push_str(generate_and_rchild(exp, &*rchild).as_str());
                },
                None => {
                    result.push_str(generate_and_expr(&*lexp).as_str());
                },
            }
        },
        None => {
            match exp.left_equal_exp.clone() {
                    Some(lchild) => {
                        match exp.right_equal_exp.clone() {
                            Some(rchild) => {
                                result.push_str(generate_eq_expr(&*lchild).as_str());
                                result.push_str(generate_and_rchild(exp, &*rchild).as_str());
                            },
                            None => {
                                result.push_str(generate_eq_expr(&*lchild).as_str());
                            },
                        }
                    },
                    None => {
                    },
                }               
        },
    }

    result
}

fn generate_eq_rchild(expr : &parser::EqualityExp, rchild : &parser::RelationalExp) -> String {
    let mut result = String::new();

    result.push_str("    pushl    %eax\n");
    result.push_str(generate_rel_expr(&*rchild).as_str());
    result.push_str("    popl     %ecx\n");
    result.push_str("    cmpl     %eax, %ecx\n");
    result.push_str("    movl     %ecx, %eax\n");

    match expr.op.as_str() {
        "==" => {
            result.push_str("    sete     %al\n");
        },
        "!=" => {
            result.push_str("    setne    %al\n");
        },
        _ => {
            println!("Found an unwritten binary(Expr): {}", expr.op.as_str());
            std::process::exit(1);
        },
    }

    result
}

fn generate_eq_expr(exp : &parser::EqualityExp) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_relation_exp.clone() {
                Some(rchild) => {
                    result.push_str(generate_eq_expr(&*lexp).as_str());
                    result.push_str(generate_eq_rchild(exp, &*rchild).as_str());
                },
                None => {
                    result.push_str(generate_eq_expr(&*lexp).as_str());
                },
            }
        },
        None => {
            match exp.left_relation_exp.clone() {
                    Some(lchild) => {
                        match exp.right_relation_exp.clone() {
                            Some(rchild) => {
                                result.push_str(generate_rel_expr(&*lchild).as_str());
                                result.push_str(generate_eq_rchild(exp, &*rchild).as_str());
                            },
                            None => {
                                result.push_str(generate_rel_expr(&*lchild).as_str());
                            },
                        }
                    },
                    None => {
                    },
                }               
        },
    }

    result
}

fn generate_rel_rchild(expr : &parser::RelationalExp, rchild : &parser::AdditiveExp) -> String {
    let mut result = String::new();

    result.push_str("    pushl    %eax\n");
    result.push_str(generate_add_expr(&*rchild).as_str());
    result.push_str("    popl     %ecx\n");
    result.push_str("    cmpl     %eax, %ecx\n");
    result.push_str("    movl     %ecx, %eax\n");

    match expr.op.as_str() {
        "<" => {
            result.push_str("    setl     %al\n");
        },
        ">" => {
            result.push_str("    setg     %al\n");
        },
        "<=" => {
            result.push_str("    setle    %al\n");
        },
        ">=" => {
            result.push_str("    setge    %al\n");
        },
        _ => {
            println!("Found an unwritten binary(Expr): {}", expr.op.as_str());
            std::process::exit(1);
        },
    }

    result
}

fn generate_rel_expr(exp : &parser::RelationalExp) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_add_exp.clone() {
                Some(rchild) => {
                    result.push_str(generate_rel_expr(&*lexp).as_str());
                    result.push_str(generate_rel_rchild(exp, &*rchild).as_str());
                },
                None => {
                    result.push_str(generate_rel_expr(&*lexp).as_str());
                },
            }
        },
        None => {
            match exp.left_add_exp.clone() {
                    Some(lchild) => {
                        match exp.right_add_exp.clone() {
                            Some(rchild) => {
                                result.push_str(generate_add_expr(&*lchild).as_str());
                                result.push_str(generate_rel_rchild(exp, &*rchild).as_str());
                            },
                            None => {
                                result.push_str(generate_add_expr(&*lchild).as_str());
                            },
                        }
                    },
                    None => {
                    },
                }               
        },
    }

    result
}

fn generate_add_expr_rterm(expr : &parser::AdditiveExp, rterm : &parser::Term) -> String {
    let mut result = String::new();
    match expr.op.as_str() {
        "-" => {
            result.push_str("    pushl   %eax\n");
            result.push_str(generate_term(&*rterm).as_str());
            result.push_str("    pushl   %eax\n");
            result.push_str("    popl    %ecx\n");
            result.push_str("    popl    %eax\n");
            result.push_str("    subl    %ecx, %eax\n");
        },
        "+" => {
            result.push_str("    push    %eax\n");
            result.push_str(generate_term(&*rterm).as_str());
            result.push_str("    popl    %ecx\n");
            result.push_str("    addl    %ecx, %eax\n");

        },
        _ => {
            println!("Found an unwritten binary(Expr): {}", expr.op.as_str());
            std::process::exit(1);
        },
    }

    result
}

fn generate_add_expr(exp : &parser::AdditiveExp) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_term.clone() {
                Some(rterm) => {
                    result.push_str(generate_add_expr(&*lexp).as_str());
                    result.push_str(generate_add_expr_rterm(exp, &*rterm).as_str());
                },
                None => {
                    result.push_str(generate_add_expr(&*lexp).as_str());
                },
            }
        },
        None => {
            match exp.left_term.clone() {
                    Some(lterm) => {
                        match exp.right_term.clone() {
                            Some(rterm) => {
                                result.push_str(generate_term(&*lterm).as_str());
                                result.push_str(generate_add_expr_rterm(exp, &*rterm).as_str());
                            },
                            None => {
                                result.push_str(generate_term(&*lterm).as_str());
                            },
                        }
                    },
                    None => {
                    },
                }               
        },
    }

    result
}

fn generate_term_rfactor(term : &parser::Term, rfactor : &parser::Factor) -> String {
    let mut result = String::new();
    match term.op.as_str() {
        "*" => {
            result.push_str("    pushl   %eax\n");
            result.push_str(generate_factor(&*rfactor).as_str());
            result.push_str("    popl    %ecx\n");
            result.push_str("    imul    %ecx, %eax\n");
        },
        "/" => {
            result.push_str("    pushl   %eax\n");
            result.push_str(generate_factor(&*rfactor).as_str());
            result.push_str("    pushl  %eax\n");
            result.push_str("    popl   %ecx\n");
            result.push_str("    popl   %eax\n");
            result.push_str("    movl    $0, %edx\n");  //Zero out edx
            result.push_str("    idivl   %ecx\n"); //ecx is divisor

        },
        _ => {
            println!("Found an unwritten Binary(Term): {}", term.op.as_str());
            std::process::exit(1);
        },
    }

    result
}

fn generate_term(term : &parser::Term) -> String {
    let mut result = String::new();
    match term.left_term.clone() {
        Some(lterm) => {
            match term.right_factor.clone() {
                Some(rfactor) => {
                    result.push_str(generate_term(&*lterm).as_str());
                    result.push_str(generate_term_rfactor(term, &*rfactor).as_str());
                },
                None => {
                    result.push_str(generate_term(&*lterm).as_str());
                },
            }
        },
        None => {
            match term.left_factor.clone() {
                    Some(lfactor) => {
                        match term.right_factor.clone() {
                            Some(rfactor) => {
                                result.push_str(generate_factor(&*lfactor).as_str());
                                result.push_str(generate_term_rfactor(term, &*rfactor).as_str());
                            },
                            None => {
                                result.push_str(generate_factor(&*lfactor).as_str());
                            },
                        }
                    },
                    None => {
                    },
                }               
        },
    }
    result
}

fn generate_factor(factor : &parser::Factor) -> String {
    let mut result = String::new();
    match factor.unary.clone() {
        Some(un) => {
            result.push_str(generate_unary(&*un).as_str());
        },
        None => {
            match factor.exp.clone() {
                Some(e) => {
                    result.push_str(generate_or_expr(&*e).as_str());
                },
                None => {
                    match factor.val {
                        Some(v) => {
                            result.push_str("    movl    $");
                            result.push_str((v).to_string().as_str());
                            result.push_str(", %eax\n");
                        },
                        None => {
                        },
                    }
                },
            }
        }
    }
    result
}


fn generate_unary(un : &parser::Unary) -> String {
    let mut result = String::new();
    match un.right_fact.clone() {
        Some(fact) => {
            result.push_str(generate_factor(&*fact).as_str());
            match un.op.as_str(){
                "!" => {
                    // MOVE TO EXTERNAL FUNCS LATER
                    result.push_str("    cmpl    $0, %eax\n");
                    result.push_str("    movl    $0, %eax\n");
                    result.push_str("    sete    %al\n");
                },
                "~" => {
                    result.push_str("    not     %eax\n");
                },
                "-" => {
                    result.push_str("    neg     %eax\n");
                },
                _ => {
                    println!("Found an unwritten unary: {}", un.op.as_str());
                    std::process::exit(1);
                },
            }
        },
        None => {
        },
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
