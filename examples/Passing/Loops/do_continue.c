int main() {
    int a = 5;

    do {
        if (a % 2 == 0) {
            ++a;
            continue; 
        }
        ++a;
    } while (a < 10);

    return a;
}
