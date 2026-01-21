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

        for func in &program.functions {
            let mut vars = HashSet::new();
            self.check_block(&func.body, &mut vars);
        }
    }

    fn collect_functions(&mut self, program: &Program) {
        for func in &program.functions {
            if self.functions.contains_key(&func.name) {
                panic!("Semantic error: duplicate function `{}`", func.name);
            }
            self.functions.insert(func.name.clone(), func.clone());
        }
    }

    fn check_block(&self, block: &Block, vars: &mut HashSet<String>) {
        for stmt in &block.statements {
            self.check_stmt(stmt, vars);
        }
    }

    fn check_stmt(&self, stmt: &Stmt, vars: &mut HashSet<String>) {
        match stmt {
            Stmt::Let { name, value } => {
                self.check_expr(value, vars);
                vars.insert(name.clone());
            }

            Stmt::Assign { name, value } => {
                if !vars.contains(name) {
                    panic!("Semantic error: assigning to undefined variable `{}`", name);
                }
                self.check_expr(value, vars);
            }

            Stmt::ExprStmt(expr) => {
                self.check_expr(expr, vars);
            }

            Stmt::If {
                condition,
                then_block,
                else_block,
            } => {
                self.check_expr(condition, vars);

                let mut then_vars = vars.clone();
                self.check_block(then_block, &mut then_vars);

                if let Some(b) = else_block {
                    let mut else_vars = vars.clone();
                    self.check_block(b, &mut else_vars);
                }
            }

            Stmt::While { condition, body } => {
                self.check_expr(condition, vars);

                let mut loop_vars = vars.clone();
                self.check_block(body, &mut loop_vars);
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

            Expr::Binary { left, right, .. } => {
                self.check_expr(left, vars);
                self.check_expr(right, vars);
            }

            Expr::IntLiteral(_) => {}
            Expr::StringLiteral(_) => {}
        }
    }
}
