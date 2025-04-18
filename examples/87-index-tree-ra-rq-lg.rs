use std::{
    io::{self, BufRead, BufReader, BufWriter, Write},
    ops::{BitAnd, Neg},
};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut seg = buf.trim().split_whitespace();
    let n: usize = seg.next().unwrap().parse().unwrap();
    let m: usize = seg.next().unwrap().parse().unwrap();
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let mut index_tree = RRIndexTree::new(n);
    buf.trim()
        .split_whitespace()
        .enumerate()
        .for_each(|(i, v)| {
            index_tree.add(i + 1, i + 1, v.parse().unwrap());
        });
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split_whitespace();
        let op: u8 = segs.next().unwrap().parse().unwrap();
        match op {
            1 => index_tree.add(
                segs.next().unwrap().parse().unwrap(),
                segs.next().unwrap().parse().unwrap(),
                segs.next().unwrap().parse().unwrap(),
            ),
            2 => bw
                .write_fmt(format_args!(
                    "{}\n",
                    index_tree.sum(
                        segs.next().unwrap().parse().unwrap(),
                        segs.next().unwrap().parse().unwrap()
                    )
                ))
                .unwrap(),
            _ => unreachable!(),
        }
    }
    bw.flush().unwrap()
}

fn bk<T: Neg<Output = T> + BitAnd<Output = T> + Copy>(v: T) -> T {
    v & (-v)
}

struct RRIndexTree {
    n: usize,
    tree1: Vec<i64>,
    tree2: Vec<i64>,
}

impl RRIndexTree {
    fn new(n: usize) -> Self {
        Self {
            n,
            // 表示 差分数组
            tree1: vec![0; n + 1],
            // 表示 (i-1)*差分数组
            tree2: vec![0; n + 1],
        }
    }

    fn t_add(n: usize, arr: &mut [i64], i: usize, v: i64) {
        let mut i = i as i32;
        while i <= n as i32 {
            arr[i as usize] += v as i64;
            i += bk(i)
        }
    }

    fn t_sum(&self, arr: &[i64], i: usize) -> i64 {
        let mut res = 0;
        let mut i = i as i32;
        while i > 0 {
            res += arr[i as usize] as i64;
            i -= bk(i)
        }
        res
    }

    // 原数组的 l..r 范围加上v
    // 等同于 差分数组中 l加上v  r+1减去v
    // 等同于 树状数组中 对l加上v r+1减去v
    fn add(&mut self, l: usize, r: usize, v: i64) {
        Self::t_add(self.n, &mut self.tree1, l, v);
        Self::t_add(self.n, &mut self.tree2, l, (l as i64 - 1) * v);
        Self::t_add(self.n, &mut self.tree1, r + 1, -v);
        Self::t_add(self.n, &mut self.tree2, r + 1, -(r as i64) * v);
    }

    fn sum(&self, l: usize, r: usize) -> i64 {
        r as i64 * self.t_sum(&self.tree1, r)
            - self.t_sum(&self.tree2, r)
            - (l as i64 - 1) * self.t_sum(&self.tree1, l - 1)
            + self.t_sum(&self.tree2, l - 1)
    }
}
