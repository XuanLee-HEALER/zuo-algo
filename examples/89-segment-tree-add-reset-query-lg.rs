use std::{
    i64,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

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
    let ori: Vec<i64> = buf
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    let mut st = STAddResetMax::new(n);
    st.build(&ori, 1, n, 1);
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        let mut segs = buf.trim().split_whitespace();
        let op: usize = segs.next().unwrap().parse().unwrap();
        match op {
            1 => {
                let l: usize = segs.next().unwrap().parse().unwrap();
                let r: usize = segs.next().unwrap().parse().unwrap();
                let v: i64 = segs.next().unwrap().parse().unwrap();
                st.reset(l, r, v, 1, n, 1);
            }
            2 => {
                let l: usize = segs.next().unwrap().parse().unwrap();
                let r: usize = segs.next().unwrap().parse().unwrap();
                let v: i64 = segs.next().unwrap().parse().unwrap();
                st.add(l, r, v, 1, n, 1);
            }
            3 => {
                let l: usize = segs.next().unwrap().parse().unwrap();
                let r: usize = segs.next().unwrap().parse().unwrap();
                bw.write_fmt(format_args!("{}\n", st.query(l, r, 1, n, 1)))
                    .unwrap();
            }
            _ => unreachable!(),
        }
    }
    bw.flush().unwrap()
}

fn m(l: usize, r: usize) -> usize {
    l + ((r - l) >> 1)
}

struct STAddResetMax {
    info: Vec<i64>,
    add: Vec<i64>,
    change: Vec<i64>,
    update: Vec<bool>,
}

impl STAddResetMax {
    fn new(n: usize) -> Self {
        Self {
            info: vec![0; n << 2],
            add: vec![0; n << 2],
            change: vec![0; n << 2],
            update: vec![false; n << 2],
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
        self.info[i] = self.info[i << 1].max(self.info[i << 1 | 1])
    }

    // 将某个位置的懒信息传递下去
    fn down(&mut self, i: usize) {
        if self.update[i] {
            let v = self.change[i];
            self.lazy_reset(i << 1, v);
            self.lazy_reset(i << 1 | 1, v);
            // 重置当前的懒信息
            self.update[i] = false;
        }

        if self.add[i] != 0 {
            let v = self.add[i];
            self.lazy_add(i << 1, v);
            self.lazy_add(i << 1 | 1, v);
            self.add[i] = 0;
        }
    }

    fn lazy_add(&mut self, i: usize, v: i64) {
        self.add[i] += v;
        self.info[i] += v
    }

    fn lazy_reset(&mut self, i: usize, v: i64) {
        // 之前的增加懒信息要被清除
        self.add[i] = 0;
        self.update[i] = true;
        self.change[i] = v;
        self.info[i] = v;
    }

    fn add(&mut self, jobl: usize, jobr: usize, jobv: i64, l: usize, r: usize, i: usize) {
        // 任务范围完全包括当前范围，那么整个范围只要累积懒信息即可
        if jobl <= l && jobr >= r {
            self.lazy_add(i, jobv);
        } else {
            let m = m(l, r);
            // 先将当前的懒信息移走，然后再在子范围上添加这个数字
            self.down(i);
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

    fn reset(&mut self, jobl: usize, jobr: usize, jobv: i64, l: usize, r: usize, i: usize) {
        // 任务范围完全包括当前范围，那么整个范围只要累积懒信息即可
        if jobl <= l && jobr >= r {
            self.lazy_reset(i, jobv);
        } else {
            let m = m(l, r);
            // 先将当前的懒信息移走，然后再在子范围上添加这个数字
            self.down(i);
            if jobl <= m {
                self.reset(jobl, jobr, jobv, l, m, i << 1);
            }
            if jobr > m {
                self.reset(jobl, jobr, jobv, m + 1, r, i << 1 | 1);
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
            self.down(i);
            let mut res = i64::MIN;
            if jobl <= m {
                res = res.max(self.query(jobl, jobr, l, m, i << 1));
            }
            if jobr > m {
                res = res.max(self.query(jobl, jobr, m + 1, r, i << 1 | 1));
            }
            res
        }
    }
}
