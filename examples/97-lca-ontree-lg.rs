use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut segs = buf.trim().split_whitespace();
    let n: usize = segs.next().unwrap().parse().unwrap();
    let m: usize = segs.next().unwrap().parse().unwrap();
    let s: usize = segs.next().unwrap().parse().unwrap();
    let mut tree = Tree::new(n);
    for _ in 1..n {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let p1: usize = segs.next().unwrap().parse().unwrap();
        let p2: usize = segs.next().unwrap().parse().unwrap();
        tree.add_edge(p1, p2);
        tree.add_edge(p2, p1);
    }
    let mut deep = vec![0; n + 1];
    let mm = least_pow2(n);
    let mut st = vec![vec![0; mm + 1]; n + 1];
    // tree.update(&mut deep, &mut st, s, None, mm);
    tree.update_iter(&mut deep, &mut st, s, mm);
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let a: usize = segs.next().unwrap().parse().unwrap();
        let b: usize = segs.next().unwrap().parse().unwrap();
        bw.write_fmt(format_args!("{}\n", query(&deep, &st, mm, a, b)))
            .unwrap()
    }
    bw.flush().unwrap()
}

struct Tree {
    head: Vec<usize>,
    next: Vec<usize>,
    to: Vec<usize>,
    cnt: usize,
}

impl Tree {
    fn new(n: usize) -> Self {
        Self {
            head: vec![0; n + 1],
            next: vec![0; n << 1],
            to: vec![0; n << 1],
            cnt: 1,
        }
    }

    fn add_edge(&mut self, p1: usize, p2: usize) {
        self.next[self.cnt] = self.head[p1];
        self.head[p1] = self.cnt;
        self.to[self.cnt] = p2;
        self.cnt += 1;
    }

    fn update(
        &self,
        deep: &mut [usize],
        st: &mut [Vec<usize>],
        cur: usize,
        par: Option<usize>,
        max_col: usize,
    ) {
        if let Some(p) = par {
            deep[cur] = deep[p] + 1;
            st[cur][0] = p;
            for j in 1..=max_col {
                st[cur][j] = st[st[cur][j - 1]][j - 1]
            }
            let mut nxt = self.head[cur];
            while nxt != 0 {
                if self.to[nxt] != p {
                    self.update(deep, st, self.to[nxt], Some(cur), max_col);
                }
                nxt = self.next[nxt]
            }
        } else {
            deep[cur] = 1;
            let mut nxt = self.head[cur];
            while nxt != 0 {
                self.update(deep, st, self.to[nxt], Some(cur), max_col);
                nxt = self.next[nxt]
            }
        }
    }

    fn update_iter(&self, deep: &mut [usize], st: &mut [Vec<usize>], root: usize, max_col: usize) {
        let n = deep.len();
        let mut stack = Vec::with_capacity(n);
        stack.push((root, 0, -1));
        while !stack.is_empty() {
            let (node, par, edge) = stack.pop().unwrap();
            let nxt = if edge == -1 {
                deep[node] = deep[par] + 1;
                st[node][0] = par;
                for j in 1..=max_col {
                    st[node][j] = st[st[node][j - 1]][j - 1]
                }
                self.head[node]
            } else {
                self.next[edge as usize]
            };
            // ⚠️只要有下一条边，就要入栈，但是这条边不一定是要处理的边，因为可能会指回父节点，下一次出栈再处理下一条边
            if nxt != 0 {
                stack.push((node, par, nxt as i32));
                if self.to[nxt] != par {
                    stack.push((self.to[nxt], node, -1));
                }
            }
        }
    }
}

fn least_pow2(n: usize) -> usize {
    let n = n >> 1;
    let mut res = 0;
    while 1 << res <= n {
        res += 1;
    }
    res
}

fn query(deep: &[usize], st: &[Vec<usize>], max_col: usize, mut a: usize, mut b: usize) -> usize {
    if deep[b] > deep[a] {
        (a, b) = (b, a)
    }
    let mut cur = a;
    for j in (0..=max_col).rev() {
        if deep[st[cur][j]] >= deep[b] {
            cur = st[cur][j]
        }
    }
    if cur == b {
        cur
    } else {
        for j in (0..=max_col).rev() {
            if st[cur][j] != st[b][j] {
                // 如果还不是一个地方，更新
                (cur, b) = (st[cur][j], st[b][j])
            }
            // 不能提前break，更小的步幅可能还是错的
        }
        st[cur][0]
    }
}
