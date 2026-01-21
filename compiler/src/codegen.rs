use crate::ast::*;

pub struct CodeGenerator {
    output: String,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            output: String::new(),
        }
    }

    pub fn generate(mut self, program: &Program) -> String {
        self.emit_prelude();

        for func in &program.functions {
            self.emit_function(func);
        }

        self.output
    }

    fn emit_prelude(&mut self) {
        self.output.push_str("#include <stdio.h>\n\n");
    }

    fn emit_function(&mut self, func: &FunctionDecl) {
        if func.name == "main" {
            self.output.push_str("int main() {\n");
        } else {
            self.output
                .push_str(&format!("void {}() {{\n", func.name));
        }

        self.emit_block(&func.body);

        self.output.push_str("}\n\n");
    }

    fn emit_block(&mut self, block: &Block) {
        for stmt in &block.statements {
            self.emit_stmt(stmt);
        }
    }

    fn emit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, value } => {
                self.output.push_str("int ");
                self.output.push_str(name);
                self.output.push_str(" = ");
                self.emit_expr(value);
                self.output.push_str(";\n");
            }

            Stmt::Assign { name, value } => {
                self.output.push_str(name);
                self.output.push_str(" = ");
                self.emit_expr(value);
                self.output.push_str(";\n");
            }

            Stmt::ExprStmt(expr) => {
                self.emit_expr(expr);
                self.output.push_str(";\n");
            }

            Stmt::If { condition, then_block, else_block } => {
                self.output.push_str("if (");
                self.emit_expr(condition);
                self.output.push_str(") {\n");
                self.emit_block(then_block);
                self.output.push_str("}");

                if let Some(else_block) = else_block {
                    self.output.push_str(" else {\n");
                    self.emit_block(else_block);
                    self.output.push_str("}");
                }

                self.output.push_str("\n");
            }

            Stmt::While { condition, body } => {
                self.output.push_str("while (");
                self.emit_expr(condition);
                self.output.push_str(") {\n");
                self.emit_block(body);
                self.output.push_str("}\n");
            }
        }
    }



    fn emit_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Call { name, args } => {
                if name == "print" {
                    self.output.push_str("printf(");

                    match args.first().unwrap() {
                        Expr::IntLiteral(_) | Expr::VarRef(_) => {
                            self.output.push_str("\"%d\\n\", ");
                            self.emit_expr(&args[0]);
                        }

                        Expr::StringLiteral(_) => {
                            self.output.push_str("\"%s\\n\", ");
                            self.emit_expr(&args[0]);
                        }

                        _ => panic!("Unsupported print argument"),
                    }

                    self.output.push_str(")");
                } else {
                    self.output.push_str(name);
                    self.output.push('(');
                    for (i, arg) in args.iter().enumerate() {
                        if i > 0 {
                            self.output.push_str(", ");
                        }
                        self.emit_expr(arg);
                    }
                    self.output.push(')');
                }
            }

            Expr::StringLiteral(value) => {
                self.output.push('"');
                self.output.push_str(value);
                self.output.push('"');
            }
            Expr::IntLiteral(v) => {
                self.output.push_str(&v.to_string());
            }
            Expr::VarRef(name) => {
                self.output.push_str(name);
            }
            Expr::Binary { left, op, right } => {
                self.output.push('(');
                self.emit_expr(left);
                match op {
                    BinOp::Add => self.output.push_str(" + "),
                    BinOp::Sub => self.output.push_str(" - "),
                    BinOp::Mul => self.output.push_str(" * "),
                    BinOp::Div => self.output.push_str(" / "),
                    BinOp::Greater => self.output.push_str(" > "),
                    BinOp::Less => self.output.push_str(" < "),
                    BinOp::Equal => self.output.push_str(" == "),

                }
                self.emit_expr(right);
                self.output.push(')');
            }

        }
    }
    
}
