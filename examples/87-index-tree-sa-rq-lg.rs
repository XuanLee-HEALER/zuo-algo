use std::{
    io::{self, BufRead, BufReader, BufWriter, Write},
    ops::{BitAnd, Neg},
};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut segs = buf.trim().split_whitespace();
    let n: usize = segs.next().unwrap().parse().unwrap();
    let m: usize = segs.next().unwrap().parse().unwrap();
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let mut idx_tree = IndexArray::new(n);
    buf.trim()
        .split_whitespace()
        .enumerate()
        .for_each(|(i, v)| idx_tree.add(i + 1, v.parse().unwrap()));
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split_whitespace();
        let op: i32 = segs.next().unwrap().parse().unwrap();
        let r1: usize = segs.next().unwrap().parse().unwrap();
        let r2: i32 = segs.next().unwrap().parse().unwrap();
        match op {
            1 => idx_tree.add(r1, r2),
            2 => bw
                .write_fmt(format_args!("{}\n", idx_tree.range_query(r1, r2 as usize)))
                .unwrap(),
            _ => unreachable!(),
        }
    }
    bw.flush().unwrap()
}

fn bk<T: Neg<Output = T> + BitAnd<Output = T> + Copy>(v: T) -> T {
    v & (-v)
}

struct IndexArray<T> {
    n: usize,
    arr: Vec<T>,
}

impl IndexArray<i32> {
    fn new(n: usize) -> Self {
        Self {
            n: n,
            arr: vec![0; n + 1],
        }
    }

    fn add(&mut self, i: usize, v: i32) {
        let mut i = i as i32;
        while i <= self.n as i32 {
            self.arr[i as usize] += v;
            i += bk(i)
        }
    }

    fn query(&self, i: usize) -> i32 {
        let mut i = i as i32;
        let mut res = 0;
        while i > 0 {
            res += self.arr[i as usize];
            i -= bk(i)
        }
        res
    }

    fn range_query(&self, l: usize, r: usize) -> i32 {
        self.query(r) - self.query(l - 1)
    }
}
