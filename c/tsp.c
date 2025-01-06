#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>

int compute(int **graph, int n, int status, int idx, int **dp)
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
        for (int i = 1; i < n; i++)
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
        return res;
    }
}

int main()
{
    int n;
    scanf("%d", &n);

    int **graph = (int **)malloc(n * sizeof(int *));
    for (int i = 0; i < n; i++)
    {
        graph[i] = (int *)malloc(n * sizeof(int));
    }

    for (int i = 0; i < n; i++)
    {
        for (int j = 0; j < n; j++)
        {
            scanf("%d", &graph[i][j]);
        }
    }

    int **dp = (int **)malloc((1 << n) * sizeof(int *));
    for (int i = 0; i < (1 << n); i++)
    {
        dp[i] = (int *)calloc(n - 1, sizeof(int));
    }

    int res = INT_MAX;
    printf("%d\n", compute(graph, n, 1, 0, dp));

    // Free allocated memory
    for (int i = 0; i < n; i++)
    {
        free(graph[i]);
    }
    free(graph);
    for (int i = 0; i < (1 << n); i++)
    {
        free(dp[i]);
    }
    free(dp);

    return 0;
}