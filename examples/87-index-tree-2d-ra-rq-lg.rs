use std::{
    io::{self, BufRead, BufReader, BufWriter, Write},
    ops::{BitAnd, Neg},
};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    let mut idx_tree = IndexTree::default();
    while let Ok(n) = br.read_line(&mut buf) {
        if n > 1 {
            let mut segs = buf.trim().split(" ");
            let op = segs.next().unwrap();
            match op {
                "X" => {
                    let n: usize = segs.next().unwrap().parse().unwrap();
                    let m: usize = segs.next().unwrap().parse().unwrap();
                    idx_tree = IndexTree::new(n, m)
                }
                "L" => {
                    let a: usize = segs.next().unwrap().parse().unwrap();
                    let b: usize = segs.next().unwrap().parse().unwrap();
                    let c: usize = segs.next().unwrap().parse().unwrap();
                    let d: usize = segs.next().unwrap().parse().unwrap();
                    let v: i32 = segs.next().unwrap().parse().unwrap();
                    idx_tree.range_add(a, b, c, d, v);
                }
                "k" => {
                    let a: usize = segs.next().unwrap().parse().unwrap();
                    let b: usize = segs.next().unwrap().parse().unwrap();
                    let c: usize = segs.next().unwrap().parse().unwrap();
                    let d: usize = segs.next().unwrap().parse().unwrap();
                    bw.write_fmt(format_args!("{}\n", idx_tree.range(a, b, c, d)))
                        .unwrap()
                }
                _ => unreachable!(),
            }
        } else {
            break;
        }
        buf.clear();
    }

    bw.flush().unwrap()
}

fn bk<T: Neg<Output = T> + BitAnd<Output = T> + Copy>(v: T) -> T {
    v & (-v)
}

#[derive(Default)]
struct IndexTree {
    n: usize,
    m: usize,
    // (n+1)*(m+1) D
    arr1: Vec<Vec<i32>>,
    // (m+1) D
    arr2: Vec<Vec<i32>>,
    // (n+1) D
    arr3: Vec<Vec<i32>>,
    // D
    arr4: Vec<Vec<i32>>,
}

impl IndexTree {
    fn new(n: usize, m: usize) -> Self {
        Self {
            n,
            m,
            arr1: vec![vec![0; m + 1]; n + 1],
            arr2: vec![vec![0; m + 1]; n + 1],
            arr3: vec![vec![0; m + 1]; n + 1],
            arr4: vec![vec![0; m + 1]; n + 1],
        }
    }

    fn add(&mut self, i: usize, j: usize, v: i32) {
        let mut i = i as i32;
        let j = j as i32;
        let v1 = v;
        let v2 = v * j;
        let v3 = v * i;
        let v4 = v * i * j;
        while i <= self.n as i32 {
            let mut j = j;
            while j <= self.m as i32 {
                self.arr1[i as usize][j as usize] += v1;
                self.arr2[i as usize][j as usize] += v2;
                self.arr3[i as usize][j as usize] += v3;
                self.arr4[i as usize][j as usize] += v4;
                j += bk(j)
            }
            i += bk(i)
        }
    }

    fn sum(&self, i: usize, j: usize) -> i32 {
        let mut res = 0;
        let mut i = i as i32;
        let j = j as i32;
        let ii = i + 1;
        let jj = j + 1;
        while i > 0 {
            let mut j = j as i32;
            while j > 0 {
                res += ii * jj * self.arr1[i as usize][j as usize]
                    - ii * self.arr2[i as usize][j as usize]
                    - jj * self.arr3[i as usize][j as usize]
                    + self.arr4[i as usize][j as usize];
                j -= bk(j)
            }
            i -= bk(i)
        }
        res
    }

    fn range_add(&mut self, lui: usize, luj: usize, rdi: usize, rdj: usize, v: i32) {
        self.add(lui, luj, v);
        self.add(lui, rdj + 1, -v);
        self.add(rdi + 1, luj, -v);
        self.add(rdi + 1, rdj + 1, v);
    }

    fn range(&self, lui: usize, luj: usize, rdi: usize, rdj: usize) -> i32 {
        self.sum(rdi, rdj) - self.sum(lui - 1, rdj) - self.sum(rdi, luj - 1)
            + self.sum(lui - 1, luj - 1)
    }
}
