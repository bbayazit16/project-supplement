void swap(int *a, int *b)
{
    int c = *a;
    *a = *b;
    *b = c;
}

int sub(int a, int b)
{
    int c = a - b;
    return c;
}

int *getPtr()
{
    int x = 3;
    return &x;
}

int main()
{

    int a = 32;
    int b = 64;

    printf("%d", a);
    printf("%d", b);

    swap(&a, &b);

    printf("Result is %d", sub(b, a));

    return 0;
}