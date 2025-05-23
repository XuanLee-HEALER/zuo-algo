use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut segs = buf.trim().split_whitespace();
    segs.next().unwrap();
    let m: usize = segs.next().unwrap().parse().unwrap();
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let nums = buf
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect::<Vec<i32>>();
    let st = ST::new(&nums);
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let a: usize = segs.next().unwrap().parse().unwrap();
        let b: usize = segs.next().unwrap().parse().unwrap();
        bw.write_fmt(format_args!("{}\n", st.q_gcd(a, b))).unwrap()
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

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

struct ST {
    log2: Vec<i32>,
    gcd_st: Vec<Vec<i32>>,
}

impl ST {
    fn new(ori: &[i32]) -> Self {
        let n = ori.len();
        let m = least_pow2(n);
        let mut log2 = vec![-1; n + 1];
        let mut gcd_st = vec![vec![0; m + 1]; n + 1];
        for i in 1..=n {
            log2[i] = log2[i >> 1] + 1;
            gcd_st[i][0] = ori[i - 1];
        }
        for j in 1..=m {
            for i in 1..=(n - (1 << j) + 1) {
                gcd_st[i][j] = gcd(gcd_st[i][j - 1], gcd_st[i + (1 << (j - 1))][j - 1]);
            }
        }
        Self { log2, gcd_st }
    }

    fn q_gcd(&self, l: usize, r: usize) -> i32 {
        let d = r - l + 1;
        let w = self.log2[d] as usize;
        gcd(self.gcd_st[l][w], self.gcd_st[r - (1 << w) + 1][w])
    }
}
