int main() {
    int i = 100;
    {
        int a = 5;
        {
            int j = a + i;
            return a;
        }
    }
    return 100;
}
