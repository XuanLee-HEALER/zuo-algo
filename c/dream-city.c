#include <stdio.h>
#include <stdlib.h>

#define MAXN 251
#define MAX(a, b) ((a) > (b) ? (a) : (b))

int dp[MAXN];

int cmp(const void *a, const void *b) {
    const int *row_a = (const int *)a;
    const int *row_b = (const int *)b;

    return row_a[1] - row_b[1];
}

int compute(int n, int m, int tree[][2]) {
    qsort(tree + 1, n, sizeof(int[2]), cmp);
    for (int i = 1; i <= n; ++i) {
        for (int j = m; j >= 1; --j) {
            dp[j] = MAX(dp[j], dp[j - 1] + tree[i][0] + (j - 1) * tree[i][1]);
        }
    }
    return dp[m];
}

int main() {
    int t;
    scanf("%d", &t);
    for (int i = 0; i < t; i++) {
        for (int i = 0; i < MAXN; i++) {
            dp[i] = 0;
        }
        int n, m;
        scanf("%d%d", &n, &m);
        int tree[n + 1][2];
        for (int i = 1; i <= n; i++) {
            scanf("%d", &tree[i][0]);
        }
        for (int i = 1; i <= n; i++) {
            scanf("%d", &tree[i][1]);
        }
        printf("%d\n", compute(n, m, tree));
    }
}
