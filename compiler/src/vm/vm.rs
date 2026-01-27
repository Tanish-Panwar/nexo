use std::collections::HashMap;

use super::bytecode::{Instruction, Program};
use super::value::Value;

#[derive(Debug)]
struct CallFrame {
    return_ip: usize,
    base: usize,
    scopes: Vec<HashMap<String, Value>>,
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

        let main_arity = functions
            .get("main")
            .map(|(_, arity)| *arity)
            .expect("No 'main' function defined");


        if main_arity != 0 {
            panic!("main() must not take arguments");
        }

        let mut code = program.code;
        let entry_ip = code.len();

        code.push(Instruction::Call("main".into(), 0));
        code.push(Instruction::Halt);

        VM {
            ip: entry_ip,
            stack: vec![],
            globals: HashMap::new(),
            frames: vec![],
            functions,
            code,
        }
    }

    fn current_frame(&self) -> Option<&CallFrame> {
        self.frames.last()
    }

    fn current_frame_mut(&mut self) -> Option<&mut CallFrame> {
        self.frames.last_mut()
    }

    pub fn run(&mut self) {
        loop {
            let instr = self.code[self.ip].clone();

            match instr {
                Instruction::PushInt(v) => {
                    self.stack.push(Value::Int(v));
                }

                Instruction::PushString(s) => {
                    self.stack.push(Value::String(s));
                }

                Instruction::LoadVar(name) => {
                    let mut value = None;

                    if let Some(frame) = self.current_frame() {
                        for scope in frame.scopes.iter().rev() {
                            if let Some(v) = scope.get(&name) {
                                value = Some(v.clone());
                                break;
                            }
                        }
                    }

                    let v = value.unwrap_or_else(|| {
                        self.globals
                            .get(&name)
                            .cloned()
                            .unwrap_or_else(|| panic!("undefined variable '{}'", name))
                    });

                    self.stack.push(v);
                }

                Instruction::StoreVar(name) => {
                    let v = self.stack.pop().expect("stack underflow");

                    if let Some(frame) = self.current_frame_mut() {
                        frame.scopes
                            .last_mut()
                            .expect("no scope")
                            .insert(name, v);
                    } else {
                        self.globals.insert(name, v);
                    }
                }

                Instruction::EnterScope => {
                    if let Some(frame) = self.current_frame_mut() {
                        frame.scopes.push(HashMap::new());
                    }
                }

                Instruction::ExitScope => {
                    if let Some(frame) = self.current_frame_mut() {
                        frame.scopes.pop();
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
                    let v = self.stack.pop().expect("stack underflow");
                    match v {
                        Value::Int(i) => println!("{}", i),
                        Value::String(s) => println!("{}", s),
                        Value::Void => println!(""),
                    }
                    self.stack.push(Value::Void);
                }

                Instruction::Jump(pos) => {
                    self.ip = pos;
                    continue;
                }

                Instruction::JumpIfFalse(pos) => {
                    let v = self.stack.pop().expect("stack underflow");
                    if !v.is_truthy() {
                        self.ip = pos;
                        continue;
                    }
                }

                Instruction::Pop => {
                    self.stack.pop();
                }

                Instruction::PushVoid => {
                    self.stack.push(Value::Void);
                }

                Instruction::Call(name, argc) => {
                    // builtin: print
                    if name == "print" {
                        let v = self.stack.pop().expect("print expects value");
                        match v {
                            Value::Int(i) => println!("{}", i),
                            Value::String(s) => println!("{}", s),
                            Value::Void => println!(""),
                        }
                        self.stack.push(Value::Void);
                        self.ip += 1;
                        continue;
                    }

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
                        scopes: vec![HashMap::new()],
                    };

                    self.frames.push(frame);
                    self.ip = entry;
                    continue;
                }


                Instruction::Return => {
                    let result = self.stack.pop().unwrap_or(Value::Void);

                    let frame = self.frames
                        .pop()
                        .expect("return outside function");

                    self.stack.truncate(frame.base);
                    self.stack.push(result);

                    self.ip = frame.return_ip;
                    continue;
                }

                Instruction::Halt => break,
            }

            self.ip += 1;
        }
    }

}

/* ===========================
   Helpers
=========================== */

fn pop_int(stack: &mut Vec<Value>) -> i64 {
    match stack.pop().expect("stack underflow") {
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
