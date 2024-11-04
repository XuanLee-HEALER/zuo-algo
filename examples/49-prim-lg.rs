use std::{
    collections::BinaryHeap,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

#[derive(Clone)]
struct PointEdge(usize, usize);

impl Ord for PointEdge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

impl Eq for PointEdge {}

impl PartialOrd for PointEdge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PointEdge {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut head = false;
    const MAXN: usize = 5_001;
    let mut n = 0;
    let mut m = 0;
    let mut node_hash = vec![false; MAXN];
    let mut graph = vec![Vec::<PointEdge>::new(); MAXN];
    let mut min_heap = BinaryHeap::new();
    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                let mut segs = buf.trim().split(" ");
                n = segs.next().unwrap().parse().unwrap();
                m = segs.next().unwrap().parse().unwrap();
                head = true;
            } else {
                let mut segs = buf.trim().split(" ");
                let p1: usize = segs.next().unwrap().parse().unwrap();
                let p2: usize = segs.next().unwrap().parse().unwrap();
                let weight: usize = segs.next().unwrap().parse().unwrap();
                // 无向图要插两条边!!!
                graph[p1].push(PointEdge(p2, weight));
                graph[p2].push(PointEdge(p1, weight));
                m -= 1;

                if m == 0 {
                    let mut total_w = 0;
                    let mut total_node = 0;

                    node_hash[1] = true;
                    for edge in &graph[1] {
                        min_heap.push(edge);
                    }
                    total_node += 1;

                    while !min_heap.is_empty() {
                        let &PointEdge(p2, w) = min_heap.pop().unwrap();
                        if !node_hash[p2] {
                            total_w += w;
                            node_hash[p2] = true;
                            for edge in &graph[p2] {
                                min_heap.push(edge);
                            }
                            total_node += 1;
                            if total_node == n {
                                break;
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
