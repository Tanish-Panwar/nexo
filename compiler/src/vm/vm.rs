use std::collections::HashMap;
use super::bytecode::Instruction;
use super::value::Value;


pub struct VM {
    ip: usize,
    stack: Vec<Value>,
    vars: HashMap<String, Value>,
    code: Vec<Instruction>,
}

impl VM {
    pub fn new(code: Vec<Instruction>) -> Self {
        VM {
            ip: 0,
            stack: vec![],
            vars: HashMap::new(),
            code,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.code[self.ip].clone() {
                Instruction::PushInt(v) => self.stack.push(Value::Int(v)),
                Instruction::PushString(s) => self.stack.push(Value::String(s)),
                Instruction::LoadVar(n) => {
                    let v = self.vars.get(&n).cloned().expect("undefined var");
                    self.stack.push(v);
                }
                Instruction::StoreVar(n) => {
                    let v = self.stack.pop().unwrap();
                    self.vars.insert(n, v);
                }

                Instruction::Add => binop(&mut self.stack, |a, b| a + b),
                Instruction::Sub => binop(&mut self.stack, |a, b| a - b),
                Instruction::Mul => binop(&mut self.stack, |a, b| a * b),
                Instruction::Div => binop(&mut self.stack, |a, b| a / b),

                Instruction::Less => {
                    let b = pop_int(&mut self.stack);
                    let a = pop_int(&mut self.stack);
                    self.stack.push(Value::Int((a < b) as i64));
                }

                Instruction::Equal => {
                    let b = pop_int(&mut self.stack);
                    let a = pop_int(&mut self.stack);
                    self.stack.push(Value::Int((a == b) as i64));
                }

                Instruction::Greater => {
                    let b = pop_int(&mut self.stack);
                    let a = pop_int(&mut self.stack);
                    self.stack.push(Value::Int((a > b) as i64));
                }

                Instruction::Jump(pos) => {
                    self.ip = pos;
                    continue;
                }

                Instruction::JumpIfFalse(pos) => {
                    let cond = self.stack.pop().unwrap();
                    if !cond.is_truthy() {
                        self.ip = pos;
                        continue;
                    }
                }

                Instruction::Print => {
                    let v = self.stack.pop().unwrap();
                    match v {
                        Value::Int(i) => println!("{}", i),
                        Value::String(s) => println!("{}", s),
                        Value::Void => {}
                    }
                }

                Instruction::Return => break,
                Instruction::Halt => break,

                _ => unimplemented!(),
            }

            self.ip += 1;
        }
    }
}

fn pop_int(stack: &mut Vec<Value>) -> i64 {
    match stack.pop().unwrap() {
        Value::Int(v) => v,
        _ => panic!("expected int"),
    }
}

fn binop(stack: &mut Vec<Value>, f: fn(i64, i64) -> i64) {
    let b = pop_int(stack);
    let a = pop_int(stack);
    stack.push(Value::Int(f(a, b)));
}
