#include <stdio.h>
#include <math.h>

// 输入：
// - 第一行包含两个数 n 和 X 。表示游戏个数和预算
// - 接下来 n 行包含每个游戏的信息，原价 ai,现价 bi，能获得的快乐值为 wi 。
// 输出：
// - 输出一个数字，表示你能获得的最大快乐值。
// 1 <= n <= 500
// 0 <= x <= 10,000
// 0 <= b_i <= a_i <= 500
// 1 <= w_i <= 1,000,000,000

#define MAX(a, b) ((a) > (b) ? (a) : (b))
#define ARR_LEN 500

long compute(int discount[], long happy[], int budget, int n);

int main(int argc, char const *argv[])
{
    int n;
    long x;
    int discount[ARR_LEN];
    long happy[ARR_LEN];
    while (EOF != scanf("%d %ld", &n, &x))
    {
        int k = 0;
        long ans = 0;
        for (int i = 1; i <= n; i++)
        {
            int ai, bi;
            long wi;
            scanf("%d %d %ld", &ai, &bi, &wi);
            int well = ai - bi - bi;
            if (well >= 0)
            {
                x += well;
                ans += wi;
            }
            else
            {
                discount[k] = -well;
                happy[k++] = wi;
            }
        }
        printf("%ld\n", ans + compute(discount, happy, x, k));
    }
}

long compute(int discount[], long happy[], int budget, int n)
{
    long dp[budget + 1];
    for (int i = 0; i <= budget; i++)
    {
        dp[i] = 0;
    }
    for (int i = 1; i <= n; i++)
    {
        for (int j = budget; j >= discount[i - 1]; j--)
        {
            dp[j] = MAX(dp[j], happy[i - 1] + dp[j - discount[i - 1]]);
        }
    }

    return dp[budget];
}