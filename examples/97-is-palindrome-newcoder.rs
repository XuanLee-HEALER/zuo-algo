use std::io::{self, BufRead, BufReader, BufWriter, Write};

const K: usize = 499;

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    let mut sol = Solution::new(n);
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let mut node_v = vec![0; n + 1];
    buf.trim()
        .chars()
        .map(|c| (c as u8 - b'a') as usize + 1)
        .enumerate()
        .for_each(|(i, u)| node_v[i + 1] = u);
    buf.clear();
    br.read_line(&mut buf).unwrap();
    buf.trim()
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .enumerate()
        .skip(1)
        .for_each(|(i, v)| {
            sol.add_edge(v, i + 1);
        });
    let mut k_pow = vec![1; n + 1];
    (1..=n).for_each(|i| k_pow[i] = K.wrapping_mul(k_pow[i - 1]));
    sol.build(&node_v, &k_pow, 1, 0);
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let q: usize = buf.trim().parse().unwrap();
    for _ in 0..q {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split_whitespace();
        let p1: usize = segs.next().unwrap().parse().unwrap();
        let p2: usize = segs.next().unwrap().parse().unwrap();
        bw.write_fmt(format_args!("{}\n", sol.query(&node_v, &k_pow, p1, p2)))
            .unwrap()
    }
    bw.flush().unwrap()
}

fn least_pow2(n: usize) -> usize {
    let mut res = 0;
    while 1 << res <= n >> 1 {
        res += 1;
    }
    res
}

struct Solution {
    head: Vec<usize>,
    next: Vec<usize>,
    to: Vec<usize>,
    cnt: usize,
    deep: Vec<usize>,
    st: Vec<Vec<usize>>,
    // 从i点向上跳x步的哈希值，不包含结束点
    stup: Vec<Vec<usize>>,
    // 跳x步到i点的哈希值，不包含起始点
    stdown: Vec<Vec<usize>>,
}

impl Solution {
    fn new(n: usize) -> Self {
        let head = vec![0; n + 1];
        let next = vec![0; n + 1];
        let to = vec![0; n + 1];
        let cnt = 1;
        let deep = vec![0; n + 1];
        let mm = least_pow2(n);
        let st = vec![vec![0; mm + 1]; n + 1];
        let stup = vec![vec![0; mm + 1]; n + 1];
        let stdown = vec![vec![0; mm + 1]; n + 1];
        Self {
            head,
            next,
            to,
            cnt,
            deep,
            st,
            stup,
            stdown,
        }
    }

    fn add_edge(&mut self, p1: usize, p2: usize) {
        self.next[self.cnt] = self.head[p1];
        self.head[p1] = self.cnt;
        self.to[self.cnt] = p2;
        self.cnt += 1;
    }

    fn build(&mut self, node_v: &[usize], kpow: &[usize], root: usize, par: usize) {
        self.deep[root] = self.deep[par] + 1;
        self.st[root][0] = par;
        self.stup[root][0] = node_v[par];
        self.stdown[root][0] = node_v[par];
        let mm = self.st[0].len();
        for j in 1..mm {
            self.st[root][j] = self.st[self.st[root][j - 1]][j - 1];
            self.stup[root][j] = self.stup[root][j - 1] * kpow[1 << (j - 1)]
                + self.stup[self.st[root][j - 1]][j - 1];
            self.stdown[root][j] = self.stdown[self.st[root][j - 1]][j - 1] * kpow[1 << (j - 1)]
                + self.stdown[root][j - 1]
        }
        let mut nxt = self.head[root];
        while nxt != 0 {
            self.build(node_v, kpow, self.to[nxt], root);
            nxt = self.next[nxt]
        }
    }

    fn lca(&self, a: usize, b: usize) -> usize {
        let (mut a, mut b) = if self.deep[a] < self.deep[b] {
            (b, a)
        } else {
            (a, b)
        };
        let mm = self.st[0].len();
        for j in (0..mm).rev() {
            if self.deep[self.st[a][j]] >= self.deep[b] {
                a = self.st[a][j]
            }
        }
        if a == b {
            a
        } else {
            for j in (0..mm).rev() {
                if self.st[a][j] != self.st[b][j] {
                    a = self.st[a][j];
                    b = self.st[b][j]
                }
            }
            self.st[a][0]
        }
    }

    fn hash(&self, node_v: &[usize], kpow: &[usize], beg: usize, lca: usize, end: usize) -> usize {
        let mm = self.st[0].len();
        let mut t_beg = beg;
        let mut up: usize = node_v[t_beg];
        for j in (0..mm).rev() {
            if self.deep[self.st[t_beg][j]] >= self.deep[lca] {
                up = up
                    .wrapping_mul(kpow[1 << j])
                    .wrapping_add(self.stup[t_beg][j]);
                t_beg = self.st[t_beg][j];
            }
        }
        if end == lca {
            return up;
        }
        t_beg = end;
        let mut down = node_v[t_beg];
        let mut height = 1;
        if end != lca {
            for j in (0..mm).rev() {
                if self.deep[self.st[t_beg][j]] > self.deep[lca] {
                    down = self.stdown[t_beg][j]
                        .wrapping_mul(kpow[height])
                        .wrapping_add(down);
                    height += 1 << j;
                    t_beg = self.st[t_beg][j]
                }
            }
        }
        up.wrapping_mul(kpow[height]).wrapping_add(down)
    }

    fn query(&self, node_v: &[usize], kpow: &[usize], a: usize, b: usize) -> &'static str {
        let lca = self.lca(a, b);
        if self.hash(node_v, kpow, a, lca, b) == self.hash(node_v, kpow, b, lca, a) {
            "YES"
        } else {
            "NO"
        }
    }
}
