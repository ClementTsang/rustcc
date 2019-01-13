int main() {
    int a = 5;
    int b = 404;

    if (b == a) b = 15;
    else if (b < a) return a;
    return b;
}
