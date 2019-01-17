int main() {
    int i = 5;
    {
        int i = 100;
        {
            {
                for (int i = 0; i < 150; ++i) {
                    ++i;
                }
            }
        }
    }
    return i;
}
