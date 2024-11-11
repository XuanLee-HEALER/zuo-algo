use std::{
    i32,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

const MAXN: usize = 2_001;
const MANM: usize = 6_001;

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut head = false;
    let mut case_head = false;
    let mut case = 0;
    let mut n = 0;
    let mut m = 0;
    let mut graph = Graph::new(MAXN, MANM);
    let mut q: Vec<usize> = vec![0; MAXN * MANM];
    let mut visited = vec![false; MAXN];
    let mut update_cnt = vec![0; MAXN];
    let mut distances = vec![i32::MAX; MAXN];

    while let Ok(sz) = br.read_line(&mut buf) {
        let mut segs = buf.trim().split(" ");
        if sz > 0 {
            if !head {
                case = segs.next().unwrap().parse().unwrap();
                head = true;
            } else if !case_head {
                n = segs.next().unwrap().parse().unwrap();
                m = segs.next().unwrap().parse().unwrap();
                case_head = true;
            } else {
                let p1: usize = segs.next().unwrap().parse().unwrap();
                let p2: usize = segs.next().unwrap().parse().unwrap();
                let w: i32 = segs.next().unwrap().parse().unwrap();
                graph.add_edge(p1, p2, w);
                if w >= 0 {
                    graph.add_edge(p2, p1, w);
                }
                m -= 1;

                if m == 0 {
                    bw.write_fmt(format_args!(
                        "{}\n",
                        if spfa(
                            &graph,
                            n,
                            &mut q,
                            &mut visited,
                            &mut update_cnt,
                            &mut distances
                        ) {
                            "YES"
                        } else {
                            "NO"
                        }
                    ))
                    .unwrap();

                    case -= 1;
                    if case == 0 {
                        break;
                    }
                    graph.clear();
                    visited = vec![false; MAXN];
                    update_cnt = vec![0; MAXN];
                    distances = vec![i32::MAX; MAXN];
                    case_head = false;
                }
            }
        } else {
            break;
        }
        buf.clear();
    }
    bw.flush().unwrap();
}

// 判断图中从1出发是否有负环
fn spfa(
    graph: &Graph,
    n: usize,
    q: &mut [usize],
    visited: &mut [bool],
    update_cnt: &mut [i32],
    distances: &mut [i32],
) -> bool {
    let mut l = 0;
    let mut r = 0;
    q[r] = 1;
    r += 1;
    visited[1] = true;
    distances[1] = 0;
    while r > l {
        let cur_p = q[l];
        l += 1;
        visited[cur_p] = false;
        update_cnt[cur_p] += 1;
        if update_cnt[cur_p] as usize == n {
            return true;
        }
        let mut next_edge = graph.head[cur_p];
        while next_edge != 0 {
            let (p2, w) = (graph.to[next_edge], graph.weight[next_edge]);
            if distances[cur_p] + w < distances[p2] {
                distances[p2] = distances[cur_p] + w;
                if !visited[p2] {
                    q[r] = p2;
                    r += 1;
                    visited[p2] = true;
                }
            }
            next_edge = graph.next[next_edge];
        }
    }

    false
}

struct Graph {
    n: usize,
    m: usize,
    head: Vec<usize>,
    next: Vec<usize>,
    to: Vec<usize>,
    weight: Vec<i32>,
    cnt: usize,
}

impl Graph {
    fn new(n: usize, m: usize) -> Self {
        Self {
            n,
            m,
            head: vec![0; n],
            next: vec![0; m + 1],
            to: vec![0; m + 1],
            weight: vec![0; m + 1],
            cnt: 1,
        }
    }

    fn add_edge(&mut self, p1: usize, p2: usize, w: i32) {
        self.next[self.cnt] = self.head[p1];
        self.head[p1] = self.cnt;
        self.to[self.cnt] = p2;
        self.weight[self.cnt] = w;
        self.cnt += 1;
    }

    fn clear(&mut self) {
        self.head = vec![0; self.n];
        self.next = vec![0; self.m + 1];
        self.to = vec![0; self.m + 1];
        self.weight = vec![0; self.m + 1];
        self.cnt = 1;
    }
}
