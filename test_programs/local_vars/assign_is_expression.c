int main(void) {
    int a;
    int b = (a = 4) + 1;
    return a * 10 + b;
}
