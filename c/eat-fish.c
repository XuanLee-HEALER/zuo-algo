#include <stdio.h>

#define ARR_LEN 100001
#define INIT_VALUE 0;

int fishes[ARR_LEN];
int stack[ARR_LEN][2];
int top = 0;

int calculate(int);
int max(int, int);

int main()
{
    int n;
    while (scanf("%d", &n) != EOF)
    {
        top = 0;
        int weight;
        for (int i = 0; i < n; i++)
        {
            scanf("%d", &weight);
            fishes[i] = weight;
        }
        int res = calculate(n);
        printf("%d\n", res);
    }
}

int calculate(int n)
{
    int ans = 0;
    for (n = n - 1; n >= 0; n--)
    {
        int cur_turn = 0;
        while (top > 0 && fishes[n] > stack[top - 1][0])
        {
            cur_turn = max(cur_turn + 1, stack[--top][1]);
        }
        ans = max(ans, cur_turn);
        stack[top][0] = fishes[n];
        stack[top][1] = cur_turn;
        top++;
    }
    return ans;
}

int max(int a, int b)
{
    return a >= b ? a : b;
}