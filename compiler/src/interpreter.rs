use std::collections::HashMap;
use crate::ast::*;

type Env = HashMap<String, Value>;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Str(String),
    Void,
}

#[derive(Debug)]
enum Control {
    None,
    Break,
    Continue,
    Return(Value),
}

pub struct Interpreter {
    functions: HashMap<String, FunctionDecl>,
    loop_depth: usize,
}

impl Interpreter {
    pub fn new(program: &Program) -> Self {
        let mut functions = HashMap::new();
        for f in &program.functions {
            functions.insert(f.name.clone(), f.clone());
        }

        Interpreter {
            functions,
            loop_depth: 0,
        }
    }

    pub fn run(&mut self) {
        let main = self.functions.get("main")
            .cloned()
            .expect("no main() function");

        let mut env = HashMap::new();
        self.exec_block(&main.body, &mut env);
    }

    fn exec_block(&mut self, block: &Block, env: &mut Env) -> Control {
        for stmt in &block.statements {
            match self.exec_stmt(stmt, env) {
                Control::None => {}
                c => return c,
            }
        }
        Control::None
    }

    fn exec_stmt(&mut self, stmt: &Stmt, env: &mut Env) -> Control {
        match stmt {
            Stmt::Let { name, value } => {
                let v = self.eval_expr(value, env);
                env.insert(name.clone(), v);
                Control::None
            }

            Stmt::Assign { name, value } => {
                let v = self.eval_expr(value, env);
                let slot = env.get_mut(name)
                    .expect("assignment to undefined variable");
                *slot = v;
                Control::None
            }

            Stmt::ExprStmt(expr) => {
                self.eval_expr(expr, env);
                Control::None
            }

            Stmt::Return(expr) => {
                let v = self.eval_expr(expr, env);
                Control::Return(v)
            }

            Stmt::If { condition, then_block, else_block } => {
                if self.eval_expr(condition, env).is_true() {
                    self.exec_block(then_block, env)
                } else if let Some(b) = else_block {
                    self.exec_block(b, env)
                } else {
                    Control::None
                }
            }

            Stmt::While { condition, body } => {
                self.loop_depth += 1;

                while self.eval_expr(condition, env).is_true() {
                    match self.exec_block(body, env) {
                        Control::None => {}
                        Control::Continue => continue,
                        Control::Break => break,
                        Control::Return(v) => {
                            self.loop_depth -= 1;
                            return Control::Return(v);
                        }
                    }
                }

                self.loop_depth -= 1;
                Control::None
            }

            Stmt::Break => {
                if self.loop_depth == 0 {
                    panic!("break used outside loop");
                }
                Control::Break
            }

            Stmt::Continue => {
                if self.loop_depth == 0 {
                    panic!("continue used outside loop");
                }
                Control::Continue
            }
        }
    }

    fn eval_expr(&mut self, expr: &Expr, env: &mut Env) -> Value {
        match expr {
            Expr::IntLiteral(i) => Value::Int(*i),
            Expr::StringLiteral(s) => Value::Str(s.clone()),

            Expr::VarRef(name) => env.get(name)
                .unwrap_or_else(|| panic!("undefined variable {}", name))
                .clone(),

            Expr::Binary { left, op, right } => {
                let l = self.eval_expr(left, env);
                let r = self.eval_expr(right, env);

                match (l, r, op) {
                    (Value::Int(a), Value::Int(b), BinOp::Add) => Value::Int(a + b),
                    (Value::Int(a), Value::Int(b), BinOp::Sub) => Value::Int(a - b),
                    (Value::Int(a), Value::Int(b), BinOp::Mul) => Value::Int(a * b),
                    (Value::Int(a), Value::Int(b), BinOp::Div) => Value::Int(a / b),
                    (Value::Int(a), Value::Int(b), BinOp::Greater) => Value::Int((a > b) as i64),
                    (Value::Int(a), Value::Int(b), BinOp::Less) => Value::Int((a < b) as i64),
                    (Value::Int(a), Value::Int(b), BinOp::Equal) => Value::Int((a == b) as i64),
                    _ => panic!("invalid binary operation"),
                }
            }

            Expr::Call { name, args } => {
                if name == "print" {
                    let v = self.eval_expr(&args[0], env);
                    match v {
                        Value::Int(i) => println!("{}", i),
                        Value::Str(s) => println!("{}", s),
                        _ => {}
                    }
                    Value::Void
                } else {
                    let func = self.functions.get(name)
                        .cloned()
                        .unwrap_or_else(|| panic!("undefined function {}", name));

                    let mut local: HashMap<String, Value> = HashMap::new();
                    for (p, a) in func.params.iter().zip(args.iter()) {
                        let v = self.eval_expr(a, env);
                        local.insert(p.clone(), v);
                    }

                    match self.exec_block(&func.body, &mut local) {
                        Control::Return(v) => v,
                        _ => Value::Void,
                    }
                }
            }
        }
    }
}

impl Value {
    fn is_true(&self) -> bool {
        matches!(self, Value::Int(i) if *i != 0)
    }
}
