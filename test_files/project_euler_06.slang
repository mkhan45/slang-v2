let sum_square = 0
let square_square = 0

for (let i = 1; i < 101; i += 1) {
    sum_square += i * i
    square_square += i
}

square_square = square_square * square_square

square_square - sum_square
