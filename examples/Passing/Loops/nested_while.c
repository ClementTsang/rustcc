int main() {
    int a = 5;
    int b = 10;
    while (a < 100) {
        printf("A: %d, %d\n", a, b);
            a += 5;
            b += a;
        }
        a += 1;
    }
    return a + b;
}
