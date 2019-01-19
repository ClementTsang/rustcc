int fib (int n) {
    if (n == 0 || n == 1) {
        return n;
    }
    return fib(n-1) + fib(n-2);
}

int main() {
    return fib(5) + fib(6) + fib (7);
}
