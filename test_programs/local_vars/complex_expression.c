int main(void) {
    int x = 10;
    int y = 3;
    int z = (x / y) * y + x % y;
    int w = (x > y) && (z == x) || !y;
    return z + w * 100;
}
