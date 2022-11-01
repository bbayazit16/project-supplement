#include <stdio.h>
#include <stdlib.h>

typedef struct LinkedList
{
    int val;
    struct LinkedList *next;
} LinkedList;

LinkedList *init(int val)
{
    LinkedList *l = malloc(sizeof(LinkedList));
    l->val = val;
    l->next = NULL;
    return l;
}

LinkedList *initdef()
{
    return malloc(sizeof(LinkedList));
}

void set(LinkedList *head, int val)
{
    LinkedList *temp, *p;
    temp = init(val);
    if (head->val == 0)
    {
        *head = *temp;
    }
    else
    {
        p = head;
        while (p->next != NULL)
        {
            p = p->next;
        }
        p->next = temp;
    }
}

LinkedList *arrInit(int *arr, int length)
{
    LinkedList *head = initdef();
    for (int i = 0; i < length; i++)
    {
        set(head, *(arr + i));
    }
    return head;
}

int main()
{

    // LinkedList *head = initdef();

    // set(head, 10);
    // set(head, 20);

    // printf("%d\n", head->val);
    // printf("%d\n", head->next->val);

    int arr[3] = {2, 4, 8};

    LinkedList *l = arrInit(arr, 3);

    printf("%d\n", l->val);
    printf("%d\n", l->next->val);
    printf("%d\n", l->next->next->val);

    return 0;
}