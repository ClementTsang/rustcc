int main() {
    int a = 5;
    for (int i = (1 + 2 + 3); i < (5 * 6); i = i + (1 * 2 + 3)) {
        a += i;
    }
    return a;
}
