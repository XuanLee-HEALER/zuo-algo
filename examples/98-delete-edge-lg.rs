use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let t: usize = buf.trim().parse().unwrap();
    for _ in 0..t {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let n: usize = buf.trim().parse().unwrap();
        let mut diag = Diagram::new(n, n << 1);
        for _ in 1..n {
            buf.clear();
            br.read_line(&mut buf).unwrap();
            let mut segs = buf.trim().split_whitespace();
            let p1: usize = segs.next().unwrap().parse().unwrap();
            let p2: usize = segs.next().unwrap().parse().unwrap();
            diag.add_edge(p1, p2);
            diag.add_edge(p2, p1);
        }
        let mut res_i = 0;
        let mut res = [0; 2];
        let mut node_size = vec![0; n + 1];
        let mut max_sub = vec![0; n + 1];
        diag.dfs1(1, 0, n, &mut node_size, &mut max_sub);
        let cmp = n >> 1;
        for (i, &mx) in max_sub.iter().enumerate().skip(1) {
            if mx <= cmp {
                res[res_i] = i;
                res_i += 1;
            }
        }
        if res_i == 2 {
            let (ln_p, ln) = diag.leave_node(res[0], res[1]);
            bw.write_fmt(format_args!("{} {}\n{} {}\n", ln_p, ln, res[1], ln))
                .unwrap()
        } else {
            let cn = diag.conn_node(res[0]);
            bw.write_fmt(format_args!("{} {}\n{} {}\n", res[0], cn, res[0], cn))
                .unwrap()
        }
    }
    bw.flush().unwrap()
}

struct Diagram {
    head: Vec<usize>,
    next: Vec<usize>,
    to: Vec<usize>,
    cnt: usize,
}

impl Diagram {
    fn new(n: usize, m: usize) -> Self {
        Self {
            head: vec![0; n + 1],
            next: vec![0; m + 1],
            to: vec![0; m + 1],
            cnt: 1,
        }
    }

    fn add_edge(&mut self, p1: usize, p2: usize) {
        self.next[self.cnt] = self.head[p1];
        self.head[p1] = self.cnt;
        self.to[self.cnt] = p2;
        self.cnt += 1;
    }

    fn dfs1(
        &self,
        root: usize,
        par: usize,
        tn: usize,
        node_size: &mut [usize],
        max_sub: &mut [usize],
    ) {
        node_size[root] = 1;
        let mut nxt = self.head[root];
        while nxt != 0 {
            let n = self.to[nxt];
            if n != par {
                self.dfs1(n, root, tn, node_size, max_sub);
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
    }

    fn conn_node(&self, node: usize) -> usize {
        self.to[self.head[node]]
    }

    fn leave_node(&self, root: usize, par: usize) -> (usize, usize) {
        let mut nxt = self.head[root];
        while nxt != 0 {
            let n = self.to[nxt];
            if n != par {
                return self.leave_node(n, root);
            }
            nxt = self.next[nxt]
        }
        (par, root)
    }
}
