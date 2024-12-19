#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <string.h>

#define MAX_SIZE int[1000001]
#define BUFFER_SIZE 4096

const int SIZE = 10;

typedef struct
{
    int left;
    int right;
} Boundary;

void quick_sort(int *, int, int);
int partition1(int *, int, int, int);
void swap(int *, int, int);
void print_arr(int *, int);
Boundary partition2(int *, int, int, int);

int main()
{
    // 使用当前时间来设置随机数种子，这样每次运行生成的随机数不同
    srand(time(NULL));

    int arr[SIZE] = {10, 3, 3, 3, 3, 3, 3, 3, 2, 1};
    quick_sort(arr, 0, SIZE - 1);
    print_arr(arr, SIZE);

    return 0;
}

void print_arr(int *arr, int length)
{
    for (int i = 0; i < length; i++)
    {
        if (i != length - 1)
            printf("%d ", arr[i]);
        else
            printf("%d", arr[i]);
    }
    printf("\n");
}

void quick_sort(int *arr, int l, int r)
{
    if (l < r)
    {
        int v = arr[l + rand() % (r - l + 1)];
        // int ti = partition1(arr, l, r, v);
        Boundary tb = partition2(arr, l, r, v);
        quick_sort(arr, l, tb.left - 1);
        quick_sort(arr, tb.right + 1, r);
    }
}

Boundary partition2(int *arr, int l, int r, int x)
{
    int a = l;
    int b = r;
    for (int i = 0; i <= r;)
    {
        if (arr[i] < arr[a])
        {
            swap(arr, i++, a++);
        }
        else if (arr[i] == arr[a])
        {
            i++;
        }
        else
        {
            swap(arr, i, b--);
        }
    }

    Boundary res = {a,
                    b};
    return res;
}

int partition1(int *arr, int l, int r, int x)
{
    int a = l;
    int ti = 0;
    for (int i = l; i <= r; i++)
    {
        if (arr[i] <= x)
        {
            swap(arr, a, i);
            if (arr[a] == x)
            {
                ti = a;
            }
            a++;
        }
    }
    swap(arr, ti, a - 1);
    return a - 1;
}

inline void swap(int *arr, int a, int b)
{
    int tmp = arr[b];
    arr[b] = arr[a];
    arr[a] = tmp;
}