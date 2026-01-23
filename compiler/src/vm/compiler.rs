use crate::ast::*;
use super::bytecode::Instruction;

pub struct BytecodeCompiler {
    pub code: Vec<Instruction>,
}

impl BytecodeCompiler {
    pub fn new() -> Self {
        Self { code: vec![] }
    }

    pub fn compile_program(mut self, program: &Program) -> Vec<Instruction> {
        for func in &program.functions {
            if func.name == "main" {
                self.compile_function(func);
            }
        }

        self.code.push(Instruction::Halt);
        self.code
    }

    fn compile_function(&mut self, func: &FunctionDecl) {
        self.compile_block(&func.body);
        self.code.push(Instruction::Return);
    }

    fn compile_block(&mut self, block: &Block) {
        for stmt in &block.statements {
            self.compile_stmt(stmt);
        }
    }

    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, value } => {
                self.compile_expr(value);
                self.code.push(Instruction::StoreVar(name.clone()));
            }

            Stmt::Assign { name, value } => {
                self.compile_expr(value);
                self.code.push(Instruction::StoreVar(name.clone()));
            }

            Stmt::ExprStmt(expr) => {
                self.compile_expr(expr);
            }

            Stmt::Return(expr) => {
                self.compile_expr(expr);
                self.code.push(Instruction::Return);
            }

            Stmt::If { condition, then_block, else_block } => {
                self.compile_expr(condition);
                let jump_if_false_pos = self.code.len();
                self.code.push(Instruction::JumpIfFalse(0));

                self.compile_block(then_block);

                let jump_end = self.code.len();
                self.code.push(Instruction::Jump(0));

                let else_start = self.code.len();
                if let Some(b) = else_block {
                    self.compile_block(b);
                }

                let end = self.code.len();
                self.code[jump_if_false_pos] = Instruction::JumpIfFalse(else_start);
                self.code[jump_end] = Instruction::Jump(end);
            }

            Stmt::While { condition, body } => {
                let loop_start = self.code.len();
                self.compile_expr(condition);

                let exit_jump = self.code.len();
                self.code.push(Instruction::JumpIfFalse(0));

                self.compile_block(body);
                self.code.push(Instruction::Jump(loop_start));

                let loop_end = self.code.len();
                self.code[exit_jump] = Instruction::JumpIfFalse(loop_end);
            }

            Stmt::Break => {
                self.code.push(Instruction::Break);
            }

            Stmt::Continue => {
                self.code.push(Instruction::Continue);
            }
        }
    }

    fn compile_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::IntLiteral(v) => {
                self.code.push(Instruction::PushInt(*v));
            }

            Expr::StringLiteral(s) => {
                self.code.push(Instruction::PushString(s.clone()));
            }

            Expr::VarRef(name) => {
                self.code.push(Instruction::LoadVar(name.clone()));
            }

            Expr::Binary { left, op, right } => {
                self.compile_expr(left);
                self.compile_expr(right);

                match op {
                    BinOp::Add => self.code.push(Instruction::Add),
                    BinOp::Sub => self.code.push(Instruction::Sub),
                    BinOp::Mul => self.code.push(Instruction::Mul),
                    BinOp::Div => self.code.push(Instruction::Div),
                    BinOp::Less => self.code.push(Instruction::Less),
                    BinOp::Greater => self.code.push(Instruction::Greater),
                    BinOp::Equal => self.code.push(Instruction::Equal),
                }

            }

            Expr::Call { name, args } => {
                for arg in args {
                    self.compile_expr(arg);
                }

                if name == "print" {
                    self.code.push(Instruction::Print);
                } else {
                    self.code.push(Instruction::Call(name.clone(), args.len()));
                }
            }
        }
    }
}
