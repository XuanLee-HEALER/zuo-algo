use std::io::{self, BufRead, BufReader, BufWriter, Write};

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
    let ori_arr = buf
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect::<Vec<f64>>();
    let mut st = STAddSum::new(n);
    st.build(&ori_arr, 1, n, 1);
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let op: usize = segs.next().unwrap().parse().unwrap();
        if op == 1 {
            let r1: usize = segs.next().unwrap().parse().unwrap();
            let r2: usize = segs.next().unwrap().parse().unwrap();
            let v: f64 = segs.next().unwrap().parse().unwrap();
            st.add(r1, r2, v, 1, n, 1);
        } else if op == 2 {
            let r1: usize = segs.next().unwrap().parse().unwrap();
            let r2: usize = segs.next().unwrap().parse().unwrap();
            bw.write_fmt(format_args!(
                "{:.4}\n",
                st.query(r1, r2, 1, n, 1).0 / (r2 - r1 + 1) as f64
            ))
            .unwrap()
        } else {
            let r1: usize = segs.next().unwrap().parse().unwrap();
            let r2: usize = segs.next().unwrap().parse().unwrap();
            let (t1, t2) = st.query(r1, r2, 1, n, 1);
            let nn = (r2 - r1 + 1) as f64;
            bw.write_fmt(format_args!("{:.4}\n", t2 / nn - (t1 / nn).powf(2.0)))
                .unwrap()
        }
    }
    bw.flush().unwrap()
}

fn m(l: usize, r: usize) -> usize {
    l + ((r - l) >> 1)
}

struct STAddSum {
    sum: Vec<f64>,
    sqr_sum: Vec<f64>,
    add: Vec<f64>,
}

impl STAddSum {
    fn new(n: usize) -> Self {
        Self {
            sum: vec![0.0; n << 2],
            sqr_sum: vec![0.0; n << 2],
            add: vec![0.0; n << 2],
        }
    }

    fn build(&mut self, ori: &[f64], l: usize, r: usize, i: usize) {
        if l == r {
            self.sum[i] = ori[l - 1];
            self.sqr_sum[i] = ori[l - 1].powf(2.0);
        } else {
            let m = m(l, r);
            self.build(ori, l, m, i << 1);
            self.build(ori, m + 1, r, i << 1 | 1);
            self.up(i);
        }
    }

    fn up(&mut self, i: usize) {
        self.sum[i] = self.sum[i << 1] + self.sum[i << 1 | 1];
        self.sqr_sum[i] = self.sqr_sum[i << 1] + self.sqr_sum[i << 1 | 1];
    }

    // 将某个位置的懒信息传递下去
    fn down(&mut self, i: usize, ln: usize, rn: usize) {
        // 如果有懒信息
        if self.add[i] != 0.0 {
            let v = self.add[i];
            self.lazy(i << 1, ln, v);
            self.lazy(i << 1 | 1, rn, v);
            // 重置当前的懒信息
            self.add[i] = 0.0;
        }
    }

    // 懒信息要加到某个范围上，需要同步更新结果信息
    fn lazy(&mut self, i: usize, n: usize, v: f64) {
        self.sqr_sum[i] = self.sqr_sum[i] + 2.0 * v * self.sum[i] + v.powf(2.0) * n as f64;
        self.sum[i] = self.sum[i] + v * n as f64;
        self.add[i] += v;
    }

    fn add(&mut self, jobl: usize, jobr: usize, jobv: f64, l: usize, r: usize, i: usize) {
        // 任务范围完全包括当前范围，那么整个范围只要累积懒信息即可
        if jobl <= l && jobr >= r {
            self.lazy(i, r - l + 1, jobv);
        } else {
            let m = m(l, r);
            // 先将当前的懒信息移走，然后再在子范围上添加这个数字
            self.down(i, m - l + 1, r - m);
            if jobl <= m {
                self.add(jobl, jobr, jobv, l, m, i << 1);
            }
            if jobr > m {
                self.add(jobl, jobr, jobv, m + 1, r, i << 1 | 1);
            }
            // ⚠️子范围调整完之后要更新父范围
            self.up(i);
        }
    }

    fn query(&mut self, jobl: usize, jobr: usize, l: usize, r: usize, i: usize) -> (f64, f64) {
        if jobl <= l && jobr >= r {
            (self.sum[i], self.sqr_sum[i])
        } else {
            let m = m(l, r);
            self.down(i, m - l + 1, r - m);
            let mut res = (0.0, 0.0);
            if jobl <= m {
                let (t1, t2) = self.query(jobl, jobr, l, m, i << 1);
                res = (res.0 + t1, res.1 + t2);
            }
            if jobr > m {
                let (t1, t2) = self.query(jobl, jobr, m + 1, r, i << 1 | 1);
                res = (res.0 + t1, res.1 + t2);
            }
            res
        }
    }
}
