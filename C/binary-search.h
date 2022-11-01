#include <stdio.h>

int binarySearch(const int *arr, int item, int start, int end)
{

    if (start > end)
        return -1;

    int idx = (end + start) / 2;
    int mid = *(arr + idx);

    if (mid == item)
        return idx;

    if (item > mid)
        return binarySearch(arr, item, idx + 1, end);

    return binarySearch(arr, item, start, idx - 1);
}