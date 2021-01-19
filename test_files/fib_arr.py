arr = [1, 1]

n = 60

for j in range(5000):
    for i in range(2, n):
        arr.append(arr[i - 1] + arr[i - 2])
    arr = [1, 1]
