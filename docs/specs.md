# Nexo Language Specification (v0.1)

## 1. Program Structure

A Nexo program consists of one or more function declarations.

fn main() {
...
}


- Execution always starts at `main`
- `main` must take zero parameters

---

## 2. Types

Nexo is dynamically typed.

Supported runtime types:
- `int` (64-bit signed integer)
- `string`
- `void`

---

## 3. Variables & Scope

- Variables are block-scoped
- `let` declares a variable
- Assignment requires prior declaration
- Inner scopes shadow outer scopes

Example:
let x = 10;
{
let x = 20;
}


---

## 4. Expressions

Supported:
- Arithmetic: `+ - * /`
- Comparison: `< > ==`
- Function calls
- Literals

Operator precedence:
1. `* /`
2. `+ -`
3. `< > ==`

---

## 5. Control Flow

### If
if (condition) {
...
} else {
...
}


### While
while (condition) {
...
}


- `break` exits the loop
- `continue` jumps to next iteration

---

## 6. Functions

- Functions are globally defined
- Support recursion
- Parameters are passed by value
- Each function creates a new stack frame

Return rules:
- `return expr;`
- Implicit `return void` if omitted

---

## 7. Built-in Functions

### print(value)
- Prints value to stdout
- Returns `void`

---

## 8. Truthiness

- `int`: `0` is false, non-zero is true
- All other values are false

---

## 9. Errors

The following cause runtime errors:
- Undefined variables
- Undefined functions
- Arity mismatch
- Type errors

---

## 10. Undefined Behavior

- Division by zero
- Integer overflow
