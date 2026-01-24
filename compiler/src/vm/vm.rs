use std::collections::HashMap;

use super::bytecode::{Instruction, Program};
use super::value::Value;

#[derive(Debug)]
struct CallFrame {
    return_ip: usize,
    base: usize,
    locals: HashMap<String, Value>,
}

pub struct VM {
    ip: usize,
    stack: Vec<Value>,
    globals: HashMap<String, Value>,
    frames: Vec<CallFrame>,
    functions: HashMap<String, (usize, usize)>, // name -> (entry, arity)
    code: Vec<Instruction>,
}

impl VM {
    pub fn new(program: Program) -> Self {
        let mut functions = HashMap::new();
        for f in program.functions {
            functions.insert(f.name, (f.entry, f.arity));
        }

        let entry_ip = program.code.len() - 2; // Call main

        VM {
            ip: entry_ip,
            stack: vec![],
            globals: HashMap::new(),
            frames: vec![],
            functions,
            code: program.code,
        }
    }

    fn current_frame_mut(&mut self) -> Option<&mut CallFrame> {
        self.frames.last_mut()
    }

    fn current_frame(&self) -> Option<&CallFrame> {
        self.frames.last()
    }

    pub fn run(&mut self) {
        loop {
            match self.code[self.ip].clone() {
                Instruction::PushInt(v) => {
                    self.stack.push(Value::Int(v));
                }

                Instruction::PushString(s) => {
                    self.stack.push(Value::String(s));
                }

                Instruction::LoadVar(name) => {
                    // 1️⃣ check local scope
                    if let Some(frame) = self.current_frame() {
                        if let Some(v) = frame.locals.get(&name) {
                            self.stack.push(v.clone());
                            self.ip += 1;
                            continue;
                        }
                    }

                    // 2️⃣ fallback to globals
                    let v = self.globals
                        .get(&name)
                        .cloned()
                        .unwrap_or_else(|| panic!("undefined variable '{}'", name));

                    self.stack.push(v);
                }

                Instruction::StoreVar(name) => {
                    let v = self.stack.pop().unwrap();

                    if let Some(frame) = self.current_frame_mut() {
                        frame.locals.insert(name, v);
                    } else {
                        self.globals.insert(name, v);
                    }
                }

                Instruction::Add => binop(&mut self.stack, |a, b| a + b),
                Instruction::Sub => binop(&mut self.stack, |a, b| a - b),
                Instruction::Mul => binop(&mut self.stack, |a, b| a * b),
                Instruction::Div => binop(&mut self.stack, |a, b| a / b),

                Instruction::Less => cmpop(&mut self.stack, |a, b| a < b),
                Instruction::Greater => cmpop(&mut self.stack, |a, b| a > b),
                Instruction::Equal => cmpop(&mut self.stack, |a, b| a == b),

                Instruction::Print => {
                    let v = self.stack.pop().unwrap();
                    println!("{:?}", v);
                }

                Instruction::Jump(pos) => {
                    self.ip = pos;
                    continue;
                }

                Instruction::JumpIfFalse(pos) => {
                    let v = self.stack.pop().unwrap();
                    if !v.is_truthy() {
                        self.ip = pos;
                        continue;
                    }
                }

                Instruction::Call(name, argc) => {
                    let (entry, arity) = self.functions
                        .get(&name)
                        .unwrap_or_else(|| panic!("undefined function '{}'", name))
                        .clone();

                    if argc != arity {
                        panic!("arity mismatch in call to '{}'", name);
                    }

                    let frame = CallFrame {
                        return_ip: self.ip + 1,
                        base: self.stack.len() - argc,
                        locals: HashMap::new(),
                    };

                    self.frames.push(frame);
                    self.ip = entry;
                    continue;
                }

                Instruction::Return => {
                    let result = self.stack.pop();

                    let frame = self.frames
                        .pop()
                        .expect("return outside function");

                    self.stack.truncate(frame.base);

                    if let Some(v) = result {
                        self.stack.push(v);
                    }

                    self.ip = frame.return_ip;
                    continue;
                }

                Instruction::Halt => break,
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

fn cmpop(stack: &mut Vec<Value>, f: fn(i64, i64) -> bool) {
    let b = pop_int(stack);
    let a = pop_int(stack);
    stack.push(Value::Int(f(a, b) as i64));
}