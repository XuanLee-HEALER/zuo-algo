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
        .collect::<Vec<u8>>();
    let mut st = STFull::new(n);
    st.build(&ori, 1, n, 1);
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split_whitespace();
        let op: u8 = segs.next().unwrap().parse().unwrap();
        let l = segs.next().unwrap().parse::<usize>().unwrap() + 1;
        let r = segs.next().unwrap().parse::<usize>().unwrap() + 1;
        match op {
            0 => st.reset(l, r, 0, 1, n, 1),
            1 => st.reset(l, r, 1, 1, n, 1),
            2 => st.reverse(l, r, 1, n, 1),
            3 => bw
                .write_fmt(format_args!("{}\n", st.query_sum(l, r, 1, n, 1)))
                .unwrap(),
            4 => bw
                .write_fmt(format_args!("{}\n", st.query_longest_one(l, r, 1, n, 1).0))
                .unwrap(),
            _ => unreachable!(),
        }
    }
    bw.flush().unwrap()
}

fn m(l: usize, r: usize) -> usize {
    l + ((r - l) >> 1)
}

struct STFull {
    // 累加和
    sum: Vec<u32>,
    // 连续1最长长度
    len1: Vec<u32>,
    // 连续1最长前缀
    pre1: Vec<u32>,
    // 连续1最长后缀
    suf1: Vec<u32>,
    // 连续0最长长度
    len0: Vec<u32>,
    // 连续0最长前缀
    pre0: Vec<u32>,
    // 连续0最长后缀
    suf0: Vec<u32>,
    // 重置值数组
    change: Vec<u8>,
    // 是否重置值
    update: Vec<bool>,
    // 是否有重置任务
    reverse: Vec<bool>,
}

impl STFull {
    fn new(n: usize) -> Self {
        Self {
            sum: vec![0; n << 2],
            len1: vec![0; n << 2],
            pre1: vec![0; n << 2],
            suf1: vec![0; n << 2],
            len0: vec![0; n << 2],
            pre0: vec![0; n << 2],
            suf0: vec![0; n << 2],
            change: vec![0; n << 2],
            update: vec![false; n << 2],
            reverse: vec![false; n << 2],
        }
    }

    fn build(&mut self, ori: &[u8], l: usize, r: usize, i: usize) {
        if l == r {
            let c = ori[l - 1];
            self.sum[i] = c as u32;
            self.len1[i] = c as u32;
            self.pre1[i] = c as u32;
            self.suf1[i] = c as u32;
            self.len0[i] = if c == 0 { 1 } else { 0 };
            self.pre0[i] = if c == 0 { 1 } else { 0 };
            self.suf0[i] = if c == 0 { 1 } else { 0 };
        } else {
            let m = m(l, r);
            self.build(ori, l, m, i << 1);
            self.build(ori, m + 1, r, i << 1 | 1);
            self.up(i, (m - l + 1) as u32, (r - m) as u32);
        }
    }

    fn up(&mut self, i: usize, ln: u32, rn: u32) {
        let (li, ri) = (i << 1, i << 1 | 1);
        self.sum[i] = self.sum[li] + self.sum[ri];
        // 连续1的最长长度 = 左边最长，右边最长，左边后缀+右边前缀 三个数的最大值
        self.len1[i] = self.len1[li]
            .max(self.len1[ri])
            .max(self.suf1[li] + self.pre1[ri]);
        // 连续1的最长前缀 = 如果左边的连续1长度和左边的范围一样，那么就是左边前缀+右边前缀，否则就是左边前缀
        self.pre1[i] = if self.len1[li] == ln {
            self.pre1[li] + self.pre1[ri]
        } else {
            self.pre1[li]
        };
        self.suf1[i] = if self.len1[ri] == rn {
            self.suf1[li] + self.suf1[ri]
        } else {
            self.suf1[ri]
        };
        self.len0[i] = self.len0[li]
            .max(self.len0[ri])
            .max(self.suf0[li] + self.pre0[ri]);
        self.pre0[i] = if self.len0[li] == ln {
            self.pre0[li] + self.pre0[ri]
        } else {
            self.pre0[li]
        };
        self.suf0[i] = if self.len0[ri] == rn {
            self.suf0[li] + self.suf0[ri]
        } else {
            self.suf0[ri]
        };
    }

    fn down(&mut self, i: usize, ln: u32, rn: u32) {
        if self.update[i] {
            let v = self.change[i];
            self.lazy_reset(i << 1, v, ln);
            self.lazy_reset(i << 1 | 1, v, rn);
            self.update[i] = false;
        }

        if self.reverse[i] {
            self.lazy_reverse(i << 1, ln);
            self.lazy_reverse(i << 1 | 1, rn);
            self.reverse[i] = false;
        }
    }

    fn lazy_reset(&mut self, i: usize, v: u8, n: u32) {
        self.update[i] = true;
        self.change[i] = v;
        self.sum[i] = if v == 0 { 0 } else { n };
        self.len1[i] = if v == 0 { 0 } else { n };
        self.pre1[i] = if v == 0 { 0 } else { n };
        self.suf1[i] = if v == 0 { 0 } else { n };
        self.len0[i] = if v == 0 { n } else { 0 };
        self.pre0[i] = if v == 0 { n } else { 0 };
        self.suf0[i] = if v == 0 { n } else { 0 };
        self.reverse[i] = false;
    }

    fn lazy_reverse(&mut self, i: usize, n: u32) {
        // ⚠️反转是取反
        self.reverse[i] = !self.reverse[i];
        self.sum[i] = n - self.sum[i];
        (
            self.len1[i],
            self.pre1[i],
            self.suf1[i],
            self.len0[i],
            self.pre0[i],
            self.suf0[i],
        ) = (
            self.len0[i],
            self.pre0[i],
            self.suf0[i],
            self.len1[i],
            self.pre1[i],
            self.suf1[i],
        );
    }

    fn reset(&mut self, jobl: usize, jobr: usize, jobv: u8, l: usize, r: usize, i: usize) {
        if jobl <= l && jobr >= r {
            self.lazy_reset(i, jobv, (r - l + 1) as u32);
        } else {
            let m = m(l, r);
            let (ll, rr) = ((m - l + 1) as u32, (r - m) as u32);
            self.down(i, ll, rr);
            if jobl <= m {
                self.reset(jobl, jobr, jobv, l, m, i << 1);
            }
            if jobr > m {
                self.reset(jobl, jobr, jobv, m + 1, r, i << 1 | 1);
            }
            self.up(i, ll, rr);
        }
    }

    fn reverse(&mut self, jobl: usize, jobr: usize, l: usize, r: usize, i: usize) {
        if jobl <= l && jobr >= r {
            self.lazy_reverse(i, (r - l + 1) as u32);
        } else {
            let m = m(l, r);
            let (ll, rr) = ((m - l + 1) as u32, (r - m) as u32);
            self.down(i, ll, rr);
            if jobl <= m {
                self.reverse(jobl, jobr, l, m, i << 1);
            }
            if jobr > m {
                self.reverse(jobl, jobr, m + 1, r, i << 1 | 1);
            }
            self.up(i, ll, rr);
        }
    }

    fn query_sum(&mut self, jobl: usize, jobr: usize, l: usize, r: usize, i: usize) -> u32 {
        if jobl <= l && jobr >= r {
            self.sum[i]
        } else {
            let m = m(l, r);
            let (ll, rr) = ((m - l + 1) as u32, (r - m) as u32);
            self.down(i, ll, rr);
            let mut res = 0;
            if jobl <= m {
                res += self.query_sum(jobl, jobr, l, m, i << 1);
            }
            if jobr > m {
                res += self.query_sum(jobl, jobr, m + 1, r, i << 1 | 1);
            }
            res
        }
    }

    fn query_longest_one(
        &mut self,
        jobl: usize,
        jobr: usize,
        l: usize,
        r: usize,
        i: usize,
    ) -> (u32, u32, u32) {
        if jobl <= l && jobr >= r {
            (self.len1[i], self.pre1[i], self.suf1[i])
        } else {
            // ⚠️只需要查找范围上的连续1相关的信息，如果范围只有左边，那就是左边的结果，如果只有右边，就是右边的结果
            let m = m(l, r);
            let (ll, rr) = ((m - l + 1) as u32, (r - m) as u32);
            self.down(i, ll, rr);
            if jobr <= m {
                self.query_longest_one(jobl, jobr, l, m, i << 1)
            } else if jobl > m {
                self.query_longest_one(jobl, jobr, m + 1, r, i << 1 | 1)
            } else {
                // 如果左边范围也有、右边范围也有，取两个范围的最大值
                let (ll1, lp1, ls1) = self.query_longest_one(jobl, jobr, l, m, i << 1);
                let (rl1, rp1, rs1) = self.query_longest_one(jobl, jobr, m + 1, r, i << 1 | 1);
                // 当前结果的左右范围以l r为准
                let tll = l.max(jobl);
                let trr = r.min(jobr);
                // 左边距离中点有几个值
                let ln = (m - tll + 1) as u32;
                // 右边距离中点有几个值
                let rn = (trr - m) as u32;
                (
                    ll1.max(rl1).max(ls1 + rp1),
                    if ll1 < ln { lp1 } else { lp1 + rp1 },
                    if rl1 < rn { rs1 } else { ls1 + rs1 },
                )
            }
        }
    }
}
