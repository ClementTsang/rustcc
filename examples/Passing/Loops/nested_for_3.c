int main() {
    int a = 0;
    int i = 0;
    for (; i < 2; ++i) {
        int j = 0;
        for (; j < 3; j++) {
            a += j;
        }
    }
    return a;
}
