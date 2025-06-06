use std::{
    io::{self, BufRead, BufReader, BufWriter, Write},
    usize,
};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    let mut node_v: Vec<usize> = vec![0; n + 1];
    let mut tn = 0;
    for i in 1..=n {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        node_v[i] = buf.trim().parse().unwrap();
        tn += node_v[i];
    }
    let mut diag = Diagram::new(n, n << 1);
    for _ in 1..n {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split_whitespace();
        let p1: usize = segs.next().unwrap().parse().unwrap();
        let p2: usize = segs.next().unwrap().parse().unwrap();
        let w: usize = segs.next().unwrap().parse().unwrap();
        diag.add_edge(p1, p2, w);
        diag.add_edge(p2, p1, w);
    }
    // 求出重心
    let mut node_size = vec![0; n + 1];
    let mut max_sub = vec![0; n + 1];
    let mut cur = usize::MAX;
    let mut res = 0;
    diag.dfs1(
        1,
        0,
        tn,
        &node_v,
        &mut node_size,
        &mut max_sub,
        &mut cur,
        &mut res,
    );
    // 求出所有点到重心的距离
    let mut dis = vec![0; n + 1];
    diag.dfs2(res, 0, &mut dis);
    let mut res = 0;
    for (i, &d) in dis.iter().enumerate().skip(1) {
        res += d * node_v[i]
    }
    bw.write_fmt(format_args!("{}\n", res)).unwrap();
    bw.flush().unwrap()
}

struct Diagram {
    head: Vec<usize>,
    next: Vec<usize>,
    to: Vec<usize>,
    weight: Vec<usize>,
    cnt: usize,
}

impl Diagram {
    fn new(n: usize, m: usize) -> Self {
        Self {
            head: vec![0; n + 1],
            next: vec![0; m + 1],
            to: vec![0; m + 1],
            weight: vec![0; m + 1],
            cnt: 1,
        }
    }

    fn add_edge(&mut self, p1: usize, p2: usize, w: usize) {
        self.next[self.cnt] = self.head[p1];
        self.head[p1] = self.cnt;
        self.to[self.cnt] = p2;
        self.weight[self.cnt] = w;
        self.cnt += 1;
    }

    fn dfs1(
        &self,
        root: usize,
        par: usize,
        tn: usize,
        node_val: &[usize],
        node_size: &mut [usize],
        max_sub: &mut [usize],
        cur: &mut usize,
        res: &mut usize,
    ) {
        node_size[root] = node_val[root];
        let mut nxt = self.head[root];
        while nxt != 0 {
            let n = self.to[nxt];
            if n != par {
                self.dfs1(n, root, tn, node_val, node_size, max_sub, cur, res);
                node_size[root] += node_size[n];
                if max_sub[root] < node_size[n] {
                    max_sub[root] = node_size[n]
                }
            }
            nxt = self.next[nxt]
        }
        let rem = tn - node_size[root];
        if max_sub[root] < rem {
            max_sub[root] = rem
        }
        if max_sub[root] < *cur {
            *cur = max_sub[root];
            *res = root
        }
    }

    fn dfs2(&self, root: usize, par: usize, res: &mut [usize]) {
        let mut nxt = self.head[root];
        while nxt != 0 {
            let n = self.to[nxt];
            if n != par {
                res[n] = self.weight[nxt] + res[root];
                self.dfs2(n, root, res);
            }
            nxt = self.next[nxt]
        }
    }
}
