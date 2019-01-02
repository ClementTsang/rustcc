int main () {
    int a = 5;
    int b = 6;
    int  c = 7;
    int d = 8;
    int e = (a = 5) + (b = 5) + (c = 5) + (d = 5);
    return (e + a + b + c + d + e) / 5;
}
