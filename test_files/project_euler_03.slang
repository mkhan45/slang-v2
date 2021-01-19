# doesn't work because floating point modulus is weird

fn factor(n) {
    let fac = n

    for (let i = 2; i < n / 2; i += 1) {
        print(i)
        print(n / i)
        if (n % i == 0) {
            fac = i;
            break;
        }

        if (fac == n) {
            break
        } else {
            let new = factor(n / fac)
            if (new > fac) {
                fac = new
            }
        }
    }

    fac
}

let n = 600851475143

factor(n)
