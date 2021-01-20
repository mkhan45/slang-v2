fn check(n) {
    let res = true

    for (let i = 20; i > 10; i = i - 1) {
        if (n % i != 0) {
            res = false
            break
        }
    }

    res
}

let n = 9699690

while (check(n) != true) {
    n += 19
}

n
