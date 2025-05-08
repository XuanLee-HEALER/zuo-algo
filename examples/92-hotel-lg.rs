use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut segs = buf.trim().split_whitespace();
    let n: usize = segs.next().unwrap().parse().unwrap();
    let m: usize = segs.next().unwrap().parse().unwrap();
    let mut st = STResetSum::new(n);
    st.build(1, n, 1);
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let op: u8 = segs.next().unwrap().parse().unwrap();
        match op {
            1 => {
                let x: usize = segs.next().unwrap().parse().unwrap();
                if st.len0[1] >= x {
                    let l = st.query(x, 1, n, 1);
                    bw.write_fmt(format_args!("{}\n", l)).unwrap();
                    st.reset(l, l + x - 1, 1, 1, n, 1);
                } else {
                    bw.write_fmt(format_args!("0\n")).unwrap();
                }
            }
            2 => {
                let l: usize = segs.next().unwrap().parse().unwrap();
                let mut r: usize = segs.next().unwrap().parse().unwrap();
                r = (l + r - 1).min(n);
                st.reset(l, r, 0, 1, n, 1);
            }
            _ => unreachable!(),
        }
    }
    bw.flush().unwrap()
}

fn m(l: usize, r: usize) -> usize {
    l + ((r - l) >> 1)
}

struct STResetSum {
    len0: Vec<usize>,
    pre0: Vec<usize>,
    suf0: Vec<usize>,
    change: Vec<u8>,
    update: Vec<bool>,
}

impl STResetSum {
    fn new(n: usize) -> Self {
        Self {
            len0: vec![0; n << 2],
            pre0: vec![0; n << 2],
            suf0: vec![0; n << 2],
            change: vec![0; n << 2],
            update: vec![false; n << 2],
        }
    }

    fn build(&mut self, l: usize, r: usize, i: usize) {
        if l == r {
            self.len0[i] = 1;
            self.pre0[i] = 1;
            self.suf0[i] = 1;
        } else {
            let m = m(l, r);
            self.build(l, m, i << 1);
            self.build(m + 1, r, i << 1 | 1);
            self.up(i, m - l + 1, r - m);
        }
    }

    fn up(&mut self, i: usize, ln: usize, rn: usize) {
        let (ll, rr) = (i << 1, i << 1 | 1);
        self.len0[i] = self.len0[ll]
            .max(self.len0[rr])
            .max(self.suf0[ll] + self.pre0[rr]);
        self.pre0[i] = if self.pre0[ll] == ln {
            self.pre0[ll] + self.pre0[rr]
        } else {
            self.pre0[ll]
        };
        self.suf0[i] = if self.suf0[rr] == rn {
            self.suf0[ll] + self.suf0[rr]
        } else {
            self.suf0[rr]
        };
    }

    // 将某个位置的懒信息传递下去
    fn down(&mut self, i: usize, ln: usize, rn: usize) {
        // 如果有懒信息
        if self.update[i] {
            let v = self.change[i];
            self.lazy(i << 1, ln, v);
            self.lazy(i << 1 | 1, rn, v);
            // 重置当前的懒信息
            self.update[i] = false;
        }
    }

    // 懒信息要加到某个范围上，需要同步更新结果信息
    fn lazy(&mut self, i: usize, n: usize, v: u8) {
        self.update[i] = true;
        self.change[i] = v;
        if v == 0 {
            self.len0[i] = n;
            self.pre0[i] = n;
            self.suf0[i] = n;
        } else {
            self.len0[i] = 0;
            self.pre0[i] = 0;
            self.suf0[i] = 0;
        }
    }

    fn reset(&mut self, jobl: usize, jobr: usize, jobv: u8, l: usize, r: usize, i: usize) {
        // 任务范围完全包括当前范围，那么整个范围只要累积懒信息即可
        if jobl <= l && jobr >= r {
            self.lazy(i, r - l + 1, jobv);
        } else {
            let m = m(l, r);
            // 先将当前的懒信息移走，然后再在子范围上添加这个数字
            self.down(i, m - l + 1, r - m);
            if jobl <= m {
                self.reset(jobl, jobr, jobv, l, m, i << 1);
            }
            if jobr > m {
                self.reset(jobl, jobr, jobv, m + 1, r, i << 1 | 1);
            }
            // ⚠️子范围调整完之后要更新父范围
            self.up(i, m - l + 1, r - m);
        }
    }

    fn query(&mut self, x: usize, l: usize, r: usize, i: usize) -> usize {
        if l == r {
            l
        } else {
            let m = m(l, r);
            self.down(i, m - l + 1, r - m);
            if self.len0[i << 1] >= x {
                self.query(x, l, m, i << 1)
            } else if self.suf0[i << 1] + self.pre0[i << 1 | 1] >= x {
                m + 1 - self.suf0[i << 1]
            } else {
                self.query(x, m + 1, r, i << 1 | 1)
            }
        }
    }
}
