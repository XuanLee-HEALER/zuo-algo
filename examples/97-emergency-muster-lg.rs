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
    let mut tree = Tree::new(n, n << 1);
    for _ in 1..n {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let p1: usize = segs.next().unwrap().parse().unwrap();
        let p2: usize = segs.next().unwrap().parse().unwrap();
        tree.diag.add_edge(p1, p2);
        tree.diag.add_edge(p2, p1);
    }
    tree.build();
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let a: usize = segs.next().unwrap().parse().unwrap();
        let b: usize = segs.next().unwrap().parse().unwrap();
        let c: usize = segs.next().unwrap().parse().unwrap();
        let (p, m) = tree.q(a, b, c);
        bw.write_fmt(format_args!("{} {}\n", p, m)).unwrap()
    }
    bw.flush().unwrap()
}

fn least_pow2(n: usize) -> usize {
    let mut res = 0;
    while 1 << res <= n >> 1 {
        res += 1
    }
    res
}

pub struct Diagram {
    pub head: Vec<usize>,
    pub next: Vec<usize>,
    pub to: Vec<usize>,
    cnt: usize,
}

impl Diagram {
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            head: vec![0; n + 1],
            next: vec![0; m + 1],
            to: vec![0; m + 1],
            cnt: 1,
        }
    }

    pub fn add_edge(&mut self, p1: usize, p2: usize) {
        self.next[self.cnt] = self.head[p1];
        self.head[p1] = self.cnt;
        self.to[self.cnt] = p2;
        self.cnt += 1;
    }
}

struct Tree {
    diag: Diagram,
    deep: Vec<usize>,
    st: Vec<Vec<usize>>,
}

impl Tree {
    fn new(n: usize, m: usize) -> Self {
        Self {
            diag: Diagram::new(n, m),
            deep: vec![0; n + 1],
            st: vec![vec![0; least_pow2(n) + 1]; n + 1],
        }
    }

    fn build(&mut self) {
        self.dfs(1, 0);
    }

    fn dfs(&mut self, root: usize, par: usize) {
        self.deep[root] = self.deep[par] + 1;
        self.st[root][0] = par;
        let mm = self.st[root].len();
        for j in 1..mm {
            self.st[root][j] = self.st[self.st[root][j - 1]][j - 1]
        }
        let mut nxt = self.diag.head[root];
        while nxt != 0 {
            if self.diag.to[nxt] != par {
                self.dfs(self.diag.to[nxt], root);
            }
            nxt = self.diag.next[nxt]
        }
    }

    fn lca(&self, a: usize, b: usize) -> (usize, usize) {
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
            return (a, self.deep[a]);
        }
        for j in (0..mm).rev() {
            if self.st[a][j] != self.st[b][j] {
                a = self.st[a][j];
                b = self.st[b][j]
            }
        }
        (self.st[a][0], self.deep[self.st[a][0]])
    }

    fn q(&self, a: usize, b: usize, c: usize) -> (usize, usize) {
        let (da, db, dc) = (self.deep[a], self.deep[b], self.deep[c]);
        let (lca1, d1) = self.lca(a, b);
        let (lca2, d2) = self.lca(b, c);
        let (lca3, d3) = self.lca(a, c);
        if lca1 == lca2 && lca2 == lca3 {
            // a,b,c的lca一样，三个点走到lca就行
            (lca1, da + db + dc - 3 * d1)
        } else {
            // a b的lca和a c的lca一样，那么a一定是远的，否则a b和b c的一样，那么b一定是最远的，否则c是最远的
            let r = if lca1 == lca3 {
                (lca2, d2 + 2 * d1)
            } else if lca1 == lca2 {
                (lca3, d3 + 2 * d2)
            } else {
                (lca1, d1 + 2 * d3)
            };
            (r.0, da + db + dc - r.1)
        }
    }
}
