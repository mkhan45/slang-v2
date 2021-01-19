let arr = [1, 1]

let n = 40

for (let i = 2; i < n; i += 1) {
    push(arr, arr[i - 1] + arr[i - 2])
}

for (let i = 0; i < len(arr); i += 1) {
     print(arr[i])
}
