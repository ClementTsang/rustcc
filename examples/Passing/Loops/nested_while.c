int main() {
    int a = 5;
    int b = 10;
    while (a < 100) {
        while (b < 100) {
            a += 5;
            b += a;
        }
        a += b;
    }
    return a + b;
}
