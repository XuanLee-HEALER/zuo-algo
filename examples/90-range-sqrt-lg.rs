use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let mut st = STAddSum::new(n);
    let ori = buf
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect::<Vec<i64>>();
    st.build(&ori, 1, n, 1);
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let m: usize = buf.trim().parse().unwrap();
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split_whitespace();
        let op: usize = segs.next().unwrap().parse().unwrap();
        let mut l: usize = segs.next().unwrap().parse().unwrap();
        let mut r: usize = segs.next().unwrap().parse().unwrap();
        if l > r {
            (l, r) = (r, l)
        }
        if op == 0 {
            st.sqrt(l, r, 1, n, 1);
        } else {
            bw.write_fmt(format_args!("{}\n", st.query(l, r, 1, n, 1)))
                .unwrap()
        }
    }
    bw.flush().unwrap()
}

fn m(l: usize, r: usize) -> usize {
    l + ((r - l) >> 1)
}

struct STAddSum {
    sum: Vec<i64>,
    max: Vec<i64>,
}

impl STAddSum {
    fn new(n: usize) -> Self {
        Self {
            sum: vec![0; n << 2],
            max: vec![0; n << 2],
        }
    }

    fn build(&mut self, ori: &[i64], l: usize, r: usize, i: usize) {
        if l == r {
            self.sum[i] = ori[l - 1];
            self.max[i] = ori[l - 1];
        } else {
            let m = m(l, r);
            self.build(ori, l, m, i << 1);
            self.build(ori, m + 1, r, i << 1 | 1);
            self.up(i);
        }
    }

    fn up(&mut self, i: usize) {
        self.sum[i] = self.sum[i << 1] + self.sum[i << 1 | 1];
        self.max[i] = self.max[i << 1].max(self.max[i << 1 | 1]);
    }

    fn sqrt(&mut self, jobl: usize, jobr: usize, l: usize, r: usize, i: usize) {
        if l == r {
            self.sum[i] = (self.sum[i] as f64).sqrt().floor() as i64;
            self.max[i] = (self.max[i] as f64).sqrt().floor() as i64;
        } else {
            let m = m(l, r);
            if jobl <= m && self.max[i << 1] > 1 {
                self.sqrt(jobl, jobr, l, m, i << 1);
            }
            if jobr > m && self.max[i << 1 | 1] > 1 {
                self.sqrt(jobl, jobr, m + 1, r, i << 1 | 1);
            }
            // ⚠️子范围调整完之后要更新父范围
            self.up(i);
        }
    }

    fn query(&mut self, jobl: usize, jobr: usize, l: usize, r: usize, i: usize) -> i64 {
        if jobl <= l && jobr >= r {
            self.sum[i]
        } else {
            let m = m(l, r);
            let mut res = 0;
            if jobl <= m {
                res += self.query(jobl, jobr, l, m, i << 1);
            }
            if jobr > m {
                res += self.query(jobl, jobr, m + 1, r, i << 1 | 1);
            }
            res
        }
    }
}
