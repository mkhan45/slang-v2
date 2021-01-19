let prod = 0

for (let i = 0; i < 10; i += 1) {
    for (let j = 0; j < 10; j += 1) {
        prod = i * j
        if ((i == 5) && (j == 5)) {
            break
        }
    }
}

prod
