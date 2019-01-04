int main() {
    int a = 5;
    int b = a + 6;
    int c = a++ * 5 / -(--b + 5);
    int d = ++c + --b + a++;
    d *= 5;
    d *= ++c;
    b ^= d;
    return ++d + b + c-- * ++a;
}
