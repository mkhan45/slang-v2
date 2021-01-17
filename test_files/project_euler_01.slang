let i = 0
let sum = 0

while (i < 1000) {
    if ((i % 3 == 0) || (i % 5 == 0)) {
        sum += i
    }
    i += 1
}

sum
