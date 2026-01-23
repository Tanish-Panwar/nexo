use std::collections::HashMap;
use crate::ast::*;

pub struct SemanticAnalyzer {
    functions: HashMap<String, FunctionDecl>,
    scopes: Vec<HashMap<String, ()>>,
    in_loop: bool,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        SemanticAnalyzer {
            functions: HashMap::new(),
            scopes: Vec::new(),
            in_loop: false,
        }
    }

    pub fn analyze(&mut self, program: &Program) {
        self.collect_functions(program);

        for func in &program.functions {
            self.check_function(func);
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

    /* ======================
        SCOPE MANAGEMENT
    ====================== */

    fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare_var(&mut self, name: &str) {
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name.to_string(), ());
    }

    fn is_var_defined(&self, name: &str) -> bool {
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(name) {
                return true;
            }
        }
        false
    }

    /* ======================
        FUNCTION CHECK
    ====================== */

    fn check_function(&mut self, func: &FunctionDecl) {
        self.push_scope();

        // Parameters are local variables
        for param in &func.params {
            self.declare_var(param);
        }

        self.check_block(&func.body);

        self.pop_scope();
    }

    /* ======================
        BLOCK / STATEMENTS
    ====================== */

    fn check_block(&mut self, block: &Block) {
        self.push_scope();

        for stmt in &block.statements {
            self.check_stmt(stmt);
        }

        self.pop_scope();
    }

    fn check_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, value } => {
                self.check_expr(value);
                self.declare_var(name);
            }

            Stmt::Assign { name, value } => {
                if !self.is_var_defined(name) {
                    panic!("Semantic error: assigning to undefined variable `{}`", name);
                }
                self.check_expr(value);
            }

            Stmt::Return(expr) => {
                self.check_expr(expr);
            }

            Stmt::ExprStmt(expr) => {
                self.check_expr(expr);
            }

            Stmt::If {
                condition,
                then_block,
                else_block,
            } => {
                self.check_expr(condition);
                self.check_block(then_block);
                if let Some(b) = else_block {
                    self.check_block(b);
                }
            }

            Stmt::While { condition, body } => {
                self.check_expr(condition);

                let old = self.in_loop;
                self.in_loop = true;
                self.check_block(body);
                self.in_loop = old;
            }

            Stmt::Break | Stmt::Continue => {
                if !self.in_loop {
                    panic!("break/continue used outside loop");
                }
            }

        }
    }

    /* ======================
        EXPRESSIONS
    ====================== */

    fn check_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::VarRef(name) => {
                if !self.is_var_defined(name) {
                    panic!("Semantic error: undefined variable `{}`", name);
                }
            }

            Expr::Call { name, args } => {
                if name == "print" {
                    if args.len() != 1 {
                        panic!("Semantic error: print expects 1 argument");
                    }
                } else if let Some(func) = self.functions.get(name) {
                    if args.len() != func.params.len() {
                        panic!(
                            "Semantic error: function `{}` expects {} args, got {}",
                            name,
                            func.params.len(),
                            args.len()
                        );
                    }
                } else {
                    panic!("Semantic error: undefined function `{}`", name);
                }

                for arg in args {
                    self.check_expr(arg);
                }
            }


            Expr::Binary { left, right, .. } => {
                self.check_expr(left);
                self.check_expr(right);
            }

            Expr::IntLiteral(_) | Expr::StringLiteral(_) => {}
        }
    }
}
