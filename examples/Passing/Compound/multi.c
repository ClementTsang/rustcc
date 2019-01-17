int main() {
    int a = 5;
    {
        int a = 6;
        {
            a = 100;
            {
                int a = 1000;
            }
        }
        a = 123;
    }
    return a;
}
