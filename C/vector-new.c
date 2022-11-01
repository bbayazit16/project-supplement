#include <stdio.h>
#include <stdlib.h>

typedef struct
{
    int *arr;
    size_t len;
} Vector;

Vector *vecinit()
{
    Vector *v = malloc(sizeof(Vector));
    v->arr = malloc(sizeof(int));
    v->len = 0;

    return v;
}

void push(Vector *v, int item)
{
    v->arr = realloc(v->arr, (v->len + 1) * sizeof(v->arr));

    v->arr[v->len++] = item;
}

int at(Vector *v, int index)
{
    return v->arr[index];
}

typedef struct
{
    Vector *values;
    Vector *keys;
} Map;

char *toHex(int val)
{
    char *c = (char *)malloc(64 * sizeof(char));
    c = realloc(c, (sprintf(c, "0x%x", val) + 1) * sizeof(char));
    return c;
}

int main()
{
    puts(toHex(25555555));
    // puts("ok");
    // Vector *v = vecinit();
    // Map *m = mapinit();

    // push(v, 1);
    // push(v, 2);
    // push(v, 3);

    // put(m, 1, 4);
    // put(m, 3, 5);

    // printf("First element in vector: %d\n", at(v, 0));
    // printf("Last element in vector: %d\n", at(v, v->len - 1));

    // printf("Map %d => %d\n", 1, get(m, 1));
    // printf("Map %d => %d\n", 3, get(m, 3));

    // printf("Vector length: %lu, Vector first: %d, Vector last: %d\n", v->len, at(v, 0), at(v, -1));

    return 0;
}