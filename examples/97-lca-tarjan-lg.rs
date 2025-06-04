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
    let mut tree = Tree::new(n, m);
    for _ in 1..n {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let p1: usize = segs.next().unwrap().parse().unwrap();
        let p2: usize = segs.next().unwrap().parse().unwrap();
        tree.add_edge(p1, p2);
        tree.add_edge(p2, p1);
    }
    let mut ans = vec![0; m];
    for i in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let a: usize = segs.next().unwrap().parse().unwrap();
        let b: usize = segs.next().unwrap().parse().unwrap();
        tree.add_query(a, b, i);
        tree.add_query(b, a, i);
    }
    // tree.query(s, 0, &mut ans);
    tree.query_iter(s, n, &mut ans);
    ans.iter()
        .for_each(|v| bw.write_fmt(format_args!("{}\n", v)).unwrap());
    bw.flush().unwrap()
}

struct Tree {
    tree_head: Vec<usize>,
    tree_next: Vec<usize>,
    tree_to: Vec<usize>,
    tree_cnt: usize,
    q_head: Vec<usize>,
    q_next: Vec<usize>,
    q_to: Vec<usize>,
    q_idx: Vec<usize>,
    q_cnt: usize,
    visited: Vec<bool>,
    father: Vec<usize>,
}

impl Tree {
    fn new(n: usize, m: usize) -> Self {
        Self {
            tree_head: vec![0; n + 1],
            tree_next: vec![0; n << 1],
            tree_to: vec![0; n << 1],
            tree_cnt: 1,
            q_head: vec![0; n + 1],
            q_next: vec![0; m << 1 | 1],
            q_to: vec![0; m << 1 | 1],
            q_idx: vec![0; m << 1 | 1],
            q_cnt: 1,
            visited: vec![false; n + 1],
            father: (0..=n).collect(),
        }
    }

    fn add_edge(&mut self, p1: usize, p2: usize) {
        self.tree_next[self.tree_cnt] = self.tree_head[p1];
        self.tree_head[p1] = self.tree_cnt;
        self.tree_to[self.tree_cnt] = p2;
        self.tree_cnt += 1;
    }

    fn add_query(&mut self, p1: usize, p2: usize, idx: usize) {
        self.q_next[self.q_cnt] = self.q_head[p1];
        self.q_head[p1] = self.q_cnt;
        self.q_to[self.q_cnt] = p2;
        self.q_idx[self.q_cnt] = idx;
        self.q_cnt += 1;
    }

    fn find(&mut self, v: usize) -> usize {
        if v != self.father[v] {
            self.father[v] = self.find(self.father[v]);
        }
        self.father[v]
    }

    fn find_iter(&mut self, mut v: usize) -> usize {
        let mut fv = self.father[v];
        while v != fv {
            self.father[v] = self.father[fv];
            (v, fv) = (fv, self.father[fv])
        }
        v
    }

    fn query(&mut self, root: usize, par: usize, ans: &mut [usize]) {
        self.visited[root] = true;
        let mut nxt = self.tree_head[root];
        while nxt != 0 {
            let sub = self.tree_to[nxt];
            if sub != par {
                self.query(sub, root, ans);
                self.father[sub] = root;
            }
            nxt = self.tree_next[nxt];
        }
        nxt = self.q_head[root];
        while nxt != 0 {
            let sub = self.q_to[nxt];
            let ai = self.q_idx[nxt];
            if self.visited[sub] {
                ans[ai] = self.find(sub)
            }
            nxt = self.q_next[nxt];
        }
    }

    fn query_iter(&mut self, root: usize, n: usize, ans: &mut [usize]) {
        let mut stack = Vec::with_capacity(n);
        stack.push((root, 0, -1));
        while !stack.is_empty() {
            let (node, par, edge) = stack.pop().unwrap();
            let mut nxt = if edge == -1 {
                self.visited[node] = true;
                self.tree_head[node]
            } else {
                self.tree_next[edge as usize]
            };
            if nxt != 0 {
                stack.push((node, par, nxt as i32));
                let sub = self.tree_to[nxt];
                if sub != par {
                    stack.push((sub, node, -1));
                }
            } else {
                nxt = self.q_head[node];
                while nxt != 0 {
                    let sub = self.q_to[nxt];
                    let ai = self.q_idx[nxt];
                    if self.visited[sub] {
                        ans[ai] = self.find_iter(sub)
                    }
                    nxt = self.q_next[nxt];
                }
                // 所有的子节点都遍历完之后，将本节点所在子树的集合合并到父节点
                self.father[node] = par
            }
        }
    }
}
