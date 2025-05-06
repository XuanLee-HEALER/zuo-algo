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
        .collect::<Vec<i64>>();
    let dif_arr = ori_arr
        .iter()
        .enumerate()
        .map(|(i, &v)| if i == 0 { v } else { v - ori_arr[i - 1] })
        .collect::<Vec<i64>>();
    let mut st = STAddSum::new(n);
    st.build(&dif_arr, 1, n, 1);
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let op: usize = segs.next().unwrap().parse().unwrap();
        if op == 1 {
            let r1: usize = segs.next().unwrap().parse().unwrap();
            let r2: usize = segs.next().unwrap().parse().unwrap();
            let k: i64 = segs.next().unwrap().parse().unwrap();
            let d: i64 = segs.next().unwrap().parse().unwrap();
            st.add(r1, r1, k, 1, n, 1);
            if r1 < n {
                st.add(r1 + 1, r2, d, 1, n, 1);
            }
            if r2 < n {
                st.add(r2 + 1, r2 + 1, -(k + d * (r2 - r1) as i64), 1, n, 1);
            }
        } else {
            let r1: usize = segs.next().unwrap().parse().unwrap();
            bw.write_fmt(format_args!("{}\n", st.query(1, r1, 1, n, 1)))
                .unwrap()
        }
    }
    bw.flush().unwrap()
}

fn m(l: usize, r: usize) -> usize {
    l + ((r - l) >> 1)
}

struct STAddSum {
    info: Vec<i64>,
    add: Vec<i64>,
}

impl STAddSum {
    fn new(n: usize) -> Self {
        Self {
            info: vec![0; n << 2],
            add: vec![0; n << 2],
        }
    }

    fn build(&mut self, ori: &[i64], l: usize, r: usize, i: usize) {
        if l == r {
            self.info[i] = ori[l - 1]
        } else {
            let m = m(l, r);
            self.build(ori, l, m, i << 1);
            self.build(ori, m + 1, r, i << 1 | 1);
            self.up(i);
        }
    }

    fn up(&mut self, i: usize) {
        self.info[i] = self.info[i << 1] + self.info[i << 1 | 1]
    }

    // 将某个位置的懒信息传递下去
    fn down(&mut self, i: usize, ln: usize, rn: usize) {
        // 如果有懒信息
        if self.add[i] != 0 {
            let v = self.add[i];
            self.lazy(i << 1, ln, v);
            self.lazy(i << 1 | 1, rn, v);
            // 重置当前的懒信息
            self.add[i] = 0;
        }
    }

    // 懒信息要加到某个范围上，需要同步更新结果信息
    fn lazy(&mut self, i: usize, n: usize, v: i64) {
        self.add[i] += v;
        self.info[i] += v * n as i64
    }

    fn add(&mut self, jobl: usize, jobr: usize, jobv: i64, l: usize, r: usize, i: usize) {
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

    fn query(&mut self, jobl: usize, jobr: usize, l: usize, r: usize, i: usize) -> i64 {
        if jobl <= l && jobr >= r {
            self.info[i]
        } else {
            let m = m(l, r);
            self.down(i, m - l + 1, r - m);
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
