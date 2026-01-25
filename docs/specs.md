# Nexo Language Specification  
**Version:** v0.1 (Day 15)  
**Status:** Implemented & Runtime-Verified

This document defines the syntax, semantics, and runtime behavior of the Nexo programming language as implemented at Day 15 of development.

Only features that are fully supported by the compiler and virtual machine are specified here.

---

## 1. Overview

Nexo is a small, imperative, block-scoped programming language with functions, control flow, and a stack-based virtual machine runtime.

Programs are:
- Parsed into an AST
- Semantically analyzed
- Compiled into bytecode
- Executed by a custom VM

---

## 2. Values and Types

Nexo is dynamically typed at runtime.

### Supported Value Types

- **`int`**
  - 64-bit signed integer
  - Used for arithmetic, comparisons, and conditions

- **`string`**
  - UTF-8 string literal
  - Primarily used for output

- **`void`**
  - Represents absence of a value
  - Default return value of functions

---

## 3. Truthiness Rules

Only integers may be used in conditional expressions.

| Value | Truthiness |
|-----|-----------|
| `int(0)` | false |
| non-zero `int` | true |
| `string` | ❌ invalid |
| `void` | ❌ invalid |

Using a non-integer value in a condition is a **semantic error**.

---

## 4. Variables

### Declaration and Assignment

- Variables must be **declared before use**
- Variables are **mutable**
- Variables hold any runtime value

Example:
```nexo
let x = 10;
x = x + 1;
Scoping Rules
Variables are lexically block-scoped

Each {} block creates a new scope

Inner scopes may shadow outer variables

Variables are destroyed when the block exits

Example:

let x = 5;
if (x > 0) {
    let x = 100;
    print(x); // 100
}
print(x); // 5
5. Functions
Function Declarations
Functions must be declared before use

Functions are globally scoped

Function names must be unique

Example:

fn add(a, b) {
    return a + b;
}
Parameters
Parameters are passed by value

Parameters are local variables within the function

Arity must match exactly at call site

Return Semantics
return expr exits the function immediately

If no return statement is executed, the function returns void

main Function
Every program must define main

main must take zero parameters

Execution always starts from main

6. Expressions
Literals
Integer literals: 123, -5

String literals: "hello"

Variable Reference
x
Binary Operators
Supported binary operators:

Operator	Meaning
+	addition
-	subtraction
*	multiplication
/	integer division
<	less than
>	greater than
==	equality
Operators work only on integers

Comparison results are integers (0 or 1)

Function Calls
add(1, 2)
7. Statements
Variable Declaration
let x = 10;
Assignment
x = 20;
Expression Statement
add(1, 2);
(The result is discarded.)

Return Statement
return x;
8. Control Flow
If / Else
if (x > 0) {
    print(x);
} else {
    print(0);
}
Condition must evaluate to an integer

else block is optional

While Loops
while (x > 0) {
    x = x - 1;
}
Break
Exits the nearest enclosing loop

Only valid inside loops

Continue
Skips to the next iteration of the nearest loop

Only valid inside loops

Using break or continue outside a loop is a semantic error.

9. Built-in Functions
print(value)
Prints the value to standard output

Accepts exactly one argument

Supported argument types:

int

string

Returns void

Example:

print(42);
print("hello");
10. Errors
Semantic Errors (Compile-time)
Compilation fails immediately on:

Use of undefined variables

Assignment to undeclared variables

Incorrect number of function arguments

Invalid condition types

break / continue outside loops

Missing main function

main function with parameters

Runtime Errors
Execution stops with an error message on:

Undefined variable access

Invalid operations on values

Invalid VM execution state

Runtime errors terminate the program immediately.

11. Execution Model
Source code is compiled to bytecode

Bytecode is executed by a stack-based virtual machine

Each function call creates a new call frame

Each block creates a new lexical scope

Variable resolution follows lexical scoping rules

Values are stored on a stack

12. Current Limitations
The following features are not implemented in v0.1:

Static type system

Floating-point numbers

Arrays or collections

User-defined structs or enums

Closures or lambdas

Recursion limits or tail-call optimization

Modules or imports

Exception handling

Garbage collection

These features may be added in future versions only after runtime stability is maintained.

13. Versioning Policy
v0.x versions may introduce breaking changes

Stability is prioritized over feature count

The specification always reflects implemented behavior