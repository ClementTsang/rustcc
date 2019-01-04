#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

/**
 * To do:
 * Format resulting .s files better (Spacing is WHACK)
**/

pub mod parser;
use std::env;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;


fn print_assembly(input : &String){
   println!("=====Resulting assembly=====\n{}", input);
   println!("=====End assembly=====");
}

fn generate_function(func : &parser::Function) -> String {
    let mut result : String = String::from(func.name.clone().as_str());
    result.push_str(":\n");

    result.push_str("    pushl   %ebp # Opening function\n");
    result.push_str("    movl    %esp, %ebp\n");

    let mut var_map : HashMap<String, i32> = HashMap::new();
    let mut stack_index : i32 = 0;

    let mut is_return_there = false;

    for st in &func.list_of_st {
        if (st.name == "return") {
            is_return_there = true;
        }
        result.push_str(generate_statement(&st, &mut var_map, &mut stack_index).as_str());
    }
    
    if (!is_return_there) {
        result.push_str("    movl    $0, %eax # Default return value\n");
    }

    result.push_str("    movl    %ebp, %esp # Close function\n");
    result.push_str("    popl    %ebp\n");
    result.push_str("    ret\n");

    result
}

fn generate_statement(st : &parser::Statement, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();
    match st.exp.clone() {
        Some(x) => {
            result.push_str(generate_assignment(&x, var_map, stack_index).as_str());
        },
        None => {
            match st.decl.clone() {
                Some (x) => {
                    result.push_str(generate_declaration(&x, var_map, stack_index).as_str());
                },
                None => (),
            }
        },
    }
    result
}

fn generate_declaration(decl : &parser::Declaration, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();

    // Check to see if the variable has been declared already.
    assert!(!var_map.contains_key(&(decl.var.var_name.clone())), "Duplicate variable declaration found for {}.", decl.var.var_name.clone());

    // Generate value to assign to variable
    result.push_str(generate_assignment(&decl.exp, var_map, stack_index).as_str());
    result.push_str("    pushl   %eax\n");

    // Push new variable to hash map, decrement stack index.
    *stack_index -= 4;
    var_map.insert(decl.var.clone().var_name, stack_index.clone());

    result
}

fn generate_assignment(assign_exp : &parser::Assignment, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();

    // Generate expression value
    match assign_exp.assign.clone() {
        Some(a) => {
            result.push_str(generate_assignment(&*a, var_map, stack_index).as_str());
        },
        None => {
            match assign_exp.exp.clone() {
                Some(exp) => {
                    result.push_str(generate_or_expr(&exp, var_map, stack_index).as_str());
                },
                None => ()
            }
        },
    }

    match assign_exp.var.clone() {
        Some(var) => {
            // Assign new value to variable IF it exists.
            assert!(var_map.contains_key(&(var.var_name.clone())), "Variable declaration not found when trying to assign.");
            let var_offset = var_map.get(&(var.var_name.clone()));
            match var_offset {
                Some (offset) => { 
                    result.push_str(format!("    movl    %eax, {}(%ebp) # Assigning new value\n", offset).as_str());
                },
                None => (),
            }
        },
        None => (),
    }

    result
}

fn generate_or_rchild(expr : &parser::OrExpression, rchild : &parser::AndExpression, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();
    result.push_str("    pushl   %eax # Generating ||\n");
    result.push_str(generate_and_expr(&*rchild, var_map, stack_index).as_str());
    result.push_str("    popl    %ecx\n");
    result.push_str("    orl     %ecx, %eax\n");
    result.push_str("    movl    $0, %eax\n");
    result.push_str("    setne   %al # End ||\n");

    result
}

fn generate_or_expr(exp : &parser::OrExpression, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_and_exp.clone() {
                Some(rchild) => {
                    result.push_str(generate_or_expr(&*lexp, var_map, stack_index).as_str());
                    result.push_str(generate_or_rchild(exp, &*rchild, var_map, stack_index).as_str());
                },
                None => {
                    result.push_str(generate_or_expr(&*lexp, var_map, stack_index).as_str());
                },
            }
        },
        None => {
            match exp.left_and_exp.clone() {
                    Some(lchild) => {
                        match exp.right_and_exp.clone() {
                            Some(rchild) => {
                                result.push_str(generate_and_expr(&*lchild, var_map, stack_index).as_str());
                                result.push_str(generate_or_rchild(exp, &*rchild, var_map, stack_index).as_str());
                            },
                            None => {
                                result.push_str(generate_and_expr(&*lchild, var_map, stack_index).as_str());
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

fn generate_and_rchild(expr : &parser::AndExpression, rchild : &parser::BitOr, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();

    result.push_str("    pushl   %eax # Generating &&\n");
    result.push_str(generate_bit_or(rchild, var_map, stack_index).as_str());
    result.push_str("    popl    %ecx\n");
    result.push_str("    cmpl    $0, %ecx\n");
    result.push_str("    setne   %cl\n");
    result.push_str("    cmpl    $0, %eax\n");
    result.push_str("    movl    $0, %eax\n");
    result.push_str("    setne   %al\n");
    result.push_str("    andb    %cl, %al # End &&\n");

    result
}

fn generate_and_expr(exp : &parser::AndExpression, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_child.clone() {
                Some(rchild) => {
                    result.push_str(generate_and_expr(&*lexp, var_map, stack_index).as_str());
                    result.push_str(generate_and_rchild(exp, &*rchild, var_map, stack_index).as_str());
                },
                None => {
                    result.push_str(generate_and_expr(&*lexp, var_map, stack_index).as_str());
                },
            }
        },
        None => {
            match exp.left_child.clone() {
                    Some(lchild) => {
                        match exp.right_child.clone() {
                            Some(rchild) => {
                                result.push_str(generate_bit_or(&*lchild, var_map, stack_index).as_str());
                                result.push_str(generate_and_rchild(exp, &*rchild, var_map, stack_index).as_str());
                            },
                            None => {
                                result.push_str(generate_bit_or(&*lchild, var_map, stack_index).as_str());
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

fn generate_bit_or_rchild(expr : &parser::BitOr, rchild : &parser::BitXor, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();

    result.push_str("    pushl   %eax # Generating |\n");
    result.push_str(generate_bit_xor(rchild, var_map, stack_index).as_str());
    result.push_str("    popl    %ecx\n");
    result.push_str("    orl     %ecx, %eax # End |\n");

    result
}

fn generate_bit_or(exp : &parser::BitOr, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_child.clone() {
                Some(rchild) => {
                    result.push_str(generate_bit_or(&*lexp, var_map, stack_index).as_str());
                    result.push_str(generate_bit_or_rchild(exp, &*rchild, var_map, stack_index).as_str());
                },
                None => {
                    result.push_str(generate_bit_or(&*lexp, var_map, stack_index).as_str());
                },
            }
        },
        None => {
            match exp.left_child.clone() {
                    Some(lchild) => {
                        match exp.right_child.clone() {
                            Some(rchild) => {
                                result.push_str(generate_bit_xor(&*lchild, var_map, stack_index).as_str());
                                result.push_str(generate_bit_or_rchild(exp, &*rchild, var_map, stack_index).as_str());
                            },
                            None => {
                                result.push_str(generate_bit_xor(&*lchild, var_map, stack_index).as_str());
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

fn generate_bit_xor_rchild(expr : &parser::BitXor, rchild : &parser::BitAnd, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();

    result.push_str("    pushl   %eax # Generating ^\n");
    result.push_str(generate_bit_and(rchild, var_map, stack_index).as_str());
    result.push_str("    popl    %ecx\n");
    result.push_str("    xorl    %ecx, %eax # End ^\n");

    result
}

fn generate_bit_xor(exp : &parser::BitXor, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_child.clone() {
                Some(rchild) => {
                    result.push_str(generate_bit_xor(&*lexp, var_map, stack_index).as_str());
                    result.push_str(generate_bit_xor_rchild(exp, &*rchild, var_map, stack_index).as_str());
                },
                None => {
                    result.push_str(generate_bit_xor(&*lexp, var_map, stack_index).as_str());
                },
            }
        },
        None => {
            match exp.left_child.clone() {
                    Some(lchild) => {
                        match exp.right_child.clone() {
                            Some(rchild) => {
                                result.push_str(generate_bit_and(&*lchild, var_map, stack_index).as_str());
                                result.push_str(generate_bit_xor_rchild(exp, &*rchild, var_map, stack_index).as_str());
                            },
                            None => {
                                result.push_str(generate_bit_and(&*lchild, var_map, stack_index).as_str());
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

fn generate_bit_and_rchild(expr : &parser::BitAnd, rchild : &parser::EqualityExp, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();

    result.push_str("    pushl   %eax # Generating &\n");
    result.push_str(generate_eq_expr(rchild, var_map, stack_index).as_str());
    result.push_str("    popl    %ecx\n");
    result.push_str("    andl     %ecx, %eax # End &\n");

    result
}

fn generate_bit_and(exp : &parser::BitAnd, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_child.clone() {
                Some(rchild) => {
                    result.push_str(generate_bit_and(&*lexp, var_map, stack_index).as_str());
                    result.push_str(generate_bit_and_rchild(exp, &*rchild, var_map, stack_index).as_str());
                },
                None => {
                    result.push_str(generate_bit_and(&*lexp, var_map, stack_index).as_str());
                },
            }
        },
        None => {
            match exp.left_child.clone() {
                    Some(lchild) => {
                        match exp.right_child.clone() {
                            Some(rchild) => {
                                result.push_str(generate_eq_expr(&*lchild, var_map, stack_index).as_str());
                                result.push_str(generate_bit_and_rchild(exp, &*rchild, var_map, stack_index).as_str());
                            },
                            None => {
                                result.push_str(generate_eq_expr(&*lchild, var_map, stack_index).as_str());
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

fn generate_eq_rchild(expr : &parser::EqualityExp, rchild : &parser::RelationalExp, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();

    result.push_str("    pushl    %eax # Generating eq\n");
    result.push_str(generate_rel_expr(&*rchild, var_map, stack_index).as_str());
    result.push_str("    popl     %ecx\n");
    result.push_str("    cmpl     %eax, %ecx\n");
    result.push_str("    movl     %ecx, %eax\n");

    match expr.op.as_str() {
        "==" => {
            result.push_str("    sete     %al # End ==\n");
        },
        "!=" => {
            result.push_str("    setne    %al # End !=\n");
        },
        _ => {
            println!("Found an unwritten binary(Expr): {}", expr.op.as_str());
            std::process::exit(1);
        },
    }

    result
}

fn generate_eq_expr(exp : &parser::EqualityExp, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_relation_exp.clone() {
                Some(rchild) => {
                    result.push_str(generate_eq_expr(&*lexp, var_map, stack_index).as_str());
                    result.push_str(generate_eq_rchild(exp, &*rchild, var_map, stack_index).as_str());
                },
                None => {
                    result.push_str(generate_eq_expr(&*lexp, var_map, stack_index).as_str());
                },
            }
        },
        None => {
            match exp.left_relation_exp.clone() {
                    Some(lchild) => {
                        match exp.right_relation_exp.clone() {
                            Some(rchild) => {
                                result.push_str(generate_rel_expr(&*lchild, var_map, stack_index).as_str());
                                result.push_str(generate_eq_rchild(exp, &*rchild, var_map, stack_index).as_str());
                            },
                            None => {
                                result.push_str(generate_rel_expr(&*lchild, var_map, stack_index).as_str());
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

fn generate_rel_rchild(expr : &parser::RelationalExp, rchild : &parser::BitShift, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();

    result.push_str(format!("    pushl    %eax # Generating rel: {}\n", expr.op.as_str()).as_str());
    result.push_str(generate_bit_shift(&*rchild, var_map, stack_index).as_str());
    result.push_str("    popl     %ecx\n");
    result.push_str("    cmpl     %eax, %ecx\n");
    result.push_str("    movl     %ecx, %eax\n");

    match expr.op.as_str() {
        "<" => {
            result.push_str("    setl     %al # End <\n");
        },
        ">" => {
            result.push_str("    setg     %al # End >\n");
        },
        "<=" => {
            result.push_str("    setle    %al # End <=\n");
        },
        ">=" => {
            result.push_str("    setge    %al # End >=\n");
        },
        _ => {
            println!("Found an unwritten binary(Expr): {}", expr.op.as_str());
            std::process::exit(1);
        },
    }

    result
}

fn generate_rel_expr(exp : &parser::RelationalExp, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_child.clone() {
                Some(rchild) => {
                    result.push_str(generate_rel_expr(&*lexp, var_map, stack_index).as_str());
                    result.push_str(generate_rel_rchild(exp, &*rchild, var_map, stack_index).as_str());
                },
                None => {
                    result.push_str(generate_rel_expr(&*lexp, var_map, stack_index).as_str());
                },
            }
        },
        None => {
            match exp.left_child.clone() {
                    Some(lchild) => {
                        match exp.right_child.clone() {
                            Some(rchild) => {
                                result.push_str(generate_bit_shift(&*lchild, var_map, stack_index).as_str());
                                result.push_str(generate_rel_rchild(exp, &*rchild, var_map, stack_index).as_str());
                            },
                            None => {
                                result.push_str(generate_bit_shift(&*lchild, var_map, stack_index).as_str());
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

fn generate_bit_shift_rchild(expr : &parser::BitShift, rchild : &parser::AdditiveExp, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();

    result.push_str(format!("    pushl   %eax # Generating rel: {}\n", expr.op.as_str()).as_str());
    result.push_str(generate_add_expr(&*rchild, var_map, stack_index).as_str());
    result.push_str("    movl    %eax, %ecx\n");
    result.push_str("    popl    %eax\n");

    match expr.op.as_str() {
        "<<" => {
            result.push_str("    sall    %cl, %eax # End <<\n");
        },
        ">>" => {
            result.push_str("    sarl    %cl, %eax # End >>\n");
        },
        _ => {
            println!("Found an unwritten binary(BitShift): {}", expr.op.as_str());
            std::process::exit(1);
        },
    }
    

    result
}

fn generate_bit_shift(exp : &parser::BitShift, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_child.clone() {
                Some(rchild) => {
                    result.push_str(generate_bit_shift(&*lexp, var_map, stack_index).as_str());
                    result.push_str(generate_bit_shift_rchild(exp, &*rchild, var_map, stack_index).as_str());
                },
                None => {
                    result.push_str(generate_bit_shift(&*lexp, var_map, stack_index).as_str());
                },
            }
        },
        None => {
            match exp.left_child.clone() {
                    Some(lchild) => {
                        match exp.right_child.clone() {
                            Some(rchild) => {
                                result.push_str(generate_add_expr(&*lchild, var_map, stack_index).as_str());
                                result.push_str(generate_bit_shift_rchild(exp, &*rchild, var_map, stack_index).as_str());
                            },
                            None => {
                                result.push_str(generate_add_expr(&*lchild, var_map, stack_index).as_str());
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

fn generate_add_expr_rterm(expr : &parser::AdditiveExp, rterm : &parser::Term, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();
    match expr.op.as_str() {
        "-" => {
            result.push_str("    pushl   %eax # Generating binary (-)\n");
            result.push_str(generate_term(&*rterm, var_map, stack_index).as_str());
            result.push_str("    pushl   %eax\n");
            result.push_str("    popl    %ecx\n");
            result.push_str("    popl    %eax\n");
            result.push_str("    subl    %ecx, %eax # End -\n");
        },
        "+" => {
            result.push_str("    pushl   %eax # Generating binary (+)\n");
            result.push_str(generate_term(&*rterm, var_map, stack_index).as_str());
            result.push_str("    popl    %ecx\n");
            result.push_str("    addl    %ecx, %eax # End +\n");

        },
        _ => {
            println!("Found an unwritten binary(Expr): {}", expr.op.as_str());
            std::process::exit(1);
        },
    }

    result
}

fn generate_add_expr(exp : &parser::AdditiveExp, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_term.clone() {
                Some(rterm) => {
                    result.push_str(generate_add_expr(&*lexp, var_map, stack_index).as_str());
                    result.push_str(generate_add_expr_rterm(exp, &*rterm, var_map, stack_index).as_str());
                },
                None => {
                    result.push_str(generate_add_expr(&*lexp, var_map, stack_index).as_str());
                },
            }
        },
        None => {
            match exp.left_term.clone() {
                    Some(lterm) => {
                        match exp.right_term.clone() {
                            Some(rterm) => {
                                result.push_str(generate_term(&*lterm, var_map, stack_index).as_str());
                                result.push_str(generate_add_expr_rterm(exp, &*rterm, var_map, stack_index).as_str());
                            },
                            None => {
                                result.push_str(generate_term(&*lterm, var_map, stack_index).as_str());
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

fn generate_term_rfactor(term : &parser::Term, rfactor : &parser::Factor, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();
    match term.op.as_str() {
        "*" => {
            result.push_str("    pushl  %eax # Generating binary (*)\n");
            result.push_str(generate_factor(&*rfactor, var_map, stack_index).as_str());
            result.push_str("    popl   %ecx\n");
            result.push_str("    imul   %ecx, %eax # End *\n");
        },
        "/" => {
            result.push_str("    pushl  %eax # Generating binary (/)\n");
            result.push_str(generate_factor(&*rfactor, var_map, stack_index).as_str());
            result.push_str("    pushl  %eax\n");
            result.push_str("    popl   %ecx\n");
            result.push_str("    popl   %eax\n");
            result.push_str("    movl   $0, %edx\n");  //Zero out edx
            result.push_str("    idivl  %ecx # End /\n"); //ecx is divisor

        },
        "%" => {
            result.push_str("    pushl  %eax # Generating binary (%)\n");
            result.push_str(generate_factor(&*rfactor, var_map, stack_index).as_str());
            result.push_str("    pushl  %eax\n");
            result.push_str("    popl   %ecx\n");
            result.push_str("    popl   %eax\n");
            result.push_str("    movl   $0, %edx\n");  //Zero out edx
            result.push_str("    idivl  %ecx # End %\n");
            result.push_str("    movl   %edx, %eax # End %\n"); //Move remainder to eax
        },
        _ => {
            println!("Found an unwritten Binary(Term): {}", term.op.as_str());
            std::process::exit(1);
        },
    }

    result
}

fn generate_term(term : &parser::Term, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();
    match term.left_term.clone() {
        Some(lterm) => {
            match term.right_child.clone() {
                Some(rfactor) => {
                    result.push_str(generate_term(&*lterm, var_map, stack_index).as_str());
                    result.push_str(generate_term_rfactor(term, &*rfactor, var_map, stack_index).as_str());
                },
                None => {
                    result.push_str(generate_term(&*lterm, var_map, stack_index).as_str());
                },
            }
        },
        None => {
            match term.left_child.clone() {
                    Some(lfactor) => {
                        match term.right_child.clone() {
                            Some(rfactor) => {
                                result.push_str(generate_factor(&*lfactor, var_map, stack_index).as_str());
                                result.push_str(generate_term_rfactor(term, &*rfactor, var_map, stack_index).as_str());
                            },
                            None => {
                                result.push_str(generate_factor(&*lfactor, var_map, stack_index).as_str());
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

fn generate_factor(factor : &parser::Factor, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();
    match factor.postfix_unary.clone() {
        Some (pf_un) => {
            result.push_str(generate_postfix_unary(&*pf_un, var_map, stack_index).as_str());
        },
        None => {
            match factor.unary.clone() {
                Some(un) => {
                    result.push_str(generate_unary(&*un, var_map, stack_index).as_str());
                },
                None => {
                    match factor.exp.clone() {
                        Some(e) => {
                            result.push_str(generate_assignment(&*e, var_map, stack_index).as_str());
                        },
                        None => {
                            match factor.val {
                                Some(v) => {
                                    result.push_str("    movl    $");
                                    result.push_str((v).to_string().as_str());
                                    result.push_str(", %eax # Constant integer reference\n");
                                },
                                None => {
                                    match factor.var.clone() {
                                        Some(va) => {
                                            // Assign new value to variable IF it exists.
                                            assert!(var_map.contains_key(&(va.var_name.clone())), "Variable declaration not found when referencing.");
                                            let var_offset = var_map.get(&(va.var_name.clone()));
                                            match var_offset {
                                                Some (offset) => { 
                                                    result.push_str(format!("    movl    {}(%ebp), %eax # Variable reference\n", offset).as_str());
                                                },
                                                None => (),
                                            }
                                        },
                                        None => (),
                                    }
                                },
                            }
                        },
                    }
                }
            }
        },
    }
    result
}


fn generate_unary(un : &parser::Unary, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();
    match un.right_child.clone() {
        Some(fact) => {
            result.push_str(generate_factor(&*fact, var_map, stack_index).as_str());
            match un.op.as_str(){
                "!" => {
                    // MOVE TO EXTERNAL FUNCS LATER
                    result.push_str("    cmpl    $0, %eax # Generating !\n");
                    result.push_str("    movl    $0, %eax\n");
                    result.push_str("    sete    %al\n");
                },
                "~" => {
                    result.push_str("    not     %eax # Generating ~\n");
                },
                "-" => {
                    result.push_str("    neg     %eax # Generating -\n");
                },
                "+" => {
                    // Don't have to do anything, as we basically just find %eax and
                    // do nothing with it.
                }
                "++" => {

                },
                "--" => {

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

fn generate_postfix_unary(un : &parser::PostFixUnary, var_map : &mut HashMap<String, i32>, stack_index : &mut i32) -> String {
    let mut result = String::new();
    match un.right_fact.clone() {
        Some(fact) => {
            result.push_str(generate_factor(&*fact, var_map, stack_index).as_str());
            match un.op.as_str(){
                "++" => {

                },
                "--" => {

                },
                _ => {
                    println!("Found an unwritten postfix unary: {}", un.op.as_str());
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
