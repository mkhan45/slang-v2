let n = 600851475143
let res = n

let i = 2
while (i < n / 2) {
    if (n % i == 0) {
        res = i
        break
    }
    i += 1
}

res
