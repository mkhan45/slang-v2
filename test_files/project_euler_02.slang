let i = 0
let sum = 2

let a = 1
let b = 2

while (b < 4000000) {
    let c = b
    b = a + b
    a = c
    if (b % 2 == 0) {
        sum += b
    }
}

sum
