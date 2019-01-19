#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(unused_assignments)]

/**
 * To do:
 * Format resulting .s files better (Spacing is WHACK)
**/

pub mod parser;
use std::env;
use std::fs::File;
use std::io::Write;
use std::collections::HashMap;

static IF_FN : &str = "if_fn";
static ELSE_FN : &str = "else_fn";
static LOOP : &str = "loop";
static POST_LOOP : &str = "post_loop";
static CONTINUE : &str = "continue";

fn print_assembly(input : &String){
   println!("=====Resulting assembly=====\n{}", input);
   println!("=====End assembly=====");
}

fn exit_program() {
    std::process::exit(1);
}

fn generate_function(func : &parser::Function, list_of_func : &Vec<parser::Function>) -> String {
    let mut similar_count = 0;
    for chk in list_of_func.clone() {
        let counter = if (chk.params.len() > func.params.len()) {
            func.params.len()
        }
        else {
            chk.params.len()
        };

        for i in 0..counter {
            match chk.params.get(i).clone() {
                Some (x) => {
                    match func.params.get(i).clone() {
                        Some (y) => {
                            if (x.param_type != y.param_type) {
                                break;
                            }
                            else if (i == counter - 1){
                                if (chk.is_definition == true || chk.params.len() != func.params.len()) {
                                    similar_count += 1;
                                }                         
                            }
                        },
                        None => (),
                    }
                },
                None => (),
            }
        }
    }
    assert!(similar_count <= 1, "Found a function that is too similar.");

    let mut result : String = String::from(func.name.clone().as_str());
    result.push_str(":\n");

    result.push_str("    pushl    %ebp # Set up stack frame\n");
    result.push_str("    movl     %esp, %ebp\n");

    let mut var_map : HashMap<String, i32> = HashMap::new();
    let mut stack_index : i32 = 8;
    let mut fn_index : i32 = 0;  // I *should* use two numbers but quite frankly I don't think it matters...
    let mut cur_map : HashMap<String, i32> = HashMap::new();  // Each hashmap will be linked to its name as key and value as the assembly code

    for param in func.params.clone() {
        //Push new variable to hash map, decrement stack index.
        var_map.insert(param.name.clone(), stack_index.clone());
        cur_map.insert(param.name.clone(), stack_index.clone());
        stack_index += 4;
    }
    stack_index = 0;

    for blk in &func.list_of_blk {
        match blk.state.clone() {
            Some (x) => {
                let mut fake_var_map : HashMap<String, i32> = var_map.clone();
                let mut fake_stack_index : i32 = stack_index;
                result.push_str(generate_statement(&x, &mut fake_var_map, &mut fake_stack_index, &mut fn_index, &mut cur_map, &mut String::from(""), &mut String::from(""), &list_of_func).as_str());
            },
            None => {
                match blk.decl.clone() {
                    Some (y) => {
                        result.push_str(generate_declaration(&y, &mut var_map, &mut stack_index, &mut fn_index, &mut cur_map, &mut String::from(""), &mut String::from(""), &list_of_func).as_str());
                    },
                    None => (),
                }
            },
        }
    }

    result.push_str("    movl     $0, %eax # Default return value\n");
    result.push_str("    movl     %ebp, %esp # Deallocate any local variables on stack\n");
    result.push_str("    popl     %ebp\n");
    result.push_str("    ret\n");

    result
}


fn generate_fn_call(fn_call : &parser::FnCall, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();

    let mut exists = false;
    for chk in list_of_func.clone() {
        if (chk.name == fn_call.name && chk.params.len() == fn_call.args.len()) {
            exists = true;
        }
    }
    assert!(exists, "Incorrect number of params.");

    for arg in fn_call.clone().args.iter().rev() {
        result.push_str(generate_assignment(&arg, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
        result.push_str("    pushl    %eax\n");        
    }

    result.push_str(format!("    call     {}\n", fn_call.name).as_str());
    result.push_str(format!("    addl     ${}, %esp\n", fn_call.args.len() * 4).as_str());

    result
}


fn generate_compound(cmp : &parser::Compound, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
    let mut new_cur_map : HashMap<String, i32> = HashMap::new();

    for blk in &cmp.list_of_blk {
        match blk.state.clone() {
            Some (x) => {
                let mut fake_var_map : HashMap<String, i32> = var_map.clone();
                let mut fake_stack_index : i32 = *stack_index;
                result.push_str(generate_statement(&x, &mut fake_var_map, &mut fake_stack_index, fn_index, &mut new_cur_map, loop_start, loop_post, list_of_func).as_str());
            },
            None => {
                match blk.decl.clone() {
                    Some (y) => {
                        result.push_str(generate_declaration(&y, var_map, stack_index, fn_index, &mut new_cur_map, loop_start, loop_post, list_of_func).as_str());
                    },
                    None => (),
                }
            },
        }
    }
    
    result.push_str(format!("    addl     ${}, %esp # Deallocate bytes\n", new_cur_map.len() * 4).as_str());

    result
}


fn generate_while(loop_type : &parser::While, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();

    *fn_index += 1;
    let while_index = *fn_index;

    *fn_index += 1;
    let continue_index = *fn_index;

    *fn_index += 1;
    let after_index = *fn_index;

    result.push_str(format!("\n{}{}:\n", LOOP, while_index).as_str());
    result.push_str(generate_assignment(&loop_type.exp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
    result.push_str("    cmpl     $0, %eax\n");

    result.push_str(format!("    je       {}{}\n", POST_LOOP, after_index).as_str());

    *loop_post = format!("{}{}", POST_LOOP, after_index);
    *loop_start = format!("{}{}", CONTINUE, continue_index);
    result.push_str(generate_statement(&*loop_type.statement, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());


    result.push_str(format!("\n{}{}:\n", CONTINUE, continue_index).as_str());
    
    result.push_str(format!("    jmp      {}{}\n", LOOP, while_index).as_str());

    result.push_str(format!("\n{}{}:\n", POST_LOOP, after_index).as_str());

    result
}


fn generate_for(loop_type : &parser::For, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();

    match (loop_type.optional_exp_1.clone()) {
        Some (x) => result.push_str(generate_assignment(&x, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str()),
        None => (),
    }

    *fn_index += 1;
    let for_index = *fn_index;
    result.push_str(format!("\n{}{}:\n", LOOP, for_index).as_str());

    *fn_index += 1;
    let continue_index = *fn_index;
    
    result.push_str(generate_assignment(&loop_type.exp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
    result.push_str("    cmpl     $0, %eax\n");

    *fn_index += 1;
    let after_index = *fn_index;
    result.push_str(format!("    je       {}{}\n", POST_LOOP, after_index).as_str());
    *loop_post = format!("{}{}", POST_LOOP, after_index);
    *loop_start = format!("{}{}", CONTINUE, continue_index);
    result.push_str(generate_statement(&loop_type.statement, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
    
    result.push_str(format!("\n{}{}:\n", CONTINUE, continue_index).as_str());
    
    match (loop_type.optional_exp_2.clone()) {
        Some (x) => result.push_str(generate_assignment(&x, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str()),
        None => (),
    }
    result.push_str(format!("    jmp      {}{}\n", LOOP, for_index).as_str());
    result.push_str(format!("\n{}{}:\n", POST_LOOP, after_index).as_str());

    result
}


fn generate_for_decl(loop_type : &parser::ForDecl, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
    let mut new_cur_map : HashMap<String, i32> = HashMap::new();

    result.push_str(generate_declaration(&loop_type.decl, var_map, stack_index, fn_index, &mut new_cur_map, loop_start, loop_post, list_of_func).as_str()); 

    *fn_index += 1;
    let for_index = *fn_index;
    result.push_str(format!("\n{}{}:\n", LOOP, for_index).as_str());

    *fn_index += 1;
    let continue_index = *fn_index;
    
    result.push_str(generate_assignment(&loop_type.exp, var_map, stack_index, fn_index, &mut new_cur_map, loop_start, loop_post, list_of_func).as_str());
    result.push_str("    cmpl     $0, %eax\n");

    *fn_index += 1;
    let after_index = *fn_index;
    result.push_str(format!("    je       {}{}\n", POST_LOOP, after_index).as_str());
    *loop_post = format!("{}{}", POST_LOOP, after_index);
    *loop_start = format!("{}{}", CONTINUE, continue_index);
    result.push_str(generate_statement(&loop_type.statement, var_map, stack_index, fn_index, &mut new_cur_map, loop_start, loop_post, list_of_func).as_str());

    result.push_str(format!("\n{}{}:\n", CONTINUE, continue_index).as_str());

    match (loop_type.optional_exp_2.clone()) {
        Some (x) => result.push_str(generate_assignment(&x, var_map, stack_index, fn_index, &mut new_cur_map, loop_start, loop_post, list_of_func).as_str()),
        None => (),
    }
    result.push_str(format!("    jmp      {}{}\n", LOOP, for_index).as_str());
    result.push_str(format!("\n{}{}:\n", POST_LOOP, after_index).as_str());
    result.push_str(format!("    addl     ${}, %esp # Deallocate bytes\n", new_cur_map.len() * 4).as_str());

    result
}


fn generate_do(loop_type : &parser::DoWhile, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();

    *fn_index += 1;
    let do_index = *fn_index;
    result.push_str(format!("\n{}{}:\n", LOOP, do_index).as_str());

    *fn_index += 1;
    let continue_index = *fn_index;

    *fn_index += 1;
    let after_index = *fn_index;

    *loop_post = format!("{}{}", POST_LOOP, after_index);
    *loop_start = format!("{}{}", CONTINUE, continue_index);

    result.push_str(generate_statement(&*loop_type.statement, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());

    result.push_str(format!("\n{}{}:\n", CONTINUE, continue_index).as_str());

    result.push_str(generate_assignment(&loop_type.exp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
    result.push_str("    cmpl     $0, %eax\n");
    result.push_str(format!("    jne      {}{}\n", LOOP, do_index).as_str());

    result.push_str(format!("\n{}{}:\n", POST_LOOP, after_index).as_str());

    result
}


fn generate_break(break_exp : &parser::Break, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
    
    assert!((loop_post != ""), "Empty loop, break in invalid area.");
    result.push_str(format!("    jmp      {} # break\n", loop_post).as_str());

    result
}


fn generate_continue(continue_exp : &parser::Continue, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
    
    assert!((loop_start != ""), "Empty loop, break in invalid area.");
    result.push_str(format!("    jmp      {} # continue\n", loop_start).as_str());

    result
}


fn generate_statement(st : &parser::Statement, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();

    let old_post : String = loop_post.clone();
    let old_start : String = loop_start.clone(); 

    if (st.name == "empty") {
        return result;
    }

    match st.exp.clone() {
        Some(x) => {
            result.push_str(generate_assignment(&x, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
            if (st.name == "return") {
                result.push_str("    movl     %ebp, %esp # Close function\n");
                result.push_str("    popl     %ebp\n");
                result.push_str("    ret\n");
            }
        },
        None => (),
    }

    match st._if.clone() {
        Some (x) => {
            result.push_str(generate_if(&x, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
        },
        None => (),
    }

    match st.compound.clone() {
        Some (x) => {
            result.push_str(generate_compound(&x, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
        }
        None => (),
    }

    // Loops
    match st._while.clone() {
        Some (y) => result.push_str(generate_while(&y, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str()),
        None => (),
    }
    match st._for.clone() {
        Some (y) => result.push_str(generate_for(&y, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str()),
        None => (),
    }
    match st._for_decl.clone() {
        Some (y) => result.push_str(generate_for_decl(&y, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str()),
        None => (),
    }
    match st._do.clone() {
        Some (y) => result.push_str(generate_do(&y, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str()),
        None => (),
    }
    match st._continue.clone() {
        Some (y) => result.push_str(generate_continue(&y, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str()),
        None => (),
    }
    match st._break.clone() {
        Some (y) => result.push_str(generate_break(&y, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str()),
        None => (),
    }

    *loop_post = old_post;
    *loop_start = old_start;
    
    result
}

fn generate_if(if_exp : &parser::If, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();

    // Cond
    result.push_str(generate_assignment(&if_exp.cond, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
    
    match if_exp.state.clone() {
        Some (x) => {
            result.push_str("    cmpl     $0, %eax\n");
            *fn_index += 1;
            let if_index = *fn_index;
            result.push_str(format!("    je       {}{} # Jump to else condition\n", IF_FN, fn_index).as_str());
            result.push_str(generate_statement(&*x, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());

            match if_exp.else_state.clone() {
                Some (y) => {
                    *fn_index += 1; 
                    let else_index = *fn_index;
                    result.push_str(format!("    jmp      {}{}\n", ELSE_FN, else_index).as_str());
                    result.push_str(format!("\n{}{}:\n", IF_FN, if_index).as_str());                        
                    result.push_str(generate_statement(&*y, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                    result.push_str(format!("\n{}{}:\n", ELSE_FN, else_index).as_str());
                },
                None => {
                    result.push_str(format!("\n{}{}:\n", IF_FN, if_index).as_str());
                },
            }
        },
        None => (),
    }

    result
}

fn generate_declaration(decl : &parser::Declaration, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();

    // Check to see if the variable has been declared already.
    assert!(!cur_map.contains_key(&(decl.var.var_name.clone())), "Duplicate variable declaration found for {}.", decl.var.var_name.clone());
    //assert!(!var_map.contains_key(&(decl.var.var_name.clone())), "Duplicate variable declaration found for {}.", decl.var.var_name.clone());


    // Generate value to assign to variable
    result.push_str(generate_assignment(&decl.exp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
    result.push_str("    pushl    %eax\n");

    // Push new variable to hash map, decrement stack index.
    *stack_index -= 4;
    var_map.insert(decl.var.clone().var_name, stack_index.clone());
    cur_map.insert(decl.var.clone().var_name, stack_index.clone());

    result
}

fn generate_assignment(assign_exp : &parser::Assignment, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();

    // Generate expression value
    match assign_exp.assign.clone() {
        Some(a) => {
            result.push_str(generate_assignment(&*a, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
        },
        None => {
            match assign_exp.exp.clone() {
                Some(exp) => {
                    result.push_str(generate_cond_exp(&exp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
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
                    if (assign_exp.op.as_str() != "=") {
                        result.push_str("    pushl     %eax # Begin generating assignment operators\n");
                        result.push_str("    popl      %ecx\n");
                        result.push_str(format!("    movl     {}(%ebp), %eax\n", offset).as_str());
                    }
                    match assign_exp.op.as_str() {
                        "=" => {
                            // Don't have to do anything, below statement is default behaviour.
                        },
                        "+=" => {
                            result.push_str("    addl     %ecx, %eax # +=\n"); 
                        },
                        "-=" => {
                            result.push_str("    subl     %ecx, %eax # -=\n");
                        },
                        "*=" => {
                            result.push_str("    imul     %ecx, %eax # *=\n");
                        },
                        "/=" => {
                            result.push_str("    movl     $0, %edx # /=\n");
                            result.push_str("    idivl    %ecx\n"); 
                        },
                        "^=" => {
                            result.push_str("    xorl     %ecx, %eax # ^=\n");
                        },
                        "&=" => {
                            result.push_str("    andl     %ecx, %eax # &=\n");
                        },
                        "|=" => {
                            result.push_str("    orl      %ecx, %eax # |=\n"); 
                        },
                        "%=" => {
                            result.push_str("    movl     $0, %edx # %=\n");
                            result.push_str("    idivl    %ecx\n"); 
                            result.push_str("    movl     %edx, %eax\n"); //Move remainder to eax
                        },
                        ">>=" => {
                            result.push_str("    sarl     %cl, %eax # >>=\n");
                        },
                        "<<=" => {
                            result.push_str("    sall     %cl, %eax # <<=\n");
                        },
                        _ => {
                            println!("Found an invalid assignment operation.");
                            exit_program();
                        }
                    }
                    result.push_str(format!("    movl     %eax, {}(%ebp) # Assigning new value\n", offset).as_str());
                },
                None => (),
            }
        },
        None => (),
    }

    result
}

fn generate_cond_exp(cond_exp : &parser::ConditionalExp, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();

    result.push_str(generate_or_expr(&cond_exp.exp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());

    match cond_exp.true_exp.clone() {
        Some(x) => {
            match cond_exp.false_exp.clone() {
                Some(y) => {
                    result.push_str("    cmpl     $0, %eax # Begin cond \n");

                    // Case where %eax == 0, so %eax was false.  Thus, we execute the "false" part of the ternary operator
                    *fn_index += 1;
                    let false_index = *fn_index;
                    result.push_str(format!("    je       {}{}\n", ELSE_FN, false_index).as_str()); 

                    let mut false_string : String = format!("{}{}:\n", ELSE_FN, false_index);
                    false_string.push_str(generate_cond_exp(&y, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());

                    // Case where %eax != 0, so %eax was true.  Thus, we execute the "true" part of the ternary operator, and after, we "jump" over the previous fn.  
                    *fn_index += 1;   
                    result.push_str(generate_assignment(&x, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                    result.push_str(format!("    jmp      {}{}\n", IF_FN, fn_index).as_str());
                    result.push_str(format!("\n{}", false_string).as_str());
                    result.push_str(format!("\n{}{}:\n", IF_FN, fn_index).as_str());

                },
                None => {
                    println!("Failed to find ternary when one left side (true) exists.");
                    exit_program();
                },
            }
        },
        None => {
            match cond_exp.false_exp.clone() {
                Some(y) => {
                    println!("Failed to find ternary when one right side (false) exists.");
                    exit_program();
                },
                None => (),
            }
        },
    }


    result
}

fn generate_or_rchild(expr : &parser::OrExpression, rchild : &parser::AndExpression, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
    result.push_str("    pushl    %eax # Generating ||\n");
    result.push_str(generate_and_expr(&*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
    result.push_str("    popl    %ecx\n");
    result.push_str("    orl     %ecx, %eax\n");
    result.push_str("    movl    $0, %eax\n");
    result.push_str("    setne   %al # End ||\n");

    result
}

fn generate_or_expr(exp : &parser::OrExpression, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_and_exp.clone() {
                Some(rchild) => {
                    result.push_str(generate_or_expr(&*lexp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                    result.push_str(generate_or_rchild(exp, &*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
                None => {
                    result.push_str(generate_or_expr(&*lexp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
            }
        },
        None => {
            match exp.left_and_exp.clone() {
                    Some(lchild) => {
                        match exp.right_and_exp.clone() {
                            Some(rchild) => {
                                result.push_str(generate_and_expr(&*lchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                                result.push_str(generate_or_rchild(exp, &*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                            },
                            None => {
                                result.push_str(generate_and_expr(&*lchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
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

fn generate_and_rchild(expr : &parser::AndExpression, rchild : &parser::BitOr, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();

    result.push_str("    pushl    %eax # Generating &&\n");
    result.push_str(generate_bit_or(rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
    result.push_str("    popl    %ecx\n");
    result.push_str("    cmpl    $0, %ecx\n");
    result.push_str("    setne   %cl\n");
    result.push_str("    cmpl    $0, %eax\n");
    result.push_str("    movl    $0, %eax\n");
    result.push_str("    setne   %al\n");
    result.push_str("    andb    %cl, %al # End &&\n");

    result
}

fn generate_and_expr(exp : &parser::AndExpression, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_child.clone() {
                Some(rchild) => {
                    result.push_str(generate_and_expr(&*lexp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                    result.push_str(generate_and_rchild(exp, &*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
                None => {
                    result.push_str(generate_and_expr(&*lexp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
            }
        },
        None => {
            match exp.left_child.clone() {
                    Some(lchild) => {
                        match exp.right_child.clone() {
                            Some(rchild) => {
                                result.push_str(generate_bit_or(&*lchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                                result.push_str(generate_and_rchild(exp, &*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                            },
                            None => {
                                result.push_str(generate_bit_or(&*lchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
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

fn generate_bit_or_rchild(expr : &parser::BitOr, rchild : &parser::BitXor, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();

    result.push_str("    pushl    %eax # Generating |\n");
    result.push_str(generate_bit_xor(rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
    result.push_str("    popl     %ecx\n");
    result.push_str("    orl      %ecx, %eax # End |\n");

    result
}

fn generate_bit_or(exp : &parser::BitOr, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_child.clone() {
                Some(rchild) => {
                    result.push_str(generate_bit_or(&*lexp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                    result.push_str(generate_bit_or_rchild(exp, &*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
                None => {
                    result.push_str(generate_bit_or(&*lexp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
            }
        },
        None => {
            match exp.left_child.clone() {
                    Some(lchild) => {
                        match exp.right_child.clone() {
                            Some(rchild) => {
                                result.push_str(generate_bit_xor(&*lchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                                result.push_str(generate_bit_or_rchild(exp, &*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                            },
                            None => {
                                result.push_str(generate_bit_xor(&*lchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
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

fn generate_bit_xor_rchild(expr : &parser::BitXor, rchild : &parser::BitAnd, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();

    result.push_str("    pushl    %eax # Generating ^\n");
    result.push_str(generate_bit_and(rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
    result.push_str("    popl     %ecx\n");
    result.push_str("    xorl     %ecx, %eax # End ^\n");

    result
}

fn generate_bit_xor(exp : &parser::BitXor, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_child.clone() {
                Some(rchild) => {
                    result.push_str(generate_bit_xor(&*lexp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                    result.push_str(generate_bit_xor_rchild(exp, &*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
                None => {
                    result.push_str(generate_bit_xor(&*lexp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
            }
        },
        None => {
            match exp.left_child.clone() {
                    Some(lchild) => {
                        match exp.right_child.clone() {
                            Some(rchild) => {
                                result.push_str(generate_bit_and(&*lchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                                result.push_str(generate_bit_xor_rchild(exp, &*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                            },
                            None => {
                                result.push_str(generate_bit_and(&*lchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
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

fn generate_bit_and_rchild(expr : &parser::BitAnd, rchild : &parser::EqualityExp, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();

    result.push_str("    pushl   %eax # Generating &\n");
    result.push_str(generate_eq_expr(rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
    result.push_str("    popl    %ecx\n");
    result.push_str("    andl     %ecx, %eax # End &\n");

    result
}

fn generate_bit_and(exp : &parser::BitAnd, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_child.clone() {
                Some(rchild) => {
                    result.push_str(generate_bit_and(&*lexp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                    result.push_str(generate_bit_and_rchild(exp, &*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
                None => {
                    result.push_str(generate_bit_and(&*lexp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
            }
        },
        None => {
            match exp.left_child.clone() {
                    Some(lchild) => {
                        match exp.right_child.clone() {
                            Some(rchild) => {
                                result.push_str(generate_eq_expr(&*lchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                                result.push_str(generate_bit_and_rchild(exp, &*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                            },
                            None => {
                                result.push_str(generate_eq_expr(&*lchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
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

fn generate_eq_rchild(expr : &parser::EqualityExp, rchild : &parser::RelationalExp, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();

    result.push_str("    pushl    %eax # Generating eq\n");
    result.push_str(generate_rel_expr(&*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
    result.push_str("    popl     %ecx\n");
    result.push_str("    cmpl     %eax, %ecx\n");
    result.push_str("    movl     $0, %eax\n");

    match expr.op.as_str() {
        "==" => {
            result.push_str("    sete     %al # End ==\n");
        },
        "!=" => {
            result.push_str("    setne    %al # End !=\n");
        },
        _ => {
            println!("Found an unwritten binary(Expr): {}", expr.op.as_str());
            exit_program();
        },
    }

    result
}

fn generate_eq_expr(exp : &parser::EqualityExp, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_relation_exp.clone() {
                Some(rchild) => {
                    result.push_str(generate_eq_expr(&*lexp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                    result.push_str(generate_eq_rchild(exp, &*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
                None => {
                    result.push_str(generate_eq_expr(&*lexp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
            }
        },
        None => {
            match exp.left_relation_exp.clone() {
                    Some(lchild) => {
                        match exp.right_relation_exp.clone() {
                            Some(rchild) => {
                                result.push_str(generate_rel_expr(&*lchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                                result.push_str(generate_eq_rchild(exp, &*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                            },
                            None => {
                                result.push_str(generate_rel_expr(&*lchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
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

fn generate_rel_rchild(expr : &parser::RelationalExp, rchild : &parser::BitShift, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();

    result.push_str(format!("    pushl    %eax # Generating rel: {}\n", expr.op.as_str()).as_str());
    result.push_str(generate_bit_shift(&*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
    result.push_str("    popl     %ecx\n");
    result.push_str("    cmpl     %eax, %ecx\n");
    result.push_str("    movl     $0, %eax\n");

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
            exit_program();
        },
    }

    result
}

fn generate_rel_expr(exp : &parser::RelationalExp, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_child.clone() {
                Some(rchild) => {
                    result.push_str(generate_rel_expr(&*lexp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                    result.push_str(generate_rel_rchild(exp, &*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
                None => {
                    result.push_str(generate_rel_expr(&*lexp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
            }
        },
        None => {
            match exp.left_child.clone() {
                    Some(lchild) => {
                        match exp.right_child.clone() {
                            Some(rchild) => {
                                result.push_str(generate_bit_shift(&*lchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                                result.push_str(generate_rel_rchild(exp, &*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                            },
                            None => {
                                result.push_str(generate_bit_shift(&*lchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
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

fn generate_bit_shift_rchild(expr : &parser::BitShift, rchild : &parser::AdditiveExp, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();

    result.push_str(format!("    pushl    %eax # Generating rel: {}\n", expr.op.as_str()).as_str());
    result.push_str(generate_add_expr(&*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
    result.push_str("    movl     %eax, %ecx\n");
    result.push_str("    popl     %eax\n");

    match expr.op.as_str() {
        "<<" => {
            result.push_str("    sall     %cl, %eax # End <<\n");
        },
        ">>" => {
            result.push_str("    sarl     %cl, %eax # End >>\n");
        },
        _ => {
            println!("Found an unwritten binary(BitShift): {}", expr.op.as_str());
            exit_program();
        },
    }
    

    result
}

fn generate_bit_shift(exp : &parser::BitShift, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_child.clone() {
                Some(rchild) => {
                    result.push_str(generate_bit_shift(&*lexp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                    result.push_str(generate_bit_shift_rchild(exp, &*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
                None => {
                    result.push_str(generate_bit_shift(&*lexp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
            }
        },
        None => {
            match exp.left_child.clone() {
                    Some(lchild) => {
                        match exp.right_child.clone() {
                            Some(rchild) => {
                                result.push_str(generate_add_expr(&*lchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                                result.push_str(generate_bit_shift_rchild(exp, &*rchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                            },
                            None => {
                                result.push_str(generate_add_expr(&*lchild, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
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

fn generate_add_expr_rterm(expr : &parser::AdditiveExp, rterm : &parser::Term, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
    match expr.op.as_str() {
        "-" => {
            result.push_str("    pushl   %eax # Generating binary (-)\n");
            result.push_str(generate_term(&*rterm, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
            result.push_str("    pushl   %eax\n");
            result.push_str("    popl    %ecx\n");
            result.push_str("    popl    %eax\n");
            result.push_str("    subl    %ecx, %eax # End -\n");
        },
        "+" => {
            result.push_str("    pushl   %eax # Generating binary (+)\n");
            result.push_str(generate_term(&*rterm, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
            result.push_str("    popl    %ecx\n");
            result.push_str("    addl    %ecx, %eax # End +\n");

        },
        _ => {
            println!("Found an unwritten binary(Expr): {}", expr.op.as_str());
            exit_program();
        },
    }

    result
}

fn generate_add_expr(exp : &parser::AdditiveExp, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
   
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_term.clone() {
                Some(rterm) => {
                    result.push_str(generate_add_expr(&*lexp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                    result.push_str(generate_add_expr_rterm(exp, &*rterm, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
                None => {
                    result.push_str(generate_add_expr(&*lexp, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
            }
        },
        None => {
            match exp.left_term.clone() {
                    Some(lterm) => {
                        match exp.right_term.clone() {
                            Some(rterm) => {
                                result.push_str(generate_term(&*lterm, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                                result.push_str(generate_add_expr_rterm(exp, &*rterm, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                            },
                            None => {
                                result.push_str(generate_term(&*lterm, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
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

fn generate_term_rfactor(term : &parser::Term, rfactor : &parser::PostFixUnary, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
    match term.op.as_str() {
        "*" => {
            result.push_str("    pushl    %eax # Generating binary (*)\n");
            result.push_str(generate_postfix_unary(&*rfactor, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
            result.push_str("    popl     %ecx\n");
            result.push_str("    imul     %ecx, %eax # End *\n");
        },
        "/" => {
            result.push_str("    pushl    %eax # Generating binary (/)\n");
            result.push_str(generate_postfix_unary(&*rfactor, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
            result.push_str("    pushl    %eax\n");
            result.push_str("    popl     %ecx\n");
            result.push_str("    popl     %eax\n");
            result.push_str("    movl     $0, %edx\n");  //Zero out edx
            result.push_str("    idivl    %ecx # End /\n"); //ecx is divisor

        },
        "%" => {
            result.push_str("    pushl    %eax # Generating binary (%)\n");
            result.push_str(generate_postfix_unary(&*rfactor, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
            result.push_str("    pushl    %eax\n");
            result.push_str("    popl     %ecx\n");
            result.push_str("    popl     %eax\n");
            result.push_str("    movl     $0, %edx\n");  //Zero out edx
            result.push_str("    idivl    %ecx\n");
            result.push_str("    movl     %edx, %eax # End %\n"); //Move remainder to eax
        },
        _ => {
            println!("Found an unwritten Binary(Term): {}", term.op.as_str());
            exit_program();
        },
    }

    result
}

fn generate_term(term : &parser::Term, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
    match term.left_term.clone() {
        Some(lterm) => {
            match term.right_child.clone() {
                Some(rfactor) => {
                    result.push_str(generate_term(&*lterm, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                    result.push_str(generate_term_rfactor(term, &*rfactor, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
                None => {
                    result.push_str(generate_term(&*lterm, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                },
            }
        },
        None => {
            match term.left_child.clone() {
                    Some(lfactor) => {
                        match term.right_child.clone() {
                            Some(rfactor) => {
                                result.push_str(generate_postfix_unary(&*lfactor, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                                result.push_str(generate_term_rfactor(term, &*rfactor, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                            },
                            None => {
                                result.push_str(generate_postfix_unary(&*lfactor, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
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

fn generate_factor(factor : &parser::Factor, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
    match factor.postfix_unary.clone() {
        Some (pf_un) => {
            result.push_str(generate_postfix_unary(&*pf_un, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
        },
        None => (),
    }
    match factor.unary.clone() {
        Some(un) => {
            result.push_str(generate_unary(&*un, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
        },
        None => (),
    }
    match factor.exp.clone() {
        Some(e) => {
            result.push_str(generate_assignment(&*e, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
        },
        None => (),
    }
    match factor.val {
        Some(v) => {
            result.push_str("    movl     $");
            result.push_str((v).to_string().as_str());
            result.push_str(",  %eax # Constant integer reference\n");
        },
        None => (),
    }
    match factor.var.clone() {
        Some(va) => {
            // Assign new value to variable IF it exists.
            assert!(var_map.contains_key(&(va.var_name.clone())), "Variable declaration not found when referencing: {}.", va.var_name.clone());
            let var_offset = var_map.get(&(va.var_name.clone()));
            match var_offset {
                Some (offset) => { 
                    result.push_str(format!("    movl     {}(%ebp), %eax # Variable reference for {}\n", offset, va.var_name.clone()).as_str());
                },
                None => (),
            }
        },
        None => (),
    }
    match factor.fn_call.clone() {
        Some(f) => {
            result.push_str(generate_fn_call(&f, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
        },
        None => (),
    }
    result
}

fn is_a_var (assign : &parser::Assignment, var_name : &mut String) -> bool {
    let mut result : bool = false;

    match assign.exp.clone() {
        Some (cond_exp) => {
            match cond_exp.exp.right_and_exp.clone() {
                Some (x) => return false,
                None => (),
            }
            match cond_exp.exp.left_and_exp.clone() {
                Some (and_exp) => {
                    match and_exp.right_child.clone() {
                        Some (x) => return false,
                        None => (),
                    }
                    match and_exp.left_child.clone() {
                        Some (bitor) => {
                            match bitor.right_child.clone() {
                                Some (x) => return false,
                                None => (),
                            }
                            match bitor.left_child.clone() {
                                Some (bitxor) => {
                                    match bitxor.right_child.clone() {
                                        Some(x) => return false,
                                        None => (),
                                    }
                                    match bitxor.left_child.clone() {
                                        Some (bitand) => {
                                            match bitand.right_child.clone() {
                                                Some(x) => return false,
                                                None => (),
                                            }
                                            match bitand.left_child.clone() {
                                                Some(eq_exp) => {
                                                    match eq_exp.right_relation_exp.clone() {
                                                        Some(x) => return false,
                                                        None => (),
                                                    }
                                                    match eq_exp.left_relation_exp.clone() {
                                                        Some(rel_exp) => {
                                                            match rel_exp.right_child.clone() {
                                                                Some(x) => return false,
                                                                None => (),
                                                            }
                                                            match rel_exp.left_child.clone() {
                                                                Some(bitshift) => {
                                                                    match bitshift.right_child.clone() {
                                                                        Some(x) => return false,
                                                                        None => (),
                                                                    }
                                                                    match bitshift.left_child.clone() {
                                                                        Some(add_exp) => {
                                                                            match add_exp.right_term {
                                                                                Some(x) => return false,
                                                                                None => (),
                                                                            }
                                                                            match add_exp.left_term {
                                                                                Some (term) => {
                                                                                    match term.right_child {
                                                                                        Some(x) => return false,
                                                                                        None => (),
                                                                                    }
                                                                                    match term.left_child {
                                                                                        Some (pf_unary) => {
                                                                                            if (pf_unary.op != String::new().clone()) {
                                                                                                return false;
                                                                                            }
                                                                                            match pf_unary.child {
                                                                                                Some (factor) => {
                                                                                                    match factor.unary {
                                                                                                        Some(x) => return false,
                                                                                                        None => (),
                                                                                                    }
                                                                                                    match factor.var {
                                                                                                        Some(var) => {
                                                                                                            result = true;
                                                                                                            *var_name = var.var_name.clone();
                                                                                                        },
                                                                                                        None => {
                                                                                                            match factor.exp {
                                                                                                                Some(exp) => result = is_a_var(&*exp, var_name),
                                                                                                                None => result = false,
                                                                                                            }
                                                                                                        },
                                                                                                    }
                                                                                                },
                                                                                                None => result = false,
                                                                                            }
                                                                                        },
                                                                                        None => result = false,
                                                                                    }
                                                                                },
                                                                                None => result = false,
                                                                            }
                                                                        },
                                                                        None => result = false,
                                                                    }
                                                                },
                                                                None => result = false,
                                                            }
                                                        },
                                                        None => result = false,
                                                    }
                                                },
                                                None => result = false,
                                            }
                                        },
                                        None => result = false,
                                    }
                                },
                                None => result = false,
                            }
                        },
                        None => result = false,
                    }
                },
                None => result = false,
            }
        },
        None => result = false,
    }

    result
}

fn generate_unary(un : &parser::Unary, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
    match un.child.clone() {
        Some(fact) => {
            result.push_str(generate_factor(&*fact, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
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
                    match un.child.clone() {
                        Some (un_child) => {
                            match un_child.var {
                                Some(var) => {
                                    // Assign new value to variable IF it exists.  MAKE THIS A SEP FUNC
                                    assert!(var_map.contains_key(&(var.var_name.clone())), "Variable declaration not found when referencing.");
                                    let var_offset = var_map.get(&(var.var_name.clone()));
                                    match var_offset {
                                        Some (offset) => { 
                                            result.push_str(format!("    movl     {}(%ebp), %eax # Variable reference for ++(pre)\n", offset).as_str());
                                            result.push_str("    addl     $1, %eax\n");
                                            result.push_str(format!("    movl     %eax, {}(%ebp) # Variable assignment for ++(pre)\n", offset).as_str());
                                        },
                                        None => (),
                                    }
                                }
                                None => {
                                    match un_child.exp {
                                        Some(exp) => {
                                            let mut var_name : String = String::new();
                                            if (is_a_var(&*exp.clone(), &mut var_name)) {
                                                result.push_str(generate_assignment(&*exp.clone(), var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                                                assert!(var_map.contains_key(&(var_name.clone())), "Variable declaration not found when referencing.");
                                                let var_offset = var_map.get(&(var_name.clone()));
                                                match var_offset {
                                                    Some (offset) => { 
                                                        result.push_str(format!("    movl     {}(%ebp), %eax # Variable reference for ++(pre)\n", offset).as_str());
                                                        result.push_str("    addl     $1, %eax\n");
                                                        result.push_str(format!("    movl     %eax, {}(%ebp) # Variable assignment for ++(pre)\n", offset).as_str());
                                                    },
                                                    None => (),
                                                }
                                            }
                                            else {
                                                println!("Tried to increment a non-variable.");
                                                exit_program();       
                                            }
                                        },
                                        None => {
                                            println!("Tried to increment a non-variable.");
                                            exit_program();
                                        },
                                    }
                                }
                            }
                        },
                        None => (),
                    }
                },
                "--" => {
                    match un.child.clone() {
                        Some (un_child) => {
                            match un_child.var {
                                Some(var) => {
                                    // Assign new value to variable IF it exists.  MAKE THIS A SEP FUNC
                                    assert!(var_map.contains_key(&(var.var_name.clone())), "Variable declaration not found when referencing.");
                                    let var_offset = var_map.get(&(var.var_name.clone()));
                                    match var_offset {
                                        Some (offset) => { 
                                            result.push_str(format!("    movl     {}(%ebp), %eax # Variable reference for --(pre)\n", offset).as_str());
                                            result.push_str("    subl     $1, %eax\n");
                                            result.push_str(format!("    movl     %eax, {}(%ebp) # Variable assignment for --(pre)\n", offset).as_str());
                                        },
                                        None => (),
                                    }
                                }
                                None => {
                                    match un_child.exp {
                                        Some(exp) => {
                                            let mut var_name : String = String::new();
                                            if (is_a_var(&*exp.clone(), &mut var_name)) {
                                                result.push_str(generate_assignment(&*exp.clone(), var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                                                assert!(var_map.contains_key(&(var_name.clone())), "Variable declaration not found when referencing.");
                                                let var_offset = var_map.get(&(var_name.clone()));
                                                match var_offset {
                                                    Some (offset) => { 
                                                        result.push_str(format!("    movl     {}(%ebp), %eax # Variable reference for --(pre)\n", offset).as_str());
                                                        result.push_str("    subl     $1, %eax\n");
                                                        result.push_str(format!("    movl     %eax, {}(%ebp) # Variable assignment for --(pre)\n", offset).as_str());
                                                    },
                                                    None => (),
                                                }
                                            }
                                            else {
                                                println!("Tried to increment a non-variable.");
                                                exit_program();       
                                            }
                                        },
                                        None => {
                                            println!("Tried to increment a non-variable.");
                                            exit_program();
                                        },
                                    }
                                }
                            }
                        },
                        None => (),
                    }
                },
                _ => {
                    println!("Found an unwritten unary: {}", un.op.as_str());
                    exit_program();
                },
            }
        },
        None => {
        },
   }
    result
}

fn generate_postfix_unary(un : &parser::PostFixUnary, var_map : &mut HashMap<String, i32>, stack_index : &mut i32, fn_index : &mut i32, cur_map: &mut HashMap<String, i32>, loop_start : &mut String, loop_post : &mut String, list_of_func : &Vec<parser::Function>) -> String {
    let mut result = String::new();
    match un.child.clone() {
        Some(fact) => {
            result.push_str(generate_factor(&*fact, var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
            match un.op.as_str(){
                "++" => {
                    match un.child.clone() {
                        Some (un_child) => {
                            match (un_child.var) {
                                Some (var) => {
                                    // Assign new value to variable IF it exists.  MAKE THIS A SEP FUNC
                                    assert!(var_map.contains_key(&(var.var_name.clone())), "Variable declaration not found when referencing ++(pre).");
                                    let var_offset = var_map.get(&(var.var_name.clone()));
                                    match var_offset {
                                        Some (offset) => { 
                                            result.push_str(format!("    movl     {}(%ebp), %eax # Variable reference for ++(post)\n", offset).as_str());
                                            result.push_str("    movl     %eax, %ecx\n");
                                            result.push_str("    addl     $1, %ecx\n");
                                            result.push_str(format!("    movl     %ecx, {}(%ebp) # Variable assignment for ++(post)\n", offset).as_str());
                                        },
                                        None => (),
                                    }
                                },
                                None => {
                                    match un_child.exp {
                                        Some(exp) => {
                                            let mut var_name : String = String::new();
                                            if (is_a_var(&*exp.clone(), &mut var_name)) {
                                                result.push_str(generate_assignment(&*exp.clone(), var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                                                assert!(var_map.contains_key(&(var_name.clone())), "Variable declaration not found when referencing.");
                                                let var_offset = var_map.get(&(var_name.clone()));
                                                match var_offset {
                                                    Some (offset) => { 
                                                        result.push_str(format!("    movl     {}(%ebp), %eax # Variable reference for ++(post)\n", offset).as_str());
                                                        result.push_str("    movl     %eax, %ecx\n");
                                                        result.push_str("    addl     $1, %ecx\n");
                                                        result.push_str(format!("    movl     %ecx, {}(%ebp) # Variable assignment for ++(post)\n", offset).as_str());
                                                    },
                                                    None => (),
                                                }
                                            }
                                            else {
                                                println!("Tried to increment a non-variable.");
                                                exit_program();       
                                            }
                                        },
                                        None => {
                                            println!("Tried to increment a non-variable.");
                                            exit_program();
                                        },
                                    }
                                },
                            }
                        },
                        None => (),
                    }
                }
                "--" => {
                    match un.child.clone() {
                        Some (un_child) => {
                            match (un_child.var) {
                                Some (var) => {
                                    // Assign new value to variable IF it exists.  MAKE THIS A SEP FUNC
                                    assert!(var_map.contains_key(&(var.var_name.clone())), "Variable declaration not found when referencing.");
                                    let var_offset = var_map.get(&(var.var_name.clone()));
                                    match var_offset {
                                        Some (offset) => { 
                                            result.push_str(format!("    movl     {}(%ebp), %eax # Variable reference for --(post)\n", offset).as_str());
                                            result.push_str("    movl     %eax, %ecx\n");
                                            result.push_str("    subl     $1, %ecx\n");
                                            result.push_str(format!("    movl     %ecx, {}(%ebp) # Variable assignment for --(post)\n", offset).as_str());
                                        },
                                        None => (),
                                    }
                                },
                                None => {
                                    match un_child.exp {
                                        Some(exp) => {
                                            let mut var_name : String = String::new();
                                            if (is_a_var(&*exp.clone(), &mut var_name)) {
                                                result.push_str(generate_assignment(&*exp.clone(), var_map, stack_index, fn_index, cur_map, loop_start, loop_post, list_of_func).as_str());
                                                assert!(var_map.contains_key(&(var_name.clone())), "Variable declaration not found when referencing.");
                                                let var_offset = var_map.get(&(var_name.clone()));
                                                match var_offset {
                                                    Some (offset) => { 
                                                        result.push_str(format!("    movl     {}(%ebp), %eax # Variable reference for --(post)\n", offset).as_str());
                                                        result.push_str("    movl     %eax, %ecx\n");
                                                        result.push_str("    subl     $1, %ecx\n");
                                                        result.push_str(format!("    movl     %ecx, {}(%ebp) # Variable assignment for --(post)\n", offset).as_str());
                                                    },
                                                    None => (),
                                                }
                                            }
                                            else {
                                                println!("Tried to increment a non-variable.");
                                                exit_program();       
                                            }
                                        },
                                        None => {
                                            println!("Tried to increment a non-variable.");
                                            exit_program();
                                        },
                                    }
                                },
                            }
                        },
                        None => (),
                    }
                },
                _ => {
                    
                },
            }
        },
        None => {
        },
    }
    result
}


fn generate_assembly(prog : &parser::Program, filename : String) -> String {
    let mut result = String::from("    .code32");

    for fnc in &prog.list_of_fnc {
        if (fnc.name != String::new() && fnc.is_definition == true){ 
            result.push_str(format!("\n    .globl    {}\n    .type {}, @function\n\n", fnc.name.clone(), fnc.name.clone()).as_str());
            result.push_str(generate_function(&fnc, &prog.list_of_fnc).as_str());
        }
    }

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
