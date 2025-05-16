use std::{
    cmp::Ordering,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

#[derive(Clone, Copy)]
struct Task(usize, usize);

impl Eq for Task {}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut builds = Vec::with_capacity(5_000);
    let mut dots = Vec::with_capacity(10_000);
    while let Ok(n) = br.read_line(&mut buf) {
        if n == 0 || buf.trim().is_empty() {
            break;
        } else {
            let mut segs = buf.trim().split_whitespace();
            let l: usize = segs.next().unwrap().parse().unwrap();
            let h: usize = segs.next().unwrap().parse().unwrap();
            let r: usize = segs.next().unwrap().parse().unwrap();
            builds.push((l, r, h));
            dots.push(l);
            dots.push(r - 1);
            dots.push(r);
            buf.clear();
        }
    }
    dots.sort_unstable();
    dots.dedup();
    builds.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    let mut dot_height = vec![0; dots.len()];
    let mut heap = zuo_algo::Heap::new(builds.len());
    let mut j = 0;
    for (i, &dot) in dots.iter().enumerate() {
        while j < builds.len() {
            if builds[j].0 <= dot {
                heap.insert(Task(
                    builds[j].2,
                    dots.binary_search(&(builds[j].1 - 1)).unwrap(),
                ));
                j += 1
            } else {
                break;
            }
        }

        while !heap.is_empty() {
            if let Some(h) = heap.peek() {
                if h.1 < i {
                    heap.remove();
                    continue;
                }
            }
            break;
        }

        dot_height[i] = if let Some(head) = heap.peek() {
            head.0
        } else {
            0
        }
    }
    let mut res = vec![];
    let mut pre = 0;
    for (i, dh) in dot_height.iter().enumerate() {
        if *dh != pre {
            res.push(dots[i]);
            res.push(*dh);
            pre = *dh
        }
    }
    for r in res {
        bw.write_fmt(format_args!("{} ", r)).unwrap()
    }
    bw.write_fmt(format_args!("\n")).unwrap();
    bw.flush().unwrap()
}
