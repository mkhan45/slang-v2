let a = 1
let b = 1
let c = 0

for (let i = 0; i < 15; i += 1) {
    print(b)
    c = b
    b = b + a
    a = c
}

print(b)
