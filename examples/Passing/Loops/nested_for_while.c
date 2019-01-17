int main() {
    int a = 10;
    int b = 0;
    for (int i = 0; i < 5; ++i) {
        while (a > 0) {
            --a;
            b += a;
        }
        b += i;
    }

    return b;
}