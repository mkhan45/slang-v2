fn check(p) {
    let digits = []
    let n = p
    let l = 0

    for (let i = 0; i < 6; i += 1) {
        push(digits, n % 10)

        if (n < 10) {
            l = i + 1
            break
        }

        n = floor(n / 10)
    }

    let res = true

    for (let i = 0; i < l / 2; i += 1) {
        if (digits[i] != digits[l - i - 1]) {
            res = false
            break
        }
    }

    res
}

{
   let result = 0

   for (let i = 999; i > 99; i = i - 1) {
       let broke = false

       for (let j = 999; j > i; j = j - 1) {
           let prod = i * j

           if (check(prod)) {
               print(prod)
               result = prod
               broke = true
               break
           }
       }

       if (broke) {
           break
       }
   }

   print(result)
}
