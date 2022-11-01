#include <stdio.h>
#include <stdlib.h>

typedef struct
{
    int *arr;
    size_t cap;
    size_t len;
} Vector;

Vector *new_vector()
{
    Vector *vec = (Vector *)malloc(sizeof(Vector));
    vec->arr = (int *)malloc(sizeof(int));
    vec->cap = 1;
    vec->len = 0;
    return vec;
}

void push(Vector *vec, int item)
{
    if (vec->len * sizeof(int) > vec->cap)
    {
        vec->arr = (int *)realloc(vec->arr, vec->cap * 2);
        vec->cap *= 2;
    }
    int *ptr = vec->arr + vec->len;
    *ptr = item;
    vec->len++;
}

int at(Vector *vec, int idx)
{
    if (idx < 0)
    {
        idx = vec->len + idx;
    }
    return *(vec->arr + idx);
}