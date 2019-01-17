#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(while_true)]

pub mod lexer;
use std::fs;

pub struct Program { 
    pub fnc : Function,
}

pub struct Function {
    pub name : String,
    pub return_type : String,
    pub list_of_blk : Vec<BlockItem>,
    pub params : Vec<String>,
}

pub struct BlockItem {
    pub state : Option<Statement>, 
    pub decl : Option<Declaration>,
}

pub struct Compound {
    pub list_of_blk : Vec<BlockItem>,
}

pub struct For {
    pub optional_exp_1 : Option<Assignment>,
    pub exp : Assignment,
    pub optional_exp_2 : Option<Assignment>,
    pub statement : Box<Statement>,
}

pub struct ForDecl {
    pub decl : Declaration,
    pub exp : Assignment,
    pub optional_exp_2 : Option<Assignment>,
    pub statement : Box<Statement>,
}

pub struct While {
    pub exp : Assignment,
    pub statement : Box<Statement>,
}

pub struct DoWhile {
    pub statement : Box<Statement>,
    pub exp : Assignment,
}

pub struct Break {
    // Empty
}

pub struct Continue {
    // Empty
}

pub struct Statement {
    pub name : String,
    pub compound : Option<Compound>,
    pub exp : Option<Assignment>,
    pub _if : Option<If>,
    pub _for : Option<For>,
    pub _for_decl : Option<ForDecl>,
    pub _while : Option<While>,
    pub _do : Option<DoWhile>,
    pub _break : Option<Break>,
    pub _continue : Option<Continue>,
}

pub struct Assignment {
    pub var : Option<Variable>,
    pub assign : Option<Box<Assignment>>,
    pub exp : Option<ConditionalExp>,
    pub op : String,
}

pub struct ConditionalExp {
    pub exp : OrExpression,
    pub true_exp : Option<Box<Assignment>>,
    pub false_exp : Option<Box<ConditionalExp>>,
}

pub struct Declaration {
    pub var : Variable,
    pub exp : Assignment,
    pub var_type : String,
}

pub struct Variable {
    pub var_name : String,
}

pub struct If {
    pub cond : Assignment,
    pub state : Option<Box<Statement>>,
    pub else_state : Option<Box<Statement>>,
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
    pub left_child : Option<Box<BitOr>>,
    pub right_child : Option<Box<BitOr>>,
}

pub struct BitOr {
    pub op : String,
    pub left_exp : Option<Box<BitOr>>,
    pub left_child : Option<Box<BitXor>>,
    pub right_child : Option<Box<BitXor>>,
}

pub struct BitXor {
    pub op : String,
    pub left_exp : Option<Box<BitXor>>,
    pub left_child : Option<Box<BitAnd>>,
    pub right_child : Option<Box<BitAnd>>,
}

pub struct BitAnd {
    pub op : String,
    pub left_exp : Option<Box<BitAnd>>,
    pub left_child : Option<Box<EqualityExp>>,
    pub right_child : Option<Box<EqualityExp>>,
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
    pub left_child : Option<Box<BitShift>>,
    pub right_child : Option<Box<BitShift>>,
}

pub struct BitShift {
    pub op : String,
    pub left_exp : Option<Box<BitShift>>,
    pub left_child : Option<Box<AdditiveExp>>,
    pub right_child : Option<Box<AdditiveExp>>,
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
    pub left_child : Option<Box<PostFixUnary>>,
    pub right_child : Option<Box<PostFixUnary>>,
}

pub struct Factor {
    pub op : String,
    pub unary : Option<Box<Unary>>,
    pub postfix_unary : Option<Box<PostFixUnary>>,
    pub exp : Option<Box<Assignment>>,
    pub val : Option<i32>,
    pub var : Option<Variable>,
}

pub struct Unary {
    pub op : String,
    pub child : Option<Box<Factor>>,
}

pub struct PostFixUnary {
    pub op : String,
    pub child : Option<Box<Factor>>,
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
            list_of_blk : Vec::new(),
            params : Vec::new(),
        }
    }
}

impl BlockItem {
    pub fn new () -> BlockItem {
        BlockItem {
            state : None,
            decl : None,
        }
    }
}

impl Compound {
    pub fn new () -> Compound {
        Compound {
            list_of_blk : Vec::new(),
        }
    }    
}

impl For {
    pub fn new() -> For {
        For {
            exp : Assignment::new(),
            optional_exp_1 : None,
            optional_exp_2 : None,
            statement : Box::new(Statement::new()),
        }
    }
}

impl ForDecl {
    pub fn new() -> ForDecl {
        ForDecl {
            exp : Assignment::new(),
            decl : Declaration::new(),
            optional_exp_2 : None,
            statement : Box::new(Statement::new()),            
        }
    }
}

impl DoWhile {
    pub fn new() -> DoWhile {
        DoWhile {
            exp : Assignment::new(),
            statement : Box::new(Statement::new()),
        }
    }
}

impl Break {
    pub fn new() -> Break {
        Break {

        }
    }
}

impl Continue {
    pub fn new() -> Continue {
        Continue {

        }
    }
}

impl While {
    pub fn new() -> While {
        While {
            exp : Assignment::new(),
            statement : Box::new(Statement::new()),
        }
    }
}

impl Statement {
    pub fn new () -> Statement {
        Statement {
            name : String::new(),
            compound : None,
            exp : None,
            _if : None,
            _for : None,
            _for_decl : None,
            _do : None,
            _break : None,
            _continue : None,
            _while : None,
        }
    }
}

impl Assignment {
    pub fn new() -> Assignment {
        Assignment {
            var : None,
            assign : None,
            exp : None,
            op : String::new(),
        }
    }

    pub fn set_to_zero() -> Assignment {
        Assignment {
            var : None,
            assign : None,
            exp : Some(ConditionalExp::set_to_zero()),
            op : String::new(),
        }
    }

    pub fn set_to_one() -> Assignment {
        Assignment {
            var : None,
            assign : None,
            exp : Some(ConditionalExp::set_to_one()),
            op : String::new(),
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

impl If {
    pub fn new() -> If {
        If {
            cond : Assignment::new(),
            state : None,
            else_state : None,
        }
    }
}

impl ConditionalExp {
    pub fn new() -> ConditionalExp {
        ConditionalExp {
            exp : OrExpression::new(),
            true_exp : None,
            false_exp : None,
        }
    }

    pub fn set_to_zero() -> ConditionalExp {
        ConditionalExp {
            exp : OrExpression::set_to_zero(),
            true_exp : None,
            false_exp : None,
        }
    }

    pub fn set_to_one() -> ConditionalExp {
        ConditionalExp {
            exp : OrExpression::set_to_one(),
            true_exp : None,
            false_exp : None,
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

    pub fn set_to_zero() -> OrExpression {
        OrExpression {
            op : String::new(),
            left_exp : None,
            left_and_exp : Some(Box::new(AndExpression::set_to_zero())),
            right_and_exp : None,
        }
    }

    pub fn set_to_one() -> OrExpression {
        OrExpression {
            op : String::new(),
            left_exp : None,
            left_and_exp : Some(Box::new(AndExpression::set_to_one())),
            right_and_exp : None,
        }
    }
}

impl AndExpression {
    pub fn new() -> AndExpression {
        AndExpression {
            op : String::new(),
            left_exp : None,
            left_child : None,
            right_child : None,
        }
    }

    pub fn set_to_zero() -> AndExpression {
        AndExpression {
            op : String::new(),
            left_exp : None,
            left_child : Some(Box::new(BitOr::set_to_zero())),
            right_child : None,
        }
    }

    pub fn set_to_one() -> AndExpression {
        AndExpression {
            op : String::new(),
            left_exp : None,
            left_child : Some(Box::new(BitOr::set_to_one())),
            right_child : None,
        }
    }
}

impl BitOr {
    pub fn new() -> BitOr {
        BitOr {
            op : String::new(),
            left_exp : None,
            left_child : None,
            right_child: None,
        }
    }

    pub fn set_to_zero() -> BitOr {
        BitOr {
            op : String::new(),
            left_exp : None,
            left_child : Some(Box::new(BitXor::set_to_zero())),
            right_child: None,
        }
    }

    pub fn set_to_one() -> BitOr {
        BitOr {
            op : String::new(),
            left_exp : None,
            left_child : Some(Box::new(BitXor::set_to_one())),
            right_child: None,
        }
    }
}

impl BitXor {
    pub fn new() -> BitXor {
        BitXor {
            op : String::new(),
            left_exp : None,
            left_child : None,
            right_child: None,
        }
    }

    pub fn set_to_zero() -> BitXor {
        BitXor {
            op : String::new(),
            left_exp : None,
            left_child : Some(Box::new(BitAnd::set_to_zero())),
            right_child: None,
        }
    }

    pub fn set_to_one() -> BitXor {
        BitXor {
            op : String::new(),
            left_exp : None,
            left_child : Some(Box::new(BitAnd::set_to_one())),
            right_child: None,
        }
    }
}

impl BitAnd {
    pub fn new() -> BitAnd {
        BitAnd {
            op : String::new(),
            left_exp : None,
            left_child : None,
            right_child: None,
        }
    }

    pub fn set_to_zero() -> BitAnd {
        BitAnd {
            op : String::new(),
            left_exp : None,
            left_child : Some(Box::new(EqualityExp::set_to_zero())),
            right_child: None,
        }
    }

    pub fn set_to_one() -> BitAnd {
        BitAnd {
            op : String::new(),
            left_exp : None,
            left_child : Some(Box::new(EqualityExp::set_to_one())),
            right_child: None,
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

    pub fn set_to_zero() -> EqualityExp {
        EqualityExp {
            op : String::new(),
            left_exp : None,
            left_relation_exp : Some(Box::new(RelationalExp::set_to_zero())),
            right_relation_exp : None,
        }
    }

    pub fn set_to_one() -> EqualityExp {
        EqualityExp {
            op : String::new(),
            left_exp : None,
            left_relation_exp : Some(Box::new(RelationalExp::set_to_one())),
            right_relation_exp : None,
        }
    }
}

impl RelationalExp {
    pub fn new() -> RelationalExp {
        RelationalExp {
            op : String::new(),
            left_exp : None,
            left_child : None,
            right_child : None,
        }
    }

    pub fn set_to_zero() -> RelationalExp {
        RelationalExp {
            op : String::new(),
            left_exp : None,
            left_child : Some(Box::new(BitShift::set_to_zero())),
            right_child : None,
        }
    }

    pub fn set_to_one() -> RelationalExp {
        RelationalExp {
            op : String::new(),
            left_exp : None,
            left_child : Some(Box::new(BitShift::set_to_one())),
            right_child : None,
        }
    }
}

impl BitShift {
    pub fn new() -> BitShift {
        BitShift {
            op : String::new(),
            left_exp : None,
            left_child : None,
            right_child: None,
        }
    }

    pub fn set_to_zero() -> BitShift {
        BitShift {
            op : String::new(),
            left_exp : None,
            left_child : Some(Box::new(AdditiveExp::set_to_zero())),
            right_child: None,
        }
    }

    pub fn set_to_one() -> BitShift {
        BitShift {
            op : String::new(),
            left_exp : None,
            left_child : Some(Box::new(AdditiveExp::set_to_one())),
            right_child: None,
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

    pub fn set_to_zero() -> AdditiveExp {
        AdditiveExp {
            op : String::new(),
            left_exp : None,
            left_term : Some(Box::new(Term::set_to_zero())),
            right_term : None,
        }
    }

    pub fn set_to_one() -> AdditiveExp {
        AdditiveExp {
            op : String::new(),
            left_exp : None,
            left_term : Some(Box::new(Term::set_to_one())),
            right_term : None,
        }
    }
}

impl Term {
    pub fn new() -> Term {
        Term {
            op: String::new(),
            left_term : None,
            left_child : None,
            right_child : None,
        }
    }

    pub fn set_to_zero() -> Term {
        Term {
            op: String::new(),
            left_term : None,
            left_child : Some(Box::new(PostFixUnary::set_to_zero())),
            right_child : None,
        }
    }

    pub fn set_to_one() -> Term {
        Term {
            op: String::new(),
            left_term : None,
            left_child : Some(Box::new(PostFixUnary::set_to_one())),
            right_child : None,
        }
    }
}

impl Factor { 
    pub fn new() -> Factor {
        Factor {
            op : String::new(),
            unary : None,
            postfix_unary : None,
            exp: None,
            val : None,
            var : None,
        }
    }

    pub fn set_to_zero() -> Factor {
        Factor {
            op : String::new(),
            unary : None,
            postfix_unary : None,
            exp: None,
            val : Some(0),
            var : None,
        }
    }

    pub fn set_to_one() -> Factor {
        Factor {
            op : String::new(),
            unary : None,
            postfix_unary : None,
            exp: None,
            val : Some(1),
            var : None,
        }
    }
}

impl Unary { 
    pub fn new() -> Unary {
        Unary {
            op : String::new(),
            child : None,
        }
    }
}

impl PostFixUnary { 
    pub fn new() -> PostFixUnary {
        PostFixUnary {
            op : String::new(),
            child : None,
        }
    }

    pub fn set_to_zero() -> PostFixUnary {
        PostFixUnary {
            op : String::new(),
            child : Some(Box::new(Factor::set_to_zero())),
        }
    }

    pub fn set_to_one() -> PostFixUnary {
        PostFixUnary {
            op : String::new(),
            child : Some(Box::new(Factor::set_to_one())),
        }
    }
}

impl Clone for Compound {
    fn clone(&self) -> Self {
        Compound {
            list_of_blk : self.list_of_blk.clone(),
        }
    }
}

impl Clone for For {
    fn clone(&self) -> Self {
        For {
            exp : self.exp.clone(),
            optional_exp_1 : self.optional_exp_1.clone(),
            optional_exp_2 : self.optional_exp_2.clone(),
            statement : self.statement.clone(),
        }
    }
}

impl Clone for ForDecl {
    fn clone(&self) -> Self {
        ForDecl {
            exp : self.exp.clone(),
            decl : self.decl.clone(),
            optional_exp_2 : self.optional_exp_2.clone(),
            statement : self.statement.clone(),
        }
    }
}

impl Clone for DoWhile {
    fn clone(&self) -> Self {
        DoWhile {
            statement : self.statement.clone(),
            exp : self.exp.clone(),
        }
    }
}

impl Clone for Break {
    fn clone(&self) -> Self {
        Break {
        }
    }
}

impl Clone for Continue {
    fn clone(&self) -> Self {
        Continue {
        }
    }
}

impl Clone for While {
    fn clone(&self) -> Self {
        While {
            statement : self.statement.clone(),
            exp : self.exp.clone(),
        }
    }
}

impl Clone for Statement {
    fn clone(&self) -> Self {
        Statement {
            _if : self._if.clone(),
            compound: self.compound.clone(),
            exp : self.exp.clone(),
            name : self.name.clone(),
            _for : self._for.clone(),
            _for_decl : self._for_decl.clone(),
            _do : self._do.clone(),
            _break : self._break.clone(),
            _continue : self._continue.clone(),
            _while : self._while.clone(),
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
            op : self.op.clone(),
        }
    }
}

impl Clone for ConditionalExp {
    fn clone(&self) -> Self {
        ConditionalExp {
            exp : self.exp.clone(),
            true_exp : self.true_exp.clone(),
            false_exp : self.false_exp.clone(),
        }
    }    
}

impl Clone for If {
    fn clone(&self) -> Self {
        If {
            cond : self.cond.clone(),
            state : self.state.clone(),
            else_state : self.else_state.clone(),
        }
    }
}

impl Clone for BlockItem {
    fn clone(&self) -> Self {
        BlockItem {
            state : self.state.clone(),
            decl : self.decl.clone(),
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
            left_child : self.left_child.clone(),
            right_child : self.right_child.clone(),
        }
    }
}

impl Clone for BitOr {
    fn clone(&self) -> Self {
        BitOr {
            op : self.op.clone(),
            left_exp : self.left_exp.clone(),
            left_child : self.left_child.clone(),
            right_child : self.right_child.clone(),
        }
    }
}

impl Clone for BitXor {
    fn clone(&self) -> Self {
        BitXor {
            op : self.op.clone(),
            left_exp : self.left_exp.clone(),
            left_child : self.left_child.clone(),
            right_child : self.right_child.clone(),
        }
    }
}

impl Clone for BitAnd {
    fn clone(&self) -> Self {
        BitAnd {
            op : self.op.clone(),
            left_exp : self.left_exp.clone(),
            left_child : self.left_child.clone(),
            right_child : self.right_child.clone(),
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

impl Clone for BitShift {
    fn clone(&self) -> Self {
        BitShift {
            op : self.op.clone(),
            left_exp : self.left_exp.clone(),
            left_child : self.left_child.clone(),
            right_child : self.right_child.clone(),
        }
    }
}

impl Clone for RelationalExp {
    fn clone(&self) -> Self {
        RelationalExp {
            op : self.op.clone(),
            left_exp : self.left_exp.clone(),
            left_child : self.left_child.clone(),
            right_child : self.right_child.clone(),
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
            left_child : self.left_child.clone(),
            right_child : self.right_child.clone(),
        }
    }
}

impl Clone for Factor {
    fn clone(&self) -> Self {
        Factor {
            op : self.op.clone(),
            unary : self.unary.clone(),
            postfix_unary : self.postfix_unary.clone(),
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
            child : self.child.clone(),
        }
    }
}

impl Clone for PostFixUnary { 
    fn clone(&self) -> Self {
        PostFixUnary {
            op : self.op.clone(),
            child : self.child.clone(),
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
   
    for bl_item in &input_prog.fnc.list_of_blk {
        match bl_item.state.clone() {
            Some (x) => {
                print!("          {} ", x.name);
                print!("[ ");
                print_statement(&x);
                println!(" ]");
            },
            None => (),
        }

        match bl_item.decl.clone() {
            Some(x) => {
                print!("          decl ");
                print!("[ ");
                print_declaration(&x);
                print!(" ]\n");                
            },
            None => (),
        }
    }
    
    println!("=====END AST PRINT=====");
}

pub fn print_while (input_while : &While) {
    print!("while");
    print_assignment(&input_while.exp);
    print!("\n            {{\n");
    print_statement(&input_while.statement);
    print!("\n            }}")
}

pub fn print_do (input_do : &DoWhile) {
    print!("do");
    print!("\n            {{\n");
    print_statement(&input_do.statement);
    print!("\n            }}");
    print!("while");
    print_assignment(&input_do.exp);
    print!("\n");
}

pub fn print_for (input_for : &For) {
    print!("for (");
    match (input_for.optional_exp_1.clone()) {
        Some (x) => print_assignment(&x),
        None => (),
    }
    print!(" ; ");
    print_assignment(&input_for.exp);
    print!(" ; ");
    match (input_for.optional_exp_2.clone()) {
        Some (x) => print_assignment(&x),
        None => (),
    }
    print!(") {{\n");

    print_statement(&input_for.statement);

    print!("}}\n");
}

pub fn print_for_decl (input_for_decl : &ForDecl) {
    print!("for (");
    
    print_declaration(&input_for_decl.decl);
    print!(" ; ");
    print_assignment(&input_for_decl.exp);
    print!(" ; ");
    match (input_for_decl.optional_exp_2.clone()) {
        Some (x) => print_assignment(&x),
        None => (),
    }
    print!(") {{\n");

    print_statement(&input_for_decl.statement);

    print!("}}\n");
}

pub fn print_continue (input_cont : &Continue) {
    print!("continue");
}

pub fn print_break (input_break : &Break) {
    print!("break");
}

pub fn print_compound(cmpd : &Compound) {
    for blk in &cmpd.list_of_blk {
        match blk.state.clone() {
            Some (x) => {
                print!("            ");
                print_statement(&x);
            },
            None => {
                match blk.decl.clone() {
                    Some (y) => {
                        print!("            ");
                        print_declaration(&y)
                    },
                    None => (),
                }
            },
        }
    }
}

pub fn print_statement (state : &Statement) {
    match state.exp.clone() {
        Some(y) => print_assignment(&y),
        None => (),
    }    
    match state._if.clone() {
        Some (y) => print_if(&y),
        None => (),
    }
    match state.compound.clone() {
        Some (y) => print_compound(&y),
        None => (),
    }
    match state._while.clone() {
        Some (y) => print_while(&y),
        None => (),
    }
    match state._for.clone() {
        Some (y) => print_for(&y),
        None => (),
    }
    match state._for_decl.clone() {
        Some (y) => print_for_decl(&y),
        None => (),
    }
    match state._do.clone() {
        Some (y) => print_do(&y),
        None => (),
    }
    match state._continue.clone() {
        Some (y) => print_continue(&y),
        None => (),
    }
    match state._break.clone() {
        Some (y) => print_break(&y),
        None => (),
    }
}

pub fn print_if (if_exp : &If) {
    print_assignment(&if_exp.cond);
    print!(" {{\n             ");

    match if_exp.state.clone() {
        Some(x) => {
            print!("{}", x.name);
            print_statement(&*x)
        },
        None => (),
    }
    print!("\n          }}\n");

    match if_exp.else_state.clone() {
        Some(x) => {
            print!("          else {{\n            {}", x.name);
            print_statement(&*x);
            print!("\n          }}");
        },
        None => (),
    }
}

pub fn print_declaration (decl : &Declaration) {
    print!("{} {} = ", decl.var_type, decl.var.var_name);
    print_assignment(&decl.exp);
}

pub fn print_assignment (exp : &Assignment) {
    match exp.var.clone() {
        Some(var) => {
            print!("{} {} ", var.var_name, exp.op);
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
                    print!("(");
                    print_cond_exp(&exp);
                    print!(")");
                },
                None => ()
            }
        },
    }
}


pub fn print_cond_exp(cond_exp : &ConditionalExp) {
    print_or(&cond_exp.exp);

    match cond_exp.true_exp.clone() {
        Some(x) => {
            match cond_exp.false_exp.clone() {
                Some(y) => {
                    print!(" ? ");
                    print_assignment(&*x);
                    print!(" : ");
                    print_cond_exp(&*y);
                },
                None => (),
            }
        },
        None => (),
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
            match exp.right_child.clone() {
                Some(r_child) => {
                    print!("(");
                    print_and(&*lexp);
                    print!(" {} ", exp.op);
                    print_bit_or(&*r_child);
                    print!(")");
                },
                None => {
                    print_and(&*lexp);
                },
            }
        },
        None => {
            match exp.left_child.clone() {
                    Some(lchild) => {
                        match exp.right_child.clone() {
                            Some(rchild) => {
                                print!("(");
                                print_bit_or(&*lchild);
                                print!(" {} ", exp.op);
                                print_bit_or(&(*rchild));
                                print!(")");
                            },
                            None => {
                                print_bit_or(&*lchild);
                            },
                        }
                    },
                    None => {
                    },
                }               
        },
    }
}

pub fn print_bit_or (exp : &BitOr) {
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_child.clone() {
                Some(r_child) => {
                    print!("(");
                    print_bit_or(&*lexp);
                    print!(" {} ", exp.op);
                    print_bit_xor(&*r_child);
                    print!(")");
                },
                None => {
                    print_bit_or(&*lexp);
                },
            }
        },
        None => {
            match exp.left_child.clone() {
                    Some(lchild) => {
                        match exp.right_child.clone() {
                            Some(rchild) => {
                                print!("(");
                                print_bit_xor(&*lchild);
                                print!(" {} ", exp.op);
                                print_bit_xor(&(*rchild));
                                print!(")");
                            },
                            None => {
                                print_bit_xor(&*lchild);
                            },
                        }
                    },
                    None => {
                    },
                }               
        },
    }
}

pub fn print_bit_xor (exp : &BitXor) {
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_child.clone() {
                Some(r_child) => {
                    print!("(");
                    print_bit_xor(&*lexp);
                    print!(" {} ", exp.op);
                    print_bit_and(&*r_child);
                    print!(")");
                },
                None => {
                    print_bit_xor(&*lexp);
                },
            }
        },
        None => {
            match exp.left_child.clone() {
                    Some(lchild) => {
                        match exp.right_child.clone() {
                            Some(rchild) => {
                                print!("(");
                                print_bit_and(&*lchild);
                                print!(" {} ", exp.op);
                                print_bit_and(&(*rchild));
                                print!(")");
                            },
                            None => {
                                print_bit_and(&*lchild);
                            },
                        }
                    },
                    None => {
                    },
                }               
        },
    }
}

pub fn print_bit_and (exp : &BitAnd) {
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_child.clone() {
                Some(r_child) => {
                    print!("(");
                    print_bit_and(&*lexp);
                    print!(" {} ", exp.op);
                    print_eq(&*r_child);
                    print!(")");
                },
                None => {
                    print_bit_and(&*lexp);
                },
            }
        },
        None => {
            match exp.left_child.clone() {
                    Some(lchild) => {
                        match exp.right_child.clone() {
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
            match exp.right_child.clone() {
                Some(r_child) => {
                    print!("(");
                    print_rel(&*lexp);
                    print!(" {} ", exp.op);
                    print_bit_shift(&*r_child);
                    print!(")");
                },
                None => {
                    print_rel(&*lexp);
                },
            }
        },
        None => {
            match exp.left_child.clone() {
                    Some(lchild) => {
                        match exp.right_child.clone() {
                            Some(rchild) => {
                                print!("(");
                                print_bit_shift(&*lchild);
                                print!(" {} ", exp.op);
                                print_bit_shift(&(*rchild));
                                print!(")");
                            },
                            None => {
                                print_bit_shift(&*lchild);
                            },
                        }
                    },
                    None => {
                    },
                }               
        },
    }
}

pub fn print_bit_shift(exp : &BitShift) {
    match exp.left_exp.clone() {
        Some(lexp) => {
            match exp.right_child.clone() {
                Some(r_child) => {
                    print!("(");
                    print_bit_shift(&*lexp); 
                    print!(" {} ", exp.op);
                    print_add(&*r_child);
                    print!(")");
                },
                None => {
                    print_bit_shift(&*lexp);
                },
            }
        },
        None => {
            match exp.left_child.clone() {
                    Some(lchild) => {
                        match exp.right_child.clone() {
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
            match term.right_child.clone() {
                Some(rfactor) => {
                    print!("(");
                    print_term(&*lterm);
                    print!(" {} ", term.op);
                    print_postfix_unary(&(*rfactor));
                    print!(")");
                },
                None => {
                    print_term(&*lterm);
                },
            }
        },
        None => {
            match term.left_child.clone() {
                    Some(lfactor) => {
                        match term.right_child.clone() {
                            Some(rfactor) => {
                                print!("(");
                                print_postfix_unary(&(*lfactor));
                                print!(" {} ", term.op);
                                print_postfix_unary(&(*rfactor));
                                print!(")");
                            },
                            None => {
                                print_postfix_unary(&(*lfactor));
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
    match factor.postfix_unary.clone() {
        Some(pf_un) => {
            print_postfix_unary(&*pf_un);
        },
        None => {
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
        },
    }
}

pub fn print_unary (unary : &Unary) {
   print!("{}(", unary.op);
   match unary.child.clone() {
        Some(child) => {
            print_factor(&(*child));
            print!(")");
        },
        None => {
        },
   }
}

pub fn print_postfix_unary (pf_unary : &PostFixUnary) {
   match pf_unary.child.clone() {
        Some(fact) => {
            print_factor(&(*fact));
        },
        None => {
        },
   }
   print!("{}", pf_unary.op);
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
    assert!(token_vec.len() >=1, "Token vector is not at least of size 1.");

    let tok : lexer::Token = get_option_token(token_vec.first().cloned());
    token_vec.remove(0);
    //println!("Token obtained: {}", tok);
    tok
}

pub fn peek_next_token(token_vec : &Vec<lexer::Token>) -> lexer::Token {
    assert!(token_vec.len() > 0, "Token vector is not at least of size 1.");
    get_option_token(token_vec.first().cloned())
}

pub fn peek_two_tokens(token_vec : &Vec<lexer::Token>) -> lexer::Token {
    assert!(token_vec.len() >=2, "Token vector is not at least of size 2.");

    let mut tok_clone = token_vec.clone();
    tok_clone.remove(0);
    get_option_token(tok_clone.first().cloned())
}

pub fn peek_n_tokens(token_vec : &Vec<lexer::Token>, n : i32) -> lexer::Token {
    assert!(token_vec.len() >=2, "Token vector is not at least of size 2.");

    let mut tok_clone = token_vec.clone();
    for i in 0..n {
        tok_clone.remove(0);
    }
    get_option_token(tok_clone.first().cloned())
}

pub fn parse_function(token_vec : &mut Vec<lexer::Token>) -> Function {
    let mut result : Function  = Function::new();
    let mut tok : lexer::Token = get_next_token(token_vec);

    assert!(tok.name == "Type", "Invalid type, saw {}", tok.name);
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

    // Block check
    while (peek_two_tokens(token_vec).value != "EOF" && peek_two_tokens(token_vec).name != "EOF TOKEN") {
        result.list_of_blk.push(parse_block_item(token_vec));
    }

    tok = get_next_token(token_vec);
    assert!(tok.name == "Punc" && tok.value == "}", "Invalid punc. (\"}\")");

    result
}


pub fn parse_block_item(token_vec : &mut Vec<lexer::Token>) -> BlockItem {
    let mut result : BlockItem = BlockItem::new();
    let mut tok : lexer::Token = peek_next_token(token_vec);

    if (tok.name=="Type") {
        result.decl = Some(parse_declaration(token_vec));
        
        tok = get_next_token(token_vec);
        assert!(tok.value == ";", "Missing semicolon, saw {}", tok.value);
    }
    else {
        result.state = Some(parse_statement(token_vec));
    }

    result
}


pub fn parse_compound(token_vec : &mut Vec<lexer::Token>) -> Compound {
    let mut result : Compound = Compound::new();
    let mut tok : lexer::Token = peek_next_token(token_vec);

    assert!(tok.name == "Punc" && tok.value == "{", "Invalid cmpd punc: (\"{\")");
    token_vec.remove(0);
    tok = peek_next_token(token_vec);

    while (!(tok.name == "Punc" && tok.value == "}")) {
        result.list_of_blk.push(parse_block_item(token_vec));
        tok = peek_next_token(token_vec);
    }

    assert!(tok.name == "Punc" && tok.value == "}", "Invalid cmpd punc: (\"}\")");
    token_vec.remove(0);
    result
}


pub fn parse_while(token_vec : &mut Vec<lexer::Token>) -> While {
    let mut result : While = While::new();
    token_vec.remove(0);
    let mut tok : lexer::Token = peek_next_token(token_vec);

    assert!(tok.value == "(", "missing ( in while, saw {}", tok.value);
    token_vec.remove(0);
    result.exp = parse_assign(token_vec);
    tok = peek_next_token(token_vec);
    assert!(tok.value == ")", "missing ) in while, saw {}", tok.value);
    token_vec.remove(0);
    result.statement = Box::new(parse_statement(token_vec));

    result
}


pub fn parse_for(token_vec : &mut Vec<lexer::Token>) -> For {
    let mut result : For = For::new();
    token_vec.remove(0);
    let mut tok : lexer::Token = peek_next_token(token_vec);

    assert!(tok.value == "(", "missing ( in for, saw {}", tok.value);
    token_vec.remove(0); 
    tok = peek_next_token(token_vec);
    if (tok.value != ";") {
        result.optional_exp_1 = Some(parse_assign(token_vec));
    } 

    tok = peek_next_token(token_vec);
    assert!(tok.value == ";", "missing ; in for, saw {}", tok.value);   
    token_vec.remove(0); 
    tok = peek_next_token(token_vec);
    if (tok.value != ";") {
        result.exp= parse_assign(token_vec);
    } 
    else {
        result.exp = Assignment::set_to_one();
    }

    tok = peek_next_token(token_vec);
    assert!(tok.value == ";", "missing ; in for, saw {}", tok.value);
    token_vec.remove(0);
    tok = peek_next_token(token_vec);
    if (tok.value != ")") {
        result.optional_exp_2 = Some(parse_assign(token_vec));
    } 

    tok = peek_next_token(token_vec);
    assert!(tok.value == ")", "missing ) in for, saw {}", tok.value);
    token_vec.remove(0);

    result.statement = Box::new(parse_statement(token_vec));
    
    result
}


pub fn parse_for_decl(token_vec : &mut Vec<lexer::Token>) -> ForDecl {
    let mut result : ForDecl = ForDecl::new();
    token_vec.remove(0);
    let mut tok : lexer::Token = peek_next_token(token_vec);

    assert!(tok.value == "(", "missing ( in for_decl, saw {}", tok.value);
    token_vec.remove(0);
    tok = peek_next_token(token_vec);
    assert!(tok.value == "int", "missing int in for_decl, saw {}", tok.value);
    result.decl = parse_declaration(token_vec);

    tok = peek_next_token(token_vec);
    assert!(tok.value == ";", "missing ; in for_decl, saw {}", tok.value);   
    token_vec.remove(0); 
    tok = peek_next_token(token_vec);
    if (tok.value != ";") {
        result.exp= parse_assign(token_vec);
    } 
    else {
        result.exp = Assignment::set_to_one();
    }

    tok = peek_next_token(token_vec);
    assert!(tok.value == ";", "missing ; in for_decl, saw {}", tok.value);
    token_vec.remove(0);
    tok = peek_next_token(token_vec);
    if (tok.value != ")") {
        result.optional_exp_2 = Some(parse_assign(token_vec));
    } 

    tok = peek_next_token(token_vec);
    assert!(tok.value == ")", "missing ) in for_decl, saw {}", tok.value);
    token_vec.remove(0);

    result.statement = Box::new(parse_statement(token_vec));
    
    result
}


pub fn parse_do_while(token_vec : &mut Vec<lexer::Token>) -> DoWhile {
    let mut result : DoWhile = DoWhile::new();
    token_vec.remove(0);
    result.statement = Box::new(parse_statement(token_vec));

    let mut tok : lexer::Token = peek_next_token(token_vec);
    token_vec.remove(0);
    assert!(tok.value == "while", "missing \"while\" in do-while, saw {}", tok.value);
    tok = peek_next_token(token_vec);

    assert!(tok.value == "(", "missing ( in do-while, saw {}", tok.value);
    token_vec.remove(0);
    result.exp = parse_assign(token_vec);
    tok = peek_next_token(token_vec);
    assert!(tok.value == ")", "missing ) in do-while, saw {}", tok.value);
    token_vec.remove(0);

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
    if (tok.name == "Keyword") {
        if (tok.value == "if") {
            result.name = String::from("if");
            result._if = Some(parse_if(token_vec));
        }
        else if (tok.value == "return") {
            result.name = String::from("return");
            token_vec.remove(0);
            result.exp = Some(parse_assign(token_vec));

            tok = get_next_token(token_vec);
            assert!(tok.value == ";", "Missing semicolon, saw {}", tok.value);
        }
        else if (tok.value == "while") {
            result._while = Some(parse_while(token_vec));
        }
        else if (tok.value == "for") {
            let tok_clone : lexer::Token;
            if (peek_n_tokens(token_vec, 2).value == "int" && peek_n_tokens(token_vec, 2).name == "Type") {
                result._for_decl = Some(parse_for_decl(token_vec));
            }
            else {
                result._for = Some(parse_for(token_vec));
            }
        }
        else if (tok.value == "do") {
            result._do = Some(parse_do_while(token_vec));
        }
        else if (tok.value == "break" || tok.value == "continue") {
            result.name = tok.value.clone();
            match tok.value.as_str() {
                "break" => result._break = Some(Break::new()),
                "continue" => result._continue = Some(Continue::new()),
                _ => (),
            }
            token_vec.remove(0);
        }
    }
    else if (tok.name == "Punc" && tok.value == "{") {
        result.compound = Some(parse_compound(token_vec));
    }
    else if (tok.name == "Punc" && tok.value == ";") {
        result.name = String::from("empty");
        token_vec.remove(0);
    }
    else {
        result.name = String::from("exp");
        result.exp = Some(parse_assign(token_vec));

        tok = get_next_token(token_vec);
        assert!(tok.value == ";", "Missing semicolon, saw {}", tok.value);
    
    }

    result
}

pub fn parse_if(token_vec : &mut Vec<lexer::Token>) -> If {
    let mut result : If = If::new();
    let mut tok : lexer::Token = peek_next_token(token_vec);

    token_vec.remove(0);
    result.cond = parse_assign(token_vec);
    result.state = Some(Box::new(parse_statement(token_vec)));
 
    tok = peek_next_token(token_vec);
    if (tok.value == "else") {
        token_vec.remove(0);
        result.else_state = Some(Box::new(parse_statement(token_vec)));
    }


    result
}

pub fn parse_declaration(token_vec : &mut Vec<lexer::Token>) -> Declaration {
    let mut result : Declaration = Declaration::new();
    let mut tok : lexer::Token = peek_next_token(token_vec);

    assert!(tok.name == "Type" && 
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

pub fn is_assignment_op(s : String) -> bool {
    let op = vec!["=", "+=", "-=", "*=", "/=", "&=", "|=", "^=", ">>=", "<<=", "%="];
    op.contains(&s.as_str())
}

pub fn parse_assign(token_vec : &mut Vec<lexer::Token>) -> Assignment {
    let mut result : Assignment = Assignment::new();
    let tok : lexer::Token = peek_next_token(token_vec);

    assert!(tok.name == "Num" ||
           tok.value == "(" ||
           valid_unary(tok.value.clone()) ||
           valid_postfix_unary(tok.value.clone()) ||
           tok.name == "Identifier",
        "Invalid assignment, saw: {}", tok.value);

    let mut next_tok = peek_two_tokens(&token_vec);

    if (tok.name == "Identifier" && is_assignment_op(next_tok.value.clone())) {

        result.op = next_tok.value.clone();

        result.var = Some(Variable {var_name : tok.value.clone()});
        while (is_assignment_op(next_tok.value.clone())) {

            token_vec.remove(0);
            token_vec.remove(0);

            result.exp = Some(parse_cond(token_vec));
            
            result.assign = Some(Box::new(Assignment {
                var : result.var.clone(),
                assign : result.assign.clone(),
                exp : result.exp.clone(),
                op : result.op.clone(),
            }));

            result.var = None;
            result.exp = None;

            next_tok = peek_next_token(token_vec);
        }
    }
    else {
        // Not an assignment, move on.       
        result.exp = Some(parse_cond(token_vec));
    }

    result
}

pub fn parse_cond(token_vec : &mut Vec<lexer::Token>) -> ConditionalExp {
    let mut result : ConditionalExp = ConditionalExp::new();
    result.exp = parse_or_exp(token_vec);

    if (peek_next_token(token_vec).value == "?") {
        token_vec.remove(0);
        result.true_exp = Some(Box::new(parse_assign(token_vec)));

        assert!(peek_next_token(token_vec).value == ":", "Ternary missing \":\", saw {}", peek_next_token(token_vec).value);

        token_vec.remove(0);
        result.false_exp = Some(Box::new(parse_cond(token_vec)));
    }

    result
}

pub fn valid_unary(s : String) -> bool {
    let op = vec!["!", "~", "-", "+", "++", "--"]; 
    return op.contains(&(s.as_str()))
}

pub fn valid_postfix_unary(s : String) -> bool {
    let op = vec!["++", "--"];
    return op.contains(&(s.as_str()))
}

pub fn parse_or_exp(token_vec : &mut Vec<lexer::Token>) -> OrExpression {
    let mut result : OrExpression = OrExpression::new();
    let mut tok : lexer::Token = peek_next_token(token_vec);

    assert!(tok.name == "Num" ||
           tok.value == "(" ||
           valid_unary(tok.value.clone()) ||
           valid_postfix_unary(tok.value.clone()) ||
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
           valid_postfix_unary(tok.value.clone()) ||
           tok.name == "Identifier", "Invalid and_exp, saw {}.", tok.value);

    result.left_child = Some(Box::new(parse_bitwise_or(token_vec)));
    
    tok = peek_next_token(token_vec);

    while (tok.value == "&&") {
        result.op = String::from(tok.value.clone());
        get_next_token(token_vec);

        result.right_child = Some(Box::new(parse_bitwise_or(token_vec)));
        tok = peek_next_token(token_vec);
        
        result.left_exp = Some(Box::new(AndExpression {
            op : result.op.clone(),
            left_exp : result.left_exp.clone(),
            left_child : result.left_child.clone(),
            right_child : result.right_child.clone(),
        }));

        result.left_child = None;
        result.right_child = None;            
        result.op = String::new();
    }
    result
}

pub fn parse_bitwise_or(token_vec : &mut Vec<lexer::Token>) -> BitOr {
    let mut result : BitOr = BitOr::new();
    let mut tok : lexer::Token = peek_next_token(token_vec);

    assert!(tok.name == "Num" ||
           tok.value == "(" ||
           valid_unary(tok.value.clone()) ||
           valid_postfix_unary(tok.value.clone()) ||
           tok.name == "Identifier",
           "Invalid rel_exp: {}.", tok.value);

    result.left_child = Some(Box::new(parse_bitwise_xor(token_vec)));
    
    tok = peek_next_token(token_vec);

    while (tok.value == "|") {
        result.op = String::from(tok.value.clone());
        get_next_token(token_vec);

        result.right_child = Some(Box::new(parse_bitwise_xor(token_vec)));
        tok = peek_next_token(token_vec);
        
        result.left_exp = Some(Box::new(BitOr {
            op : result.op.clone(),
            left_exp : result.left_exp.clone(),
            left_child : result.left_child.clone(),
            right_child : result.right_child.clone(),
        }));

        result.left_child = None;
        result.right_child = None;            
        result.op = String::new();
    }


    result
}

pub fn parse_bitwise_xor(token_vec : &mut Vec<lexer::Token>) -> BitXor {
    let mut result : BitXor = BitXor::new();
    let mut tok : lexer::Token = peek_next_token(token_vec);

    assert!(tok.name == "Num" ||
           tok.value == "(" ||
           valid_unary(tok.value.clone()) ||
           valid_postfix_unary(tok.value.clone()) ||
           tok.name == "Identifier",
           "Invalid rel_exp: {}.", tok.value);

    result.left_child = Some(Box::new(parse_bitwise_and(token_vec)));
    
    tok = peek_next_token(token_vec);

    while (tok.value == "^") {
        result.op = String::from(tok.value.clone());
        get_next_token(token_vec);

        result.right_child = Some(Box::new(parse_bitwise_and(token_vec)));
        tok = peek_next_token(token_vec);
        
        result.left_exp = Some(Box::new(BitXor {
            op : result.op.clone(),
            left_exp : result.left_exp.clone(),
            left_child : result.left_child.clone(),
            right_child : result.right_child.clone(),
        }));

        result.left_child = None;
        result.right_child = None;            
        result.op = String::new();
    }


    result
}

pub fn parse_bitwise_and(token_vec : &mut Vec<lexer::Token>) -> BitAnd {
    let mut result : BitAnd = BitAnd::new();
    let mut tok : lexer::Token = peek_next_token(token_vec);

    assert!(tok.name == "Num" ||
           tok.value == "(" ||
           valid_unary(tok.value.clone()) ||
           valid_postfix_unary(tok.value.clone()) ||
           tok.name == "Identifier",
           "Invalid rel_exp: {}.", tok.value);

    result.left_child = Some(Box::new(parse_equal_exp(token_vec)));
    
    tok = peek_next_token(token_vec);

    while (tok.value == "&") {
        result.op = String::from(tok.value.clone());
        get_next_token(token_vec);

        result.right_child = Some(Box::new(parse_equal_exp(token_vec)));
        tok = peek_next_token(token_vec);
        
        result.left_exp = Some(Box::new(BitAnd {
            op : result.op.clone(),
            left_exp : result.left_exp.clone(),
            left_child : result.left_child.clone(),
            right_child : result.right_child.clone(),
        }));

        result.left_child = None;
        result.right_child = None;            
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
           valid_postfix_unary(tok.value.clone()) ||
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
           valid_postfix_unary(tok.value.clone()) ||
           tok.name == "Identifier",
           "Invalid rel_exp: {}.", tok.value);

    result.left_child = Some(Box::new(parse_bitwise_shift(token_vec)));
    
    tok = peek_next_token(token_vec);

    while (tok.value == ">" || tok.value == ">=" || tok.value == "<" || tok.value == "<=") {
        result.op = String::from(tok.value.clone());
        get_next_token(token_vec);

        result.right_child = Some(Box::new(parse_bitwise_shift(token_vec)));
        tok = peek_next_token(token_vec);
        
        result.left_exp = Some(Box::new(RelationalExp {
            op : result.op.clone(),
            left_exp : result.left_exp.clone(),
            left_child : result.left_child.clone(),
            right_child : result.right_child.clone(),
        }));

        result.left_child = None;
        result.right_child = None;            
        result.op = String::new();
    }


    result
}

pub fn parse_bitwise_shift(token_vec : &mut Vec<lexer::Token>) -> BitShift {
    let mut result : BitShift = BitShift::new();
    let mut tok : lexer::Token = peek_next_token(token_vec);

    assert!(tok.name == "Num" ||
           tok.value == "(" ||
           valid_unary(tok.value.clone()) ||
           valid_postfix_unary(tok.value.clone()) ||
           tok.name == "Identifier",
           "Invalid rel_exp: {}.", tok.value);

    result.left_child = Some(Box::new(parse_add_exp(token_vec)));
    
    tok = peek_next_token(token_vec);

    while (tok.value == ">>" || tok.value == "<<") {
        result.op = String::from(tok.value.clone());
        get_next_token(token_vec);

        result.right_child = Some(Box::new(parse_add_exp(token_vec)));
        tok = peek_next_token(token_vec);
        
        result.left_exp = Some(Box::new(BitShift {
            op : result.op.clone(),
            left_exp : result.left_exp.clone(),
            left_child : result.left_child.clone(),
            right_child : result.right_child.clone(),
        }));

        result.left_child = None;
        result.right_child = None;            
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
           valid_postfix_unary(tok.value.clone()) ||
           tok.name == "Identifier", 
           "Invalid add_exp: {}.", tok.value);

    result.left_term = Some(Box::new(parse_term(token_vec)));

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
           valid_postfix_unary(tok.value.clone()) ||
           tok.name == "Identifier", 
           "Invalid term: {}.", tok.value);

    let left_child : Factor = parse_factor(token_vec);
    result.left_child = Some(Box::new(parse_postfix_unary(token_vec, left_child.clone())));
    tok = peek_next_token(token_vec);

    while (tok.value == "*" || tok.value == "/" || tok.value == "%") {
        result.op = String::from(tok.value.clone());
        tok = get_next_token(token_vec);
       
        if (tok.value == "/" && peek_next_token(token_vec).name == "Num") {
            if (peek_next_token(token_vec).value.parse::<i32>().unwrap() == 0) {
                println!("Tried dividing by zero.");
                std::process::exit(1);
            }
        }
        
        let right_child : Factor = parse_factor(token_vec);
        result.right_child = Some(Box::new(parse_postfix_unary(token_vec, right_child.clone())));

        tok = peek_next_token(token_vec);
         
        result.left_term = Some(Box::new(Term {
            op : result.op.clone(),
            left_term : result.left_term.clone(),
            left_child : result.left_child.clone(),
            right_child : result.right_child.clone(),
        }));

        result.left_child = None;
        result.right_child = None;            
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
    else if (valid_unary(tok.value.clone()) || valid_postfix_unary(tok.value.clone())) {
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
    let tok : lexer::Token = get_next_token(token_vec);

    result.op = String::from(tok.value);
    result.child = Some(Box::new(parse_factor(token_vec)));

    result
}


pub fn parse_postfix_unary(token_vec : &mut Vec<lexer::Token>, factor : Factor) -> PostFixUnary {
    let mut result : PostFixUnary = PostFixUnary::new();
    let tok : lexer::Token = peek_next_token(token_vec);

    if (tok.value.clone() == "--" || tok.value.clone() == "++") {
        result.op = tok.value.clone();
        token_vec.remove(0);
    }

    result.child = Some(Box::new(factor));

    //result.child = Some(Box::new(parse_factor(token_vec)));

    result
}

pub fn print_tokens(token_vec : &Vec<lexer::Token>) {
    println!("=====Resulting tokens=====");
    for token in &*token_vec {
        println!("Token: {}", token);
    }
    println!("=====End of tokens=====");
}

pub fn parse_to_ast(filename : &String) -> Program {
    // Take in file.
    let mut file_contents = fs::read_to_string(filename).expect("Could not read file.");
    print!("=====Contents of file=====\n{}", file_contents);
    print!("=====End of file contents=====\n");

    // Convert to tokens
    let mut token_vec : Vec<lexer::Token> = lexer::lexer(&mut file_contents);
    //print_tokens(&token_vec);

    // Parse tokens into AST
    let result_ast : Program = parse_program(&mut token_vec); 
    //print_ast(&result_ast);

    result_ast
}

