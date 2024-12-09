use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin());
    let mut bw = BufWriter::new(io::stdout());
    let mut buf = String::new();
    let mut n = 0;
    let mut m = 0;
    let mut head = false;

    const MAXN: usize = 100_002;
    let mut xhead = vec![0; MAXN];
    let mut next = vec![0; MAXN];
    let mut to = vec![0; MAXN];
    let mut indegree = vec![0; MAXN];
    let mut cnt = 1;

    while let Ok(sz) = br.read_line(&mut buf) {
        if sz > 0 {
            if !head {
                let mut segs = buf.trim().split(" ");
                n = segs.next().unwrap().parse().unwrap();
                m = segs.next().unwrap().parse().unwrap();
                head = true
            } else {
                let mut segs = buf.trim().split(" ");
                let p1: usize = segs.next().unwrap().parse().unwrap();
                let p2 = segs.next().unwrap().parse().unwrap();
                next[cnt] = xhead[p1];
                xhead[p1] = cnt;
                to[cnt] = p2;
                indegree[p2] += 1;
                cnt += 1;
                m -= 1;

                if m == 0 {
                    let mut ans = vec![0; n];
                    topo(n, &xhead, &next, &to, &mut indegree, &mut ans);
                    for &e in ans.iter().take(n - 1) {
                        print!("{} ", e);
                    }
                    println!("{}", ans[n - 1]);
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

fn topo(
    n: usize,
    head: &[usize],
    next: &[usize],
    to: &[usize],
    indegree: &mut [usize],
    ans: &mut [usize],
) {
    let mut in_heap = MinHeap::new(n);

    for (i, &e) in indegree.iter().take(n + 1).skip(1).enumerate() {
        if e == 0 {
            in_heap.push(i + 1);
        }
    }

    let mut cnt = 0;
    while in_heap.len > 0 {
        let sorted = in_heap.pop();
        ans[cnt] = sorted;
        cnt += 1;
        let mut j = head[sorted];
        while j > 0 {
            if indegree[to[j]] == 1 {
                in_heap.push(to[j]);
            }
            indegree[to[j]] -= 1;
            j = next[j];
        }
    }
}

struct MinHeap {
    heap: Vec<usize>,
    len: usize,
}

impl MinHeap {
    fn new(n: usize) -> Self {
        Self {
            heap: vec![0; n],
            len: 0,
        }
    }

    fn push(&mut self, v: usize) {
        let mut i = self.len;
        self.heap[i] = v;
        self.len += 1;
        while i > 0 {
            let p = self.heap[(i - 1) >> 1];
            if self.heap[i] < p {
                self.heap.swap(i, (i - 1) >> 1);
                i = (i - 1) >> 1;
            } else {
                break;
            }
        }
    }

    fn pop(&mut self) -> usize {
        let res = self.heap[0];
        self.heap.swap(0, self.len - 1);
        self.len -= 1;
        self.heapify();
        res
    }

    fn heapify(&mut self) {
        let mut i = 0;
        while i < self.len {
            let cur = self.heap[i];
            let left_idx = (i << 1) + 1;
            let right_idx = (i << 1) + 2;
            let left = self.heap[left_idx];
            let right = self.heap[right_idx];
            if left_idx < self.len && right_idx < self.len {
                if left < right {
                    if cur > left {
                        self.heap.swap(i, left_idx);
                        i = left_idx;
                    } else {
                        break;
                    }
                } else if cur > right {
                    self.heap.swap(i, right_idx);
                    i = right_idx;
                } else {
                    break;
                }
            } else if left_idx < self.len {
                if cur > left {
                    self.heap.swap(i, i * 2 + 1);
                    i = left_idx;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test_min_heap {
    #[test]
    fn test_min_heap_op() {
        let mut mhp = super::MinHeap::new(10);
        mhp.push(4);
        mhp.push(9);
        mhp.push(2);
        mhp.push(1);
        mhp.push(14);
        mhp.push(22);
        while mhp.len > 0 {
            println!("{}", mhp.pop());
        }
    }
}
