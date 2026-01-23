# Nexo Language Specification (v0.1)

## Values
- int (64-bit signed)
- string
- void

## Truthiness
- int(0) → false
- any non-zero int → true
- string → not allowed in conditions

## Variables
- Variables are block scoped
- Variables are mutable
- Must be declared before use

## Functions
- Functions must be declared before use
- Parameters are passed by value
- Return value defaults to `void`

## Control Flow
- if / else supported
- while loops supported
- break / continue only allowed inside loops
- return exits the current function immediately

## Built-in Functions
- print(value)

## Errors
- Semantic errors stop compilation
- Runtime errors stop execution with message
