use std::collections::{HashMap, HashSet};
use crate::ast::*;

pub struct SemanticAnalyzer {
    functions: HashMap<String, FunctionDecl>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            functions: HashMap::new(),
        }
    }

    pub fn analyze(&mut self, program: &Program) {
        self.collect_functions(program);
        self.check_functions(program);
    }

    fn collect_functions(&mut self, program: &Program) {
        for func in &program.functions {
            if self.functions.contains_key(&func.name) {
                panic!("Semantic error: duplicate function `{}`", func.name);
            }
            self.functions.insert(func.name.clone(), func.clone());
        }
    }

    fn check_functions(&self, program: &Program) {
        for func in &program.functions {
            self.check_block(&func.body);
        }
    }

    fn check_block(&self, block: &Block) {
        let mut variables = HashSet::new();

        for stmt in &block.statements {
            match stmt {
                Stmt::Let { name, value } => {
                    self.check_expr(value, &variables);
                    variables.insert(name.clone());
                }
                Stmt::ExprStmt(expr) => {
                    self.check_expr(expr, &variables);
                }
                Stmt::If {
                    condition,
                    then_block,
                    else_block,
                } => {
                    self.check_expr(condition, &variables);
                    self.check_block(then_block);
                    if let Some(b) = else_block {
                        self.check_block(b);
                    }
                }

            }
        }
    }

    fn check_expr(&self, expr: &Expr, vars: &HashSet<String>) {
        match expr {
            Expr::VarRef(name) => {
                if !vars.contains(name) {
                    panic!("Semantic error: undefined variable `{}`", name);
                }
            }

            Expr::Call { name, args } => {
                if name != "print" && !self.functions.contains_key(name) {
                    panic!("Semantic error: undefined function `{}`", name);
                }
                for arg in args {
                    self.check_expr(arg, vars);
                }
            }

            Expr::IntLiteral(_) => {}
            Expr::StringLiteral(_) => {}

            Expr::Binary { left, right, .. } => {
                self.check_expr(left, vars);
                self.check_expr(right, vars);
            }

        }
    }
}
