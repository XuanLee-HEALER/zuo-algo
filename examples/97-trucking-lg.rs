use std::{
    io::{self, BufRead, BufReader, BufWriter, Write},
    usize,
};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut segs = buf.trim().split_whitespace();
    let n: usize = segs.next().unwrap().parse().unwrap();
    let m: usize = segs.next().unwrap().parse().unwrap();
    let mut edges = Vec::with_capacity(m);
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let p1: usize = segs.next().unwrap().parse().unwrap();
        let p2: usize = segs.next().unwrap().parse().unwrap();
        let w: usize = segs.next().unwrap().parse().unwrap();
        edges.push((p1, p2, w));
    }
    // 最大连通图
    let mut k_diag = Kruskal::new(n, m);
    k_diag.build_max_tree(&mut edges);
    for i in 1..=n {
        if !k_diag.visited[i] {
            k_diag.dfs(i, 0, 0);
        }
    }
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let q: usize = buf.trim().parse().unwrap();
    for _ in 0..q {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let p1: usize = segs.next().unwrap().parse().unwrap();
        let p2: usize = segs.next().unwrap().parse().unwrap();
        bw.write_fmt(format_args!(
            "{}\n",
            if k_diag.find(p1) != k_diag.find(p2) {
                -1
            } else {
                k_diag.query(p1, p2) as i32
            }
        ))
        .unwrap()
    }
    bw.flush().unwrap()
}

fn least_2pow(u: usize) -> usize {
    let mut res = 0;
    while 1 << res <= u >> 1 {
        res += 1
    }
    res
}

struct Kruskal {
    father: Vec<usize>,
    visited: Vec<bool>,
    head: Vec<usize>,
    next: Vec<usize>,
    to: Vec<usize>,
    weight: Vec<usize>,
    cnt: usize,
    deep: Vec<usize>,
    st: Vec<Vec<usize>>,
    st_min: Vec<Vec<usize>>,
}

impl Kruskal {
    fn new(n: usize, m: usize) -> Self {
        let father: Vec<usize> = (0..=n).collect();
        let visited = vec![false; n + 1];
        let head = vec![0; n + 1];
        let next = vec![0; m << 1 + 1];
        let to = vec![0; m << 1 + 1];
        let weight = vec![0; m << 1 + 1];
        let cnt = 1;
        let mm = least_2pow(n);
        let deep = vec![0; n + 1];
        let st = vec![vec![0; mm + 1]; n + 1];
        let st_min = vec![vec![usize::MAX; mm + 1]; n + 1];
        Self {
            father,
            visited,
            head,
            next,
            to,
            weight,
            cnt,
            deep,
            st,
            st_min,
        }
    }

    fn find(&mut self, p1: usize) -> usize {
        if p1 != self.father[p1] {
            self.father[p1] = self.find(self.father[p1])
        }
        self.father[p1]
    }

    fn build_max_tree(&mut self, edges: &mut [(usize, usize, usize)]) {
        edges.sort_unstable_by(|a, b| a.2.cmp(&b.2).reverse());
        for &(p1, p2, w) in edges.iter() {
            let (f1, f2) = (self.find(p1), self.find(p2));
            if f1 != f2 {
                self.add_edge(p1, p2, w);
                self.add_edge(p2, p1, w);
                self.father[f1] = f2;
            }
        }
    }

    fn add_edge(&mut self, p1: usize, p2: usize, w: usize) {
        self.next[self.cnt] = self.head[p1];
        self.head[p1] = self.cnt;
        self.to[self.cnt] = p2;
        self.weight[self.cnt] = w;
        self.cnt += 1;
    }

    fn dfs(&mut self, root: usize, par: usize, w: usize) {
        self.visited[root] = true;
        self.deep[root] = if par == 0 {
            self.st[root][0] = root;
            1
        } else {
            self.st[root][0] = par;
            self.st_min[root][0] = w;
            self.deep[par] + 1
        };
        let mm = self.st[0].len();
        for j in 1..mm {
            self.st[root][j] = self.st[self.st[root][j - 1]][j - 1];
            self.st_min[root][j] =
                self.st_min[root][j - 1].min(self.st_min[self.st[root][j - 1]][j - 1])
        }
        let mut nxt = self.head[root];
        while nxt != 0 {
            if !self.visited[self.to[nxt]] {
                self.dfs(self.to[nxt], root, self.weight[nxt]);
            }
            nxt = self.next[nxt]
        }
    }

    fn query(&self, p1: usize, p2: usize) -> usize {
        let (mut p1, mut p2) = if self.deep[p1] < self.deep[p2] {
            (p2, p1)
        } else {
            (p1, p2)
        };
        let mut res = usize::MAX;
        let mm = self.st[0].len();
        for j in (0..mm).rev() {
            if self.deep[self.st[p1][j]] >= self.deep[p2] {
                res = res.min(self.st_min[p1][j]);
                p1 = self.st[p1][j]
            }
        }
        if p1 != p2 {
            for j in (0..mm).rev() {
                if self.st[p1][j] != self.st[p2][j] {
                    res = res.min(self.st_min[p1][j]).min(self.st_min[p2][j]);
                    p1 = self.st[p1][j];
                    p2 = self.st[p2][j];
                }
            }
            res = res.min(self.st_min[p1][0]).min(self.st_min[p2][0])
        }
        res
    }
}
