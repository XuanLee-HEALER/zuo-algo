use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut segs = buf.trim().split_whitespace();
    let n: usize = segs.next().unwrap().parse().unwrap();
    let _ = segs.next().unwrap();
    let o: usize = segs.next().unwrap().parse().unwrap();
    let mut st = STAddSum::new(n);
    st.build(1, n, 1);
    for _ in 0..o {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let op: char = segs.next().unwrap().parse().unwrap();
        if op == 'C' {
            let mut l: usize = segs.next().unwrap().parse().unwrap();
            let mut r: usize = segs.next().unwrap().parse().unwrap();
            let c: usize = segs.next().unwrap().parse().unwrap();
            if l > r {
                (l, r) = (r, l)
            }
            // 设置颜色
            st.set(l, r, 1 << (c - 1), 1, n, 1);
        } else {
            let mut l: usize = segs.next().unwrap().parse().unwrap();
            let mut r: usize = segs.next().unwrap().parse().unwrap();
            if l > r {
                (l, r) = (r, l)
            }
            // 查询几种颜色
            let s = st.query(l, r, 1, n, 1);
            bw.write_fmt(format_args!("{}\n", s.count_ones())).unwrap()
        }
    }
    bw.flush().unwrap()
}

fn m(l: usize, r: usize) -> usize {
    l + ((r - l) >> 1)
}

struct STAddSum {
    info: Vec<u32>,
    set: Vec<u32>,
}

impl STAddSum {
    fn new(n: usize) -> Self {
        Self {
            info: vec![0; n << 2],
            set: vec![0; n << 2],
        }
    }

    fn build(&mut self, l: usize, r: usize, i: usize) {
        if l == r {
            self.info[i] = 1
        } else {
            let m = m(l, r);
            self.build(l, m, i << 1);
            self.build(m + 1, r, i << 1 | 1);
            self.up(i);
        }
    }

    fn up(&mut self, i: usize) {
        self.info[i] = self.info[i << 1] | self.info[i << 1 | 1]
    }

    // 将某个位置的懒信息传递下去
    fn down(&mut self, i: usize) {
        // 如果有懒信息
        if self.set[i] != 0 {
            let v = self.set[i];
            self.lazy(i << 1, v);
            self.lazy(i << 1 | 1, v);
            // 重置当前的懒信息
            self.set[i] = 0;
        }
    }

    // 懒信息要加到某个范围上，需要同步更新结果信息
    fn lazy(&mut self, i: usize, v: u32) {
        self.info[i] = v;
        self.set[i] = v;
    }

    fn set(&mut self, jobl: usize, jobr: usize, jobv: u32, l: usize, r: usize, i: usize) {
        // 任务范围完全包括当前范围，那么整个范围只要累积懒信息即可
        if jobl <= l && jobr >= r {
            self.lazy(i, jobv);
        } else {
            let m = m(l, r);
            // 先将当前的懒信息移走，然后再在子范围上添加这个数字
            self.down(i);
            if jobl <= m {
                self.set(jobl, jobr, jobv, l, m, i << 1);
            }
            if jobr > m {
                self.set(jobl, jobr, jobv, m + 1, r, i << 1 | 1);
            }
            // ⚠️子范围调整完之后要更新父范围
            self.up(i);
        }
    }

    fn query(&mut self, jobl: usize, jobr: usize, l: usize, r: usize, i: usize) -> u32 {
        if jobl <= l && jobr >= r {
            self.info[i]
        } else {
            let m = m(l, r);
            self.down(i);
            let mut res = 0;
            if jobl <= m {
                res |= self.query(jobl, jobr, l, m, i << 1);
            }
            if jobr > m {
                res |= self.query(jobl, jobr, m + 1, r, i << 1 | 1);
            }
            res
        }
    }
}
