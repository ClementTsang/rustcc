int main() {
    int foo = 100;
    {
        int foo;
        foo = 15;
    }
    return foo;
}
