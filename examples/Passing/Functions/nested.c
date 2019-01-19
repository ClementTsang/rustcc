int a () {
    return 5;
}

int b () {
    return 6;
}

int c (int one, int two) {
    return one + two;
}

int main() {
    return c(a(), b());
}
