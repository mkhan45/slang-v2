let a = [1, 2, 3, "abc"]
print(a[0])
print(a[0 + 1])
print(a[2 - a[0] + 1])

push(a, 5)
push(a, a)
print(a)
print(a[len(a) - 1][0])
