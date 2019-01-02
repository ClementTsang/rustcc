int main() {
    int test = 5;
    int var = 6;
    int bazing = (test = 7 + (var = 9));
    bazing = bazing - 5;
    return bazing;
}
