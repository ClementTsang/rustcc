int main() {
    int a = 0;
    for (int i = 0; i < 5; ++i) {
        for (int j = 0 ; j < 10; j++) {
            a += j;
        }
        a -= i;
    }
    return a;
}
