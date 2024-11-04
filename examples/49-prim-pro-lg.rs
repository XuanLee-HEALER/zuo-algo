use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut n = 0;
    let mut m = 0;
    let mut head = false;
    const MAXN: usize = 5_001;
    const MAXM: usize = 400_001;
    let mut graph = Graph::new(MAXN, MAXM);
    let mut min_heap = MinHeap::new(MAXN);
    while let Ok(sz) = br.read_line(&mut buf) {
        let mut segs = buf.trim().split(" ");
        if sz > 0 {
            if !head {
                n = segs.next().unwrap().parse().unwrap();
                m = segs.next().unwrap().parse().unwrap();
                head = true;
            } else {
                let p1: usize = segs.next().unwrap().parse().unwrap();
                let p2: usize = segs.next().unwrap().parse().unwrap();
                let w: usize = segs.next().unwrap().parse().unwrap();
                graph.add_edge(p1, p2, w);
                graph.add_edge(p2, p1, w);
                m -= 1;

                if m == 0 {
                    min_heap.search[1] = -2;
                    let mut total_node = 1;
                    let mut total_w = 0;
                    for &(p2, w) in &graph.edges(1) {
                        // 这里不能无脑加
                        match min_heap.search[p2] {
                            -1 => {
                                min_heap.heap_insert((p2, w));
                            }
                            o if o >= 0 => {
                                if w < min_heap.heap[o as usize].1 {
                                    min_heap.heap_update(p2, w);
                                }
                            }
                            _ => {
                                continue;
                            }
                        }
                    }

                    while !min_heap.is_empty() {
                        let (p2, w) = min_heap.pop();
                        total_node += 1;
                        total_w += w;
                        for &(p2_x, xw) in &graph.edges(p2) {
                            match min_heap.search[p2_x] {
                                -1 => {
                                    min_heap.heap_insert((p2_x, xw));
                                }
                                o if o >= 0 => {
                                    if xw < min_heap.heap[o as usize].1 {
                                        min_heap.heap_update(p2_x, xw);
                                    }
                                }
                                _ => {
                                    continue;
                                }
                            }
                        }
                    }

                    bw.write_fmt(format_args!(
                        "{}\n",
                        if total_node == n {
                            total_w.to_string()
                        } else {
                            "orz".into()
                        }
                    ))
                    .unwrap();
                    break;
                }
            }
        } else {
            break;
        }
        buf.clear();
    }
    bw.flush().unwrap();
}

struct Graph {
    head: Vec<usize>,
    next: Vec<usize>,
    to: Vec<usize>,
    weight: Vec<usize>,
    cnt: usize,
}

impl Graph {
    fn new(n: usize, m: usize) -> Self {
        Self {
            head: vec![0; n],
            next: vec![0; m],
            to: vec![0; m],
            weight: vec![0; m],
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

    fn edges(&self, p1: usize) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        let mut nl = self.head[p1];
        while nl > 0 {
            res.push((self.to[nl], self.weight[nl]));
            nl = self.next[nl];
        }
        res
    }
}

struct MinHeap {
    heap: Vec<(usize, usize)>,
    size: usize,
    search: Vec<i32>,
}

impl MinHeap {
    fn new(n: usize) -> Self {
        Self {
            heap: vec![(0, 0); n],
            size: 0,
            search: vec![-1; n],
        }
    }

    fn heapify(&mut self, n: usize) {
        let mut n = n;
        while n < self.size {
            let l_idx = (n << 1) + 1;
            let r_idx = l_idx + 1;
            if l_idx < self.size && r_idx < self.size {
                let a_idx = if self.heap[l_idx].1 < self.heap[r_idx].1 {
                    l_idx
                } else {
                    r_idx
                };
                if self.heap[n].1 > self.heap[a_idx].1 {
                    self.heap.swap(n, a_idx);
                    self.search.swap(self.heap[n].0, self.heap[a_idx].0);
                    n = a_idx;
                } else {
                    break;
                }
            } else if l_idx < self.size {
                if self.heap[n].1 > self.heap[l_idx].1 {
                    self.heap.swap(n, l_idx);
                    self.search.swap(self.heap[n].0, self.heap[l_idx].0);
                    n = l_idx;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    fn heap_insert(&mut self, v: (usize, usize)) {
        let (p2, _) = v;
        let mut cur_idx = self.size;
        self.heap[cur_idx] = v;
        self.search[p2] = cur_idx as i32;
        self.size += 1;
        while cur_idx > 0 {
            let p_idx = (cur_idx - 1) >> 1;
            if self.heap[cur_idx].1 < self.heap[p_idx].1 {
                self.heap.swap(cur_idx, p_idx);
                self.search.swap(self.heap[cur_idx].0, self.heap[p_idx].0);
                cur_idx = p_idx;
            } else {
                break;
            }
        }
    }

    fn heap_update(&mut self, p2: usize, v: usize) {
        let mut cur_idx = self.search[p2] as usize;
        self.heap[cur_idx].1 = v;
        while cur_idx > 0 {
            let p_idx = (cur_idx - 1) >> 1;
            if self.heap[cur_idx].1 < self.heap[p_idx].1 {
                self.heap.swap(cur_idx, p_idx);
                self.search.swap(self.heap[cur_idx].0, self.heap[p_idx].0);
                cur_idx = p_idx;
            } else {
                break;
            }
        }
    }

    fn pop(&mut self) -> (usize, usize) {
        let res = self.heap[0];
        self.heap.swap(0, self.size - 1);
        self.search.swap(self.heap[0].0, self.heap[self.size - 1].0);
        self.search[self.heap[self.size - 1].0] = -2;
        self.size -= 1;
        self.heapify(0);
        res
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}
