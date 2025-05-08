use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut segs = buf.trim().split_whitespace();
    let n: usize = segs.next().unwrap().parse().unwrap();
    let m: usize = segs.next().unwrap().parse().unwrap();
    let mut stack = Vec::with_capacity(m);
    let mut st = STResetSum::new(n);
    st.build(1, n, 1);
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let op: char = segs.next().unwrap().parse().unwrap();
        match op {
            'D' => {
                let l = segs.next().unwrap().parse().unwrap();
                stack.push(l);
                st.reset(l, 0, 1, n, 1);
            }
            'R' => st.reset(stack.pop().unwrap(), 1, 1, n, 1),
            'Q' => {
                let l = segs.next().unwrap().parse().unwrap();
                bw.write_fmt(format_args!("{}\n", st.query(l, 1, n, 1)))
                    .unwrap()
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
    pre_a: Vec<usize>,
    suf_a: Vec<usize>,
}

impl STResetSum {
    fn new(n: usize) -> Self {
        Self {
            pre_a: vec![0; n << 2],
            suf_a: vec![0; n << 2],
        }
    }

    fn build(&mut self, l: usize, r: usize, i: usize) {
        if l == r {
            self.pre_a[i] = 1;
            self.suf_a[i] = 1;
        } else {
            let m = m(l, r);
            self.build(l, m, i << 1);
            self.build(m + 1, r, i << 1 | 1);
            self.up(i, m - l + 1, r - m);
        }
    }

    fn up(&mut self, i: usize, ln: usize, rn: usize) {
        self.pre_a[i] = if self.pre_a[i << 1] == ln {
            self.pre_a[i << 1] + self.pre_a[i << 1 | 1]
        } else {
            self.pre_a[i << 1]
        };
        self.suf_a[i] = if self.suf_a[i << 1 | 1] == rn {
            self.suf_a[i << 1] + self.suf_a[i << 1 | 1]
        } else {
            self.suf_a[i << 1 | 1]
        };
    }

    fn reset(&mut self, jobl: usize, jobv: usize, l: usize, r: usize, i: usize) {
        // 任务范围完全包括当前范围，那么整个范围只要累积懒信息即可
        if l == r {
            self.pre_a[i] = jobv;
            self.suf_a[i] = jobv;
        } else {
            let m = m(l, r);
            if jobl <= m {
                self.reset(jobl, jobv, l, m, i << 1);
            } else {
                self.reset(jobl, jobv, m + 1, r, i << 1 | 1);
            }
            self.up(i, m - l + 1, r - m);
        }
    }

    fn query(&mut self, jobl: usize, l: usize, r: usize, i: usize) -> usize {
        if l == r {
            self.pre_a[i]
        } else {
            let m = m(l, r);
            if jobl <= m {
                if self.suf_a[i << 1] >= m - jobl + 1 {
                    self.suf_a[i << 1] + self.pre_a[i << 1 | 1]
                } else {
                    self.query(jobl, l, m, i << 1)
                }
            } else {
                if self.pre_a[i << 1 | 1] >= jobl - m {
                    self.suf_a[i << 1] + self.pre_a[i << 1 | 1]
                } else {
                    self.query(jobl, m + 1, r, i << 1 | 1)
                }
            }
        }
    }
}
