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
    let ori = buf
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect::<Vec<i64>>();
    let mut st = SegmentTree::new(n);
    st.build(&ori, 1, n, 1);
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split_whitespace();
        let op: usize = segs.next().unwrap().parse().unwrap();
        if op == 1 {
            let l: usize = segs.next().unwrap().parse().unwrap();
            let r: usize = segs.next().unwrap().parse().unwrap();
            let v: i64 = segs.next().unwrap().parse().unwrap();
            st.add(l, r, v, 1, n, 1);
        } else if op == 2 {
            let l: usize = segs.next().unwrap().parse().unwrap();
            let r: usize = segs.next().unwrap().parse().unwrap();
            bw.write_fmt(format_args!("{}\n", st.query(l, r, 1, n, 1)))
                .unwrap();
        }
    }
    bw.flush().unwrap()
}

struct SegmentTree {
    sum: Vec<i64>,
    add: Vec<i64>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        Self {
            sum: vec![0; n << 2],
            add: vec![0; n << 2],
        }
    }

    fn build(&mut self, ori: &[i64], l: usize, r: usize, i: usize) {
        if l == r {
            self.sum[i] = ori[l - 1]
        } else {
            let m = l + ((r - l) >> 1);
            self.build(ori, l, m, i << 1);
            self.build(ori, m + 1, r, i << 1 | 1);
            self.accumulate(i);
        }
    }

    /// 聚合操作，这是子范围的值汇聚到父范围所需的操作，可定制
    fn accumulate(&mut self, i: usize) {
        self.sum[i] = self.sum[i << 1] + self.sum[i << 1 | 1]
    }

    /// 分发懒值，是将当前位置的懒值传递给它左边和右边的子范围，具体的值（在累加和情况下）是范围长度*增加的值
    fn down(&mut self, i: usize, ln: usize, rn: usize) {
        let v = self.add[i];
        if v != 0 {
            self.lazy(i << 1, v, ln);
            self.lazy(i << 1 | 1, v, rn);
            self.add[i] = 0
        }
    }

    /// 懒值的创建，至少将要其对应的那个结果更新掉，更新的值和范围以及值相关，这里也是定制的
    fn lazy(&mut self, i: usize, v: i64, len: usize) {
        // 当前位置懒信息记录
        self.add[i] += v;
        // 当前位置的累加和要更新
        self.sum[i] += v * (len as i64)
    }

    fn add(&mut self, jobl: usize, jobr: usize, v: i64, l: usize, r: usize, i: usize) {
        if jobl <= l && jobr >= r {
            self.lazy(i, v, r - l + 1);
        } else {
            let m = l + ((r - l) >> 1);
            self.down(i, m - l + 1, r - m);
            if jobl <= m {
                self.add(jobl, jobr, v, l, m, i << 1);
            }
            if jobr > m {
                self.add(jobl, jobr, v, m + 1, r, i << 1 | 1);
            }
            self.accumulate(i);
        }
    }

    fn query(&mut self, jobl: usize, jobr: usize, l: usize, r: usize, i: usize) -> i64 {
        if jobl <= l && jobr >= r {
            self.sum[i]
        } else {
            let m = l + ((r - l) >> 1);
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

fn range_max_id(l: usize, r: usize, i: usize) -> usize {
    if l == r {
        i
    } else {
        let m = l + ((r - l) >> 1);
        range_max_id(l, m, 2 * i).max(range_max_id(m + 1, r, 2 * i + 1))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_range_max_id() {
        let mut max_times = (1, 1, 0.0);
        for i in 1..=10_000 {
            let m_id = super::range_max_id(1, i, 1);
            let c_times = m_id as f64 / i as f64;
            if c_times > max_times.2 {
                max_times = (i, m_id, c_times)
            }
        }
        dbg!(max_times);
    }
}
