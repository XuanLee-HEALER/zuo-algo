#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int* head;
    int* next;
    int* to;
    int cnt;
} Diag;

Diag* create_diag(int n) {
    int* head = (int*)calloc(n + 1, sizeof(int));
    int* next = (int*)calloc(((n << 1) + 1), sizeof(int));
    int* to = (int*)calloc(((n << 1) + 1), sizeof(int));
    Diag* res = (Diag*)malloc(sizeof(Diag));
    res->head = head;
    res->next = next;
    res->to = to;
    res->cnt = 1;
    return res;
}

void add_edge(Diag* diag, int p1, int p2) {
    diag->next[diag->cnt] = diag->head[p1];
    diag->head[p1] = diag->cnt;
    diag->to[diag->cnt++] = p2;
}

void free_diag(Diag* diag) {
    free(diag->head);
    free(diag->next);
    free(diag->to);
    free(diag);
}

int center = 0;
int min = 20001;

void dfs(Diag* diag, int n, int root, int par, int* node_size, int* max_sub) {
    node_size[root] = 1;
    for (int nxt = diag->head[root]; nxt != 0; nxt = diag->next[nxt]) {
        if (diag->to[nxt] != par) {
            dfs(diag, n, diag->to[nxt], root, node_size, max_sub);
            node_size[root] += node_size[diag->to[nxt]];
            if (node_size[diag->to[nxt]] > max_sub[root]) {
                max_sub[root] = node_size[diag->to[nxt]];
            }
        }
    }
    int rem = n - node_size[root];
    if (max_sub[root] < rem) {
        max_sub[root] = rem;
    }
    if (max_sub[root] < min || (max_sub[root] == min && root < center)) {
        center = root;
        min = max_sub[root];
    }
}

int main() {
    int t;
    scanf("%d", &t);
    for (int i = 0; i < t; ++i) {
        int n, p1, p2;
        scanf("%d", &n);
        Diag* diag = create_diag(n);
        for (int i = 1; i < n; i++) {
            scanf("%d %d", &p1, &p2);
            add_edge(diag, p1, p2);
            add_edge(diag, p2, p1);
        }
        // node_size 任意节点作为根节点的子树的节点数量
        int* node_size = (int*)calloc(n + 1, sizeof(int));
        // max_sub 任意节点的最大子树的节点数大小，所有这些最大的最小值所对应的节点就是重心
        int* max_sub = (int*)calloc(n + 1, sizeof(int));
        dfs(diag, n, 1, 0, node_size, max_sub);
        printf("%d %d\n", center, min);
        free(node_size);
        free(max_sub);
        free_diag(diag);
    }
}