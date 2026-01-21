use std::collections::HashMap;

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
        for stmt in &block.statements {
            self.check_stmt(stmt);
        }
    }

    fn check_stmt(&self, stmt: &Stmt) {
        match stmt {
            Stmt::ExprStmt(expr) => self.check_expr(expr),
        }
    }

    fn check_expr(&self, expr: &Expr) {
        match expr {
            Expr::Call { name, args } => {
                if name != "print" && !self.functions.contains_key(name) {
                    panic!("Semantic error: undefined function `{}`", name);
                }

                for arg in args {
                    self.check_expr(arg);
                }
            }

            Expr::StringLiteral(_) => {}
        }
    }
}
