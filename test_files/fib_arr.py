arr = [1, 1]

n = 40

for i in range(2, n):
    arr.append(arr[i - 1] + arr[i - 2])

for i in arr:
    print(i)
