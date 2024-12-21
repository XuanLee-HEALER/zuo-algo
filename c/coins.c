#include <stdio.h>

#define MAXN 101
#define MAXM 100001

int dp[MAXM], costs[MAXN], nums[MAXN];

int min(int a, int b)
{
    return a > b ? b : a;
}

int compute(int *costs, int *nums, int n, int limit)
{
    for (int i = 0; i <= limit; ++i)
    {
        dp[i] = 0;
    }

    dp[0] = 1;
    for (int i = 0; i < n; ++i)
    {
        if (nums[i] == 1)
        {
            for (int j = limit; j >= costs[i]; --j)
            {
                if (dp[j - costs[i]])
                {
                    dp[j] = 1;
                }
            }
        }
        else if (costs[i] * nums[i] > limit)
        {
            // 只有大于才是完全背包，该物品可以无限选择
            for (int j = costs[i]; j <= limit; ++j)
            {
                if (dp[j - costs[i]])
                {
                    dp[j] = 1;
                }
            }
        }
        else
        {
            // 多重背包
            for (int mod = 0; mod < costs[i]; ++mod)
            {
                int true_cnt = 0;
                for (int cnt = 0, k = limit - mod; cnt <= nums[i] && k >= 0; ++cnt, k -= costs[i])
                {
                    true_cnt += dp[k] ? 1 : 0;
                }
                for (int k = limit - mod, enter = k - (costs[i] * (nums[i] + 1)); k >= 1; k -= costs[i], enter -= costs[i])
                {
                    if (dp[k])
                    {
                        // 先移出最右的元素，也就是自己
                        true_cnt -= 1;
                    }
                    else
                    {
                        // 只要不是0就表示这个位置是true
                        if (true_cnt != 0)
                        {
                            dp[k] = 1;
                        }
                    }
                    if (enter >= 0)
                    {
                        true_cnt += dp[enter] ? 1 : 0;
                    }
                }
            }
        }
    }
    int res = 0;
    for (int j = 1; j <= limit; j++)
    {
        if (dp[j])
        {
            res++;
        }
    }
    return res;
}

int main(void)
{
    int n, m;
    for (int i = 0; i < MAXN; i++)
    {
        costs[i] = 0;
        nums[i] = 0;
    }
    while (EOF != scanf("%d %d", &n, &m) && n + m > 0)
    {
        for (int i = 0; i < n; ++i)
        {
            scanf("%d", &costs[i]);
        }

        for (int i = 0; i < n; ++i)
        {
            scanf("%d", &nums[i]);
        }

        printf("%d\n", compute(costs, nums, n, m));
    }

    return 0;
}