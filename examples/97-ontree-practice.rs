fn main() {
    // let ta = TreeAncestor::new(7, vec![-1, 0, 0, 1, 1, 2, 2]);
    // println!("res: [3,1] {}", ta.get_kth_ancestor(3, 1));
    // println!("res: [5,2] {}", ta.get_kth_ancestor(5, 2));
    // println!("res: [6,3] {}", ta.get_kth_ancestor(6, 3));
    println!(
        "res: {:?}",
        Solution::min_operations_queries(
            7,
            vec![
                vec![0, 1, 1],
                vec![1, 2, 1],
                vec![2, 3, 1],
                vec![3, 4, 2],
                vec![4, 5, 2],
                vec![5, 6, 2]
            ],
            vec![vec![0, 3], vec![3, 6], vec![2, 6], vec![0, 6]]
        )
    );
    println!(
        "res: {:?}",
        Solution::min_operations_queries(
            8,
            vec![
                vec![1, 2, 6],
                vec![1, 3, 4],
                vec![2, 4, 6],
                vec![2, 5, 3],
                vec![3, 6, 6],
                vec![3, 0, 8],
                vec![7, 0, 2]
            ],
            vec![vec![4, 6], vec![0, 4], vec![6, 5], vec![7, 4]]
        )
    );
    println!("res {}", Solution::get_max_function_value(vec![2, 0, 1], 4));
    println!(
        "res {}",
        Solution::get_max_function_value(vec![1, 1, 1, 2, 3], 3)
    );
    println!(
        "res {}",
        Solution::get_max_function_value(vec![1, 0], 10000000000)
    );
}

struct Solution;

impl Solution {
    pub fn min_operations_queries(
        n: i32,
        edges: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let m = n - 1;
        let qm = queries.len();
        let mut d_head = vec![0; n];
        let mut d_next = vec![0; m << 1 + 1];
        let mut d_to = vec![0; m << 1 + 1];
        let mut d_weight = vec![0; m << 1 + 1];
        let mut visited = vec![false; n];
        let mut q_head = vec![0; n];
        let mut q_next = vec![0; qm << 1 + 1];
        let mut q_to = vec![0; qm << 1 + 1];
        let mut q_weight = vec![0; qm << 1 + 1];
        let mut cnt = 1;
        let mut father: Vec<usize> = (0..n).collect();
        let mut weight_count = vec![[0; 27]; n];
        let mut res = vec![0; qm];
        for edge in &edges {
            Self::add_edge(
                &mut d_head,
                &mut d_next,
                &mut d_to,
                &mut d_weight,
                &mut cnt,
                edge[0] as usize,
                edge[1] as usize,
                edge[2] as usize,
            );
            Self::add_edge(
                &mut d_head,
                &mut d_next,
                &mut d_to,
                &mut d_weight,
                &mut cnt,
                edge[1] as usize,
                edge[0] as usize,
                edge[2] as usize,
            );
        }
        Self::dfs(
            &mut weight_count,
            &d_head,
            &d_next,
            &d_to,
            &d_weight,
            0,
            None,
            0,
        );
        cnt = 1;
        for (i, query) in queries.iter().enumerate() {
            Self::add_edge(
                &mut q_head,
                &mut q_next,
                &mut q_to,
                &mut q_weight,
                &mut cnt,
                query[0] as usize,
                query[1] as usize,
                i,
            );
            Self::add_edge(
                &mut q_head,
                &mut q_next,
                &mut q_to,
                &mut q_weight,
                &mut cnt,
                query[1] as usize,
                query[0] as usize,
                i,
            );
        }
        Self::query(
            &mut father,
            &d_head,
            &d_next,
            &d_to,
            &q_head,
            &q_next,
            &q_to,
            &q_weight,
            &mut visited,
            &weight_count,
            0,
            None,
            &queries,
            &mut res,
        );
        res
    }

    fn dfs(
        ct: &mut [[usize; 27]],
        head: &[usize],
        next: &[usize],
        to: &[usize],
        weight: &[usize],
        root: usize,
        par: Option<usize>,
        w: usize,
    ) {
        let mut d = false;
        if let Some(par) = par {
            for i in 1..=26 {
                ct[root][i] = ct[par][i];
            }
            ct[root][w] += 1;
        } else {
            d = true
        }
        let mut nxt = head[root];
        while nxt != 0 {
            let nn = to[nxt];
            if d || nn != par.unwrap() {
                Self::dfs(ct, head, next, to, weight, nn, Some(root), weight[nxt]);
            }
            nxt = next[nxt];
        }
    }

    fn add_edge(
        head: &mut [usize],
        next: &mut [usize],
        to: &mut [usize],
        weight: &mut [usize],
        cnt: &mut usize,
        p1: usize,
        p2: usize,
        w: usize,
    ) {
        next[*cnt] = head[p1];
        head[p1] = *cnt;
        to[*cnt] = p2;
        weight[*cnt] = w;
        *cnt += 1;
    }

    fn find(father: &mut [usize], i: usize) -> usize {
        if i != father[i] {
            father[i] = Self::find(father, father[i])
        }
        father[i]
    }

    fn query(
        father: &mut [usize],
        d_head: &[usize],
        d_next: &[usize],
        d_to: &[usize],
        q_head: &[usize],
        q_next: &[usize],
        q_to: &[usize],
        q_w: &[usize],
        visited: &mut [bool],
        ct: &[[usize; 27]],
        root: usize,
        par: Option<usize>,
        query: &[Vec<i32>],
        res: &mut [i32],
    ) {
        visited[root] = true;
        let mut nxt = d_head[root];
        while nxt != 0 {
            if par.is_none() || d_to[nxt] != par.unwrap() {
                Self::query(
                    father,
                    d_head,
                    d_next,
                    d_to,
                    q_head,
                    q_next,
                    q_to,
                    q_w,
                    visited,
                    ct,
                    d_to[nxt],
                    Some(root),
                    query,
                    res,
                );
                father[d_to[nxt]] = root
            }
            nxt = d_next[nxt];
        }
        nxt = q_head[root];
        while nxt != 0 {
            let sub = q_to[nxt];
            let idx = q_w[nxt];
            if visited[sub] {
                let lca = Self::find(father, sub);
                let p1 = query[idx][0] as usize;
                let p2 = query[idx][1] as usize;
                let mut all_cnt = 0;
                let mut max_cnt = 0;
                for u in 1..=26 {
                    let c_ct = ct[p1][u] + ct[p2][u] - 2 * ct[lca][u];
                    all_cnt += c_ct;
                    max_cnt = max_cnt.max(c_ct);
                }
                res[idx] = (all_cnt - max_cnt) as i32;
            }
            nxt = q_next[nxt]
        }
    }

    pub fn get_max_function_value(receiver: Vec<i32>, k: i64) -> i64 {
        let n = receiver.len();
        let m = least_pow2(k as usize);
        let mut st = vec![vec![0; m + 1]; n];
        let mut st_sum = vec![vec![0; m + 1]; n];
        for (i, &r) in receiver.iter().enumerate() {
            st[i][0] = r;
            st_sum[i][0] = r as i64;
        }
        // ⚠️先处理完jump表才能更新编号累加和的表，先列再行！
        for j in 1..=m {
            for i in 0..n {
                st[i][j] = st[st[i][j - 1] as usize][j - 1];
                st_sum[i][j] = st_sum[i][j - 1] + st_sum[st[i][j - 1] as usize][j - 1];
            }
        }
        let mut k_bits = Vec::new();
        let mut k = k;
        for m in (0..=m).rev() {
            let cur = 1 << m as i64;
            if k >= cur {
                k_bits.push(m);
                k -= cur
            }
        }
        let mut res = 0;
        for i in 0..n {
            let mut cur = i as i64;
            let mut beg = i;
            for &b in &k_bits {
                cur += st_sum[beg][b] as i64;
                beg = st[beg][b] as usize;
            }
            res = res.max(cur as i64)
        }
        res
    }
}

struct Tree {
    head: Vec<usize>,
    next: Vec<usize>,
    to: Vec<usize>,
}

impl Tree {
    fn new(n: usize, nodes: &[i32]) -> Self {
        let mut head = vec![0; n];
        let mut next = vec![0; n];
        let mut to = vec![0; n];
        let mut cnt = 1;
        for (i, &p) in nodes.iter().enumerate().skip(1) {
            // p->i
            let p = p as usize;
            next[cnt] = head[p];
            head[p] = cnt;
            to[cnt] = i;
            cnt += 1;
        }
        Self { head, next, to }
    }

    fn update(
        &self,
        deep: &mut [usize],
        st: &mut [Vec<usize>],
        cur: usize,
        parent: Option<usize>,
        col: usize,
    ) {
        // 每个节点的层级，当前更新的是某个点的层级
        // 然后更新子节点的层级
        // 同时更新这个节点的st表
        if let Some(p) = parent {
            deep[cur] = deep[p] + 1;
            st[cur][0] = p;
            for j in 1..=col {
                st[cur][j] = st[st[cur][j - 1]][j - 1]
            }
        } else {
            // 根节点
            deep[cur] = 1;
        }
        let mut sub = self.head[cur];
        while sub != 0 {
            self.update(deep, st, self.to[sub], Some(cur), col);
            sub = self.next[sub]
        }
    }
}

fn least_pow2(n: usize) -> usize {
    let mut res = 0;
    let n = n >> 1;
    while 1 << res <= n {
        res += 1;
    }
    res
}

struct TreeAncestor {
    log2: usize,
    deep: Vec<usize>,
    sttree: Vec<Vec<usize>>,
}

impl TreeAncestor {
    fn new(n: i32, parent: Vec<i32>) -> Self {
        let tree = Tree::new(n as usize, &parent);
        let nn = least_pow2(n as usize) + 1;
        let mut deep = vec![0; n as usize];
        let mut sttree = vec![vec![0; nn + 1]; n as usize];

        tree.update(&mut deep, &mut sttree, 0, None, nn);
        Self {
            log2: nn,
            deep,
            sttree,
        }
    }

    fn get_kth_ancestor(&self, node: i32, k: i32) -> i32 {
        let node = node as usize;
        let k = k as usize;
        if self.deep[node] <= k {
            -1
        } else {
            let mut target = k;
            let mut res = node;
            for beg in (0..=self.log2).rev() {
                if 1 << beg <= target {
                    res = self.sttree[res][beg];
                    target -= 1 << beg;
                }
            }
            res as i32
        }
    }
}
