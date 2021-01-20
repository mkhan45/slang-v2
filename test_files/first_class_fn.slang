fn add(a, b) {
    a + b
}

fn run(f, a, b) {
    f(a, b)
}

fn square(a) {
    a * a
}

fn call_n_times(n, f, x) {
    if (n == 1) {
        f(x)
    } else {
        call_n_times(n - 1, f, f(x))
    }
}

print(run(add, 15, 20))
print(call_n_times(4, square, 2))
