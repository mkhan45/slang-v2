# slang

Slang is a treewalk scripting language interpreter from loosely following https://craftinginterpreters.com/. I've also used <https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html> to write the parser.

## Examples

#### Project Euler 01

Prints the sum of all numbers under 1000 that are divisible by 3 or 5.

```lua
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
```lua
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
```lua
let arr = [1, 1]

let n = 40

for (let i = 2; i < n; i += 1) {
    push(arr, arr[i - 1] + arr[i - 2])
}

for (let i = 0; i < len(arr); i += 1) {
     print(arr[i])
}
```
