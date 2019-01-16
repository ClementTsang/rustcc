int main() {
    int i = 5;
    int a = 5;
    for (int i = 0; i < 5; ++i) {
        {
            int i = 100;
            a += i;
        }
        a += i;
    }
    a += i;
    return a;
}
