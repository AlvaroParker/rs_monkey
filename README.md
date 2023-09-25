# The Monkey Programming language
This is a programming language with an interpreter writen in Golang. 

## Basic monkey syntax: 
The basic Monkey lang syntax looks like this
```rust
let age = 1;
let name = "Monkey";
let result = 10 * (20 / 2);
```
For arrays, hashmaps and function we have:
```
let myArray = [1, 2, 3, 4, 5];
let parkmap = {"name": "Parker", "age": 21};
```
To access this arrays and hashmaps:
```
myArray[0] // => 1
parkmap["name"] // => "Parker"
```
We also have functions: 
```
let add = fn(a, b) { return a + b; };
```
And implicit return values inside those functions: 
```
let add = fn(a, b) { a + b; };
```
And to call this functions: 
```
add(1, 2);
```
Functions can also take functions as arguments: 
```
let twice = fn(f, x) {
    return f(f(x));
};

let addTwo = fn(x) {
    return x + 2;
};

twice(addTwo, 2);
```
This interpreter will tokenize and parse the monkey source code in a REPL (read, eval, print and loop).

The major parts of the interpreter are:
- The lexer
- The parser
- The Abstract Syntax Tree (AST)
- The internal object system
- The evaluator
