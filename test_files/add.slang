let a = 1
let b = 1

for (let i = 0; i < 10; i += 1) {
    let c = b
    b += a
    a = c
    print(b)
}

for (let i = 0; i < 10; i += 1) {
    for (let j = 0; j < 10; j += 1) {
        print(i * j)
    }
}
