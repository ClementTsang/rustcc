int main() {
    int a = 5;
    int b = 10;
    if (a == b) b = 15;
    else if (a > b) b = 5;
    else if (a < b - 9) b = 100;
    else b = (1 < 2) ? 404 : 505;

    if (b == a) b = 15;
    else if (b < a) return a;
    return b;
}
