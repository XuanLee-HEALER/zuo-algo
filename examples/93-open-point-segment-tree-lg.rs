use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut segs = buf.trim().split_whitespace();
    let n: usize = segs.next().unwrap().parse().unwrap();
    let m: usize = segs.next().unwrap().parse().unwrap();
    let mut st = STAddSum::new(80_000);
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split_whitespace();
        let op: u8 = segs.next().unwrap().parse().unwrap();
        if op == 1 {
            let l: usize = segs.next().unwrap().parse().unwrap();
            let r: usize = segs.next().unwrap().parse().unwrap();
            let v: i64 = segs.next().unwrap().parse().unwrap();
            st.add(l, r, v, 1, n, 1);
        } else {
            let l: usize = segs.next().unwrap().parse().unwrap();
            let r: usize = segs.next().unwrap().parse().unwrap();
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
    cnt: usize,
    left: Vec<usize>,
    right: Vec<usize>,
    sum: Vec<i64>,
    add: Vec<i64>,
}

impl STAddSum {
    fn new(n: usize) -> Self {
        Self {
            cnt: 1,
            left: vec![0; n],
            right: vec![0; n],
            sum: vec![0; n],
            add: vec![0; n],
        }
    }

    fn up(&mut self, i: usize) {
        self.sum[i] = self.sum[self.left[i]] + self.sum[self.right[i]]
    }

    // 将某个位置的懒信息传递下去
    fn down(&mut self, i: usize, ln: usize, rn: usize) {
        // 如果有懒信息
        if self.add[i] != 0 {
            let v = self.add[i];
            if self.left[i] == 0 {
                self.cnt += 1;
                self.left[i] = self.cnt;
            }
            if self.right[i] == 0 {
                self.cnt += 1;
                self.right[i] = self.cnt;
            }
            self.lazy(self.left[i], ln, v);
            self.lazy(self.right[i], rn, v);
            // 重置当前的懒信息
            self.add[i] = 0;
        }
    }

    // 懒信息要加到某个范围上，需要同步更新结果信息
    fn lazy(&mut self, i: usize, n: usize, v: i64) {
        self.add[i] += v;
        self.sum[i] += v * n as i64
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
                if self.left[i] == 0 {
                    self.cnt += 1;
                    self.left[i] = self.cnt;
                }
                self.add(jobl, jobr, jobv, l, m, self.left[i]);
            }
            if jobr > m {
                if self.right[i] == 0 {
                    self.cnt += 1;
                    self.right[i] = self.cnt;
                }
                self.add(jobl, jobr, jobv, m + 1, r, self.right[i]);
            }
            self.up(i);
        }
    }

    fn query(&mut self, jobl: usize, jobr: usize, l: usize, r: usize, i: usize) -> i64 {
        if jobl <= l && jobr >= r {
            self.sum[i]
        } else {
            let m = m(l, r);
            self.down(i, m - l + 1, r - m);
            let mut res = 0;
            if jobl <= m {
                res += self.query(jobl, jobr, l, m, self.left[i]);
            }
            if jobr > m {
                res += self.query(jobl, jobr, m + 1, r, self.right[i]);
            }
            res
        }
    }
}
