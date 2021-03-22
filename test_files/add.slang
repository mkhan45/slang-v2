let a = 1
let b = 1

for (let i = 0; i < 10; i += 1) {
    let c = b
    b += a
    a = c
    print(b)
}
