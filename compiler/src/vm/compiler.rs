use crate::ast::{
    Program as AstProgram,
    FunctionDecl,
    Block,
    Stmt,
    Expr,
    BinOp,
};

use super::bytecode::{
    Instruction,
    Program as BytecodeProgram,
    Function as BytecodeFunction,
};

pub struct BytecodeCompiler {
    code: Vec<Instruction>,
    functions: Vec<BytecodeFunction>,
    loop_stack: Vec<LoopContext>,
}

struct LoopContext {
    start: usize,
    breaks: Vec<usize>,
}

impl BytecodeCompiler {
    pub fn new() -> Self {
        Self {
            code: vec![],
            functions: vec![],
            loop_stack: vec![],
        }
    }

    /// ENTRY POINT
    pub fn compile(mut self, program: &AstProgram) -> BytecodeProgram {
        // First pass: reserve function entries
        for func in &program.functions {
            let entry = self.code.len();
            self.compile_function(func);

            self.functions.push(BytecodeFunction {
                name: func.name.clone(),
                arity: func.params.len(),
                entry,
            });
        }

        BytecodeProgram {
            functions: self.functions,
            code: self.code,
        }
    }

    fn compile_function(&mut self, func: &FunctionDecl) {
        // 🔥 bind parameters (reverse order)
        for param in func.params.iter().rev() {
            self.code.push(Instruction::StoreVar(param.clone()));
        }

        self.compile_block(&func.body);
        self.code.push(Instruction::PushVoid);
        self.code.push(Instruction::Return);
    }

    fn compile_block(&mut self, block: &Block) {
        self.code.push(Instruction::EnterScope);

        for stmt in &block.statements {
            self.compile_stmt(stmt);
        }

        self.code.push(Instruction::ExitScope);
    }


    fn compile_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, value }
            | Stmt::Assign { name, value } => {
                self.compile_expr(value);
                self.code.push(Instruction::StoreVar(name.clone()));
            }

            Stmt::ExprStmt(expr) => {
                self.compile_expr(expr);
                self.code.push(Instruction::Pop);
            }


            Stmt::Return(expr) => {
                self.compile_expr(expr);
                self.code.push(Instruction::Return);
            }

            Stmt::If {
                condition,
                then_block,
                else_block,
            } => {
                self.compile_expr(condition);
                let jmp_false = self.code.len();
                self.code.push(Instruction::JumpIfFalse(0));

                self.compile_block(then_block);

                let jmp_end = self.code.len();
                self.code.push(Instruction::Jump(0));

                let else_start = self.code.len();
                if let Some(b) = else_block {
                    self.compile_block(b);
                }

                let end = self.code.len();
                self.code[jmp_false] = Instruction::JumpIfFalse(else_start);
                self.code[jmp_end] = Instruction::Jump(end);
            }

            Stmt::While { condition, body } => {
                let start = self.code.len();

                self.compile_expr(condition);
                let exit = self.code.len();
                self.code.push(Instruction::JumpIfFalse(0));

                self.loop_stack.push(LoopContext {
                    start,
                    breaks: vec![],
                });

                self.compile_block(body);
                self.code.push(Instruction::Jump(start));

                let end = self.code.len();
                self.code[exit] = Instruction::JumpIfFalse(end);

                let ctx = self.loop_stack.pop().unwrap();
                for b in ctx.breaks {
                    self.code[b] = Instruction::Jump(end);
                }
            }

            Stmt::Break => {
                let ctx = self.loop_stack
                    .last_mut()
                    .expect("break outside loop");

                let pos = self.code.len();
                self.code.push(Instruction::Jump(0));
                ctx.breaks.push(pos);
            }

            Stmt::Continue => {
                let ctx = self.loop_stack
                    .last()
                    .expect("continue outside loop");

                self.code.push(Instruction::Jump(ctx.start));
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

                self.code.push(match op {
                    BinOp::Add => Instruction::Add,
                    BinOp::Sub => Instruction::Sub,
                    BinOp::Mul => Instruction::Mul,
                    BinOp::Div => Instruction::Div,
                    BinOp::Less => Instruction::Less,
                    BinOp::Greater => Instruction::Greater,
                    BinOp::Equal => Instruction::Equal,
                });
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