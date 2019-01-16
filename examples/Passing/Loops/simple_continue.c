int main() {
    int a = 0;
    for (int i = 0; i < 100; ++i) {
        if (i % 2 == 1) {
            continue;
        }
        a += i;
    }
    return a;
}
