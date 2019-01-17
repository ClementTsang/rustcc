int main() {
    int a = 5;
    do {
        int i = 5;
        do {
            ++i;
            ++a;
        } while(i < 100);
    } while(a < 500);

    return a;
}