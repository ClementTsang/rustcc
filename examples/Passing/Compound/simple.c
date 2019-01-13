int main () {
    int a = 5;
    int b = 10;
    int cintd = 11;
    int intent10 = 100;

    if (a == b) {
        if (cintd) {
            return intent10;
        }
        else {
            b = 100;
            return b;
        }
    }else{
        if (intent10 == 100) {
            cintd += 501;
        }
        else {
            cintd += 1000;
        }

        cintd += b;
    }
    return cintd;
}

