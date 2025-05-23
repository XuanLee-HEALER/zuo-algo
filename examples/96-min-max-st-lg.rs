use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut segs = buf.trim().split_whitespace();
    let n: usize = segs.next().unwrap().parse().unwrap();
    let m: usize = segs.next().unwrap().parse().unwrap();
    let mut heights = Vec::with_capacity(n);
    for _ in 0..n {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        heights.push(buf.trim().parse::<i32>().unwrap());
    }
    let st = ST::new(&heights);
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let a: usize = segs.next().unwrap().parse().unwrap();
        let b: usize = segs.next().unwrap().parse().unwrap();
        bw.write_fmt(format_args!("{}\n", st.q_max(a, b) - st.q_min(a, b)))
            .unwrap()
    }
    bw.flush().unwrap()
}

fn least_pow2(n: usize) -> usize {
    let mut r = 0;
    while 1 << r <= n >> 1 {
        r += 1
    }
    r
}

struct ST {
    log2: Vec<i32>,
    max_st: Vec<Vec<i32>>,
    min_st: Vec<Vec<i32>>,
}

impl ST {
    fn new(ori: &[i32]) -> Self {
        let n = ori.len();
        let m = least_pow2(n);
        let mut log2 = vec![-1; n + 1];
        let mut max_st = vec![vec![0; m + 1]; n + 1];
        let mut min_st = vec![vec![0; m + 1]; n + 1];
        for i in 1..=n {
            log2[i] = log2[i >> 1] + 1;
            max_st[i][0] = ori[i - 1];
            min_st[i][0] = ori[i - 1];
        }
        for j in 1..=m {
            for i in 1..=(n - (1 << j) + 1) {
                max_st[i][j] = max_st[i][j - 1].max(max_st[i + (1 << (j - 1))][j - 1]);
                min_st[i][j] = min_st[i][j - 1].min(min_st[i + (1 << (j - 1))][j - 1])
            }
        }
        Self {
            log2,
            max_st,
            min_st,
        }
    }

    fn q_max(&self, l: usize, r: usize) -> i32 {
        let d = r - l + 1;
        let w = self.log2[d] as usize;
        self.max_st[l][w].max(self.max_st[r - (1 << w) + 1][w])
    }

    fn q_min(&self, l: usize, r: usize) -> i32 {
        let d = r - l + 1;
        let w = self.log2[d] as usize;
        self.min_st[l][w].min(self.min_st[r - (1 << w) + 1][w])
    }
}
