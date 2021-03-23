# slang

Slang is a treewalk scripting language interpreter from loosely following https://craftinginterpreters.com/. I've also used <https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html> to write the parser.

Since this is a treewalk interpreter and I'm planning on writing a bytecode interpreter version, I put absolutely no effort into optimization and it's quite slow. The project euler tests are translated from <https://github.com/asterane/project-euler> since Slang is basically C with less features. Throughout the Readme, Rust syntax highlighting is used because it's close enough.

In the REPL, add '~' at the end of the input. This is the easiest way I could think of to make it work with multiline.

There's also a proof of concept bytecode compiler on the `bytecode` branch which compiles instructions for [TinyVM](https://github.com/mkhan45/tinyvm/tree/less_simple). It supports only integers, if statements, and loops, but is many times faster than the treewalk version. I plan to design and write a more complete bytecode interpreter sometime soon.

## Examples

#### Project Euler 01

Prints the sum of all numbers under 1000 that are divisible by 3 or 5.

```rust
let sum = 0

for (let i = 0; i < 1000; i += 1) {
    if ((i % 3 == 0) || (i % 5 == 0)) {
        print(i)
        sum += i
    }
}

sum
```

#### Recursive Fibonacci
```rust
fn fib(n) {
    if (n < 2) {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fib(15)
```

#### Memoized Fibonacci
```rust
let arr = [1, 1]

let n = 40

for (let i = 2; i < n; i += 1) {
    push(arr, arr[i - 1] + arr[i - 2])
}

for (let i = 0; i < len(arr); i += 1) {
     print(arr[i])
}
```

## Features

#### Variables

- Ints
- Floats
- Strings
- Arrays
- Booleans

To declare a variable, use `let`. To change the variable with typechecking, just assign it with `=`.

```rust
let x = 5
let y = "abc"

x = "abc" # errors
let y = 5 # doesn't error
```

Arithmetic operations are implemented between Floats and Integers.

#### Loops

While loops and C-style for loops are implemented.

```rust
let x = 0
while (x < 10) {
    print(x)
    x += 1
}

for (let i = 0; i < 10; i += 1) {
    print(x)
}
```

for loops are limited to one statement or expression per section.

#### Scopes

Any section delimited by curly braces is a scope. Variables in outer scopes are accessible and variables go out of scope at the end of the block they are declared in.

```rust
let x = 5

{
    print(x) # 5
    let x = "abc"
    print(x) # "abc"
}

print(x) # 5
```

#### Arrays

Arrays are expandible, heterogenous, and nestable. They can be indexed by square brackets. They also use two built in functions, `push`, and `len`.

```rust
let a = [1, 2, 3, "abc"]
print(a[0]) # 1
print(a[0 + 1]) # 2 
print(a[2 - a[0] + 1]) # 3

push(a, 5)
push(a, a)
print(a) # debug output, not very pretty
print(a[len(a) - 1][0]) # 1
```

#### Functions

First class functions are declared using the `fn` keyword.

```rust
fn add(a, b) {
    a + b
}

fn run(f, a, b) {
    f(a, b)
}

fn square(a) {
    a * a
}

fn call_n_times(n, f, x) {
    if (n == 1) {
        f(x)
    } else {
        call_n_times(n - 1, f, f(x))
    }
}

print(run(add, 15, 20)) # 35
print(call_n_times(4, square, 2)) # 65536
```

#### Examples

There are more examples in the `test_files` directory. The most impressive one is `pong_stdg.slang` which is Pong with two AIs. It needs to be run with <https://github.com/calebwin/stdg/releases/tag/v0.2.0>.
