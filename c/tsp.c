#include <stdio.h>
#include <limits.h>
#include <stdlib.h>

#define MAXN 20
#define STATUS_LEN 1 << 20

int compute(int graph[MAXN][MAXN], int n, int status, int idx, int **dp)
{
    if (status == (1 << n) - 1)
    {
        return graph[idx][0];
    }
    else if (dp[status][idx] != 0)
    {
        return dp[status][idx];
    }
    else
    {
        int res = INT_MAX;
        for (int i = 1; i < n; ++i)
        {
            if ((status & (1 << i)) == 0)
            {
                int temp = graph[idx][i] + compute(graph, n, status | (1 << i), i, dp);
                if (temp < res)
                {
                    res = temp;
                }
            }
        }
        dp[status][idx] = res;
        return dp[status][idx];
    }
}

int main(int argc, char const *argv[])
{
    int n;
    scanf("%d", &n);
    int graph[MAXN][MAXN];
    for (int i = 0; i < n; ++i)
    {
        for (int j = 0; j < n; ++j)
        {
            scanf("%d", &graph[i][j]);
        }
    }
    int **dp = (int **)calloc(STATUS_LEN, sizeof(int *));
    for (int i = 0; i < STATUS_LEN; ++i)
    {
        dp[i] = (int *)calloc(MAXN, sizeof(int));
    }
    printf("%d\n", compute(graph, n, 1, 0, dp));

    for (int i = 0; i < STATUS_LEN; ++i)
    {
        free(dp[i]);
    }
    free(dp);
    return 0;
}