use std::{
    i64,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

const LOWEST: i64 = i64::MIN;

fn main() {
    let mut br = BufReader::new(io::stdin().lock());
    let mut bw = BufWriter::new(io::stdout().lock());
    let mut buf = String::new();
    br.read_line(&mut buf).unwrap();
    let mut segs = buf.trim().split_whitespace();
    let n: usize = segs.next().unwrap().parse().unwrap();
    let m: usize = segs.next().unwrap().parse().unwrap();
    let mut rhs = RHSegmentTree::new(n, LOWEST);
    buf.clear();
    br.read_line(&mut buf).unwrap();
    let ori: Vec<i64> = buf
        .trim()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    rhs.build(&ori, 1, n, 1);
    for _ in 0..m {
        buf.clear();
        br.read_line(&mut buf).unwrap();
        segs = buf.trim().split_whitespace();
        let op: u8 = segs.next().unwrap().parse().unwrap();
        match op {
            1 => {
                let l: usize = segs.next().unwrap().parse().unwrap();
                let r: usize = segs.next().unwrap().parse().unwrap();
                let v: i64 = segs.next().unwrap().parse().unwrap();
                rhs.add(l, r, v, 1, n, 1);
            }
            2 => {
                let l: usize = segs.next().unwrap().parse().unwrap();
                let r: usize = segs.next().unwrap().parse().unwrap();
                let v: i64 = segs.next().unwrap().parse().unwrap();
                rhs.min(l, r, v, 1, n, 1);
            }
            3 => {
                let l: usize = segs.next().unwrap().parse().unwrap();
                let r: usize = segs.next().unwrap().parse().unwrap();
                bw.write_fmt(format_args!("{}\n", rhs.q_sum(l, r, 1, n, 1)))
                    .unwrap()
            }
            4 => {
                let l: usize = segs.next().unwrap().parse().unwrap();
                let r: usize = segs.next().unwrap().parse().unwrap();
                bw.write_fmt(format_args!("{}\n", rhs.q_max(l, r, 1, n, 1)))
                    .unwrap()
            }
            5 => {
                let l: usize = segs.next().unwrap().parse().unwrap();
                let r: usize = segs.next().unwrap().parse().unwrap();
                bw.write_fmt(format_args!("{}\n", rhs.q_h_max(l, r, 1, n, 1)))
                    .unwrap()
            }
            _ => unreachable!(),
        }
    }
    bw.flush().unwrap()
}

struct RHSegmentTree {
    // 累加和
    sum: Vec<i64>,
    // 最大值
    max: Vec<i64>,
    // 最大值个数
    max_s: Vec<usize>,
    // 严格次大值，⚠️用来在设置最小值时进行剪枝
    sec: Vec<i64>,
    // 最大值增幅，懒信息，这种结构设计可以整合lazy_add和lazy_min
    max_add: Vec<i64>,
    // 其它值增幅，懒信息
    oth_add: Vec<i64>,
    // 历史最大值
    max_h: Vec<i64>,
    // 历史最大值增幅，懒信息，必须有一个记录历史最大值增幅的数组，才能得到多次懒更新中的历史最大值
    max_h_add: Vec<i64>,
    // 历史其它值增幅，懒信息
    max_oth_add: Vec<i64>,
}

impl RHSegmentTree {
    fn new(n: usize, lowest: i64) -> Self {
        Self {
            sum: vec![0; n << 2],
            max: vec![0; n << 2],
            max_s: vec![0; n << 2],
            // 次大值的默认值就是一个不存在的最小值
            sec: vec![lowest; n << 2],
            max_add: vec![0; n << 2],
            oth_add: vec![0; n << 2],
            max_h: vec![0; n << 2],
            max_h_add: vec![0; n << 2],
            max_oth_add: vec![0; n << 2],
        }
    }

    fn build(&mut self, ori: &[i64], l: usize, r: usize, i: usize) {
        if l == r {
            self.sum[i] = ori[l - 1];
            self.max[i] = ori[l - 1];
            self.max_s[i] = 1;
            self.max_h[i] = ori[l - 1];
        } else {
            let m = l + ((r - l) >> 1);
            self.build(ori, l, m, i << 1);
            self.build(ori, m + 1, r, i << 1 | 1);
            self.up(i);
        }
    }

    fn up(&mut self, i: usize) {
        let l = i << 1;
        let r = i << 1 | 1;
        // 累加和是左+右
        self.sum[i] = self.sum[l] + self.sum[r];
        // 最大值是左右取最大
        self.max[i] = self.max[l].max(self.max[r]);
        // 历史最大值左右取最大
        self.max_h[i] = self.max_h[l].max(self.max_h[r]);
        // 处理最大值个数和严格次大值
        if self.max[l] > self.max[r] {
            // 父节点最大值是左侧
            self.max_s[i] = self.max_s[l];
            self.sec[i] = self.max[r].max(self.sec[l])
        } else if self.max[r] > self.max[l] {
            // 父节点最大值是右侧
            self.max_s[i] = self.max_s[r];
            self.sec[i] = self.max[l].max(self.sec[r])
        } else {
            // 左右两侧的最大值相同
            self.max_s[i] = self.max_s[l] + self.max_s[r];
            self.sec[i] = self.sec[l].max(self.sec[r])
        }
    }

    fn down(&mut self, i: usize, ln: usize, rn: usize) {
        let l = i << 1;
        let r = i << 1 | 1;
        // 根据左右范围的最大值和次大值决定如何往下传
        let cmp = self.max[l].max(self.max[r]);
        // 如果左边有最大值
        if cmp == self.max[l] {
            self.lazy(
                l,
                ln,
                self.max_add[i],
                self.oth_add[i],
                self.max_h_add[i],
                self.max_oth_add[i],
            );
        } else {
            self.lazy(
                l,
                ln,
                self.oth_add[i],
                self.oth_add[i],
                self.max_oth_add[i],
                self.max_oth_add[i],
            );
        }
        // 如果右边有最大值
        if cmp == self.max[r] {
            self.lazy(
                r,
                rn,
                self.max_add[i],
                self.oth_add[i],
                self.max_h_add[i],
                self.max_oth_add[i],
            );
        } else {
            self.lazy(
                r,
                rn,
                self.oth_add[i],
                self.oth_add[i],
                self.max_oth_add[i],
                self.max_oth_add[i],
            );
        }
        // 重置懒信息
        self.max_add[i] = 0;
        self.oth_add[i] = 0;
        self.max_h_add[i] = 0;
        self.max_oth_add[i] = 0;
    }

    /// lazy 在任意一个范围上可以放的懒信息，包括累加的值和可能变小的值
    /// 累加值=最大值增加x，非最大值加x，可能变小的值=最大值减少x，非最大值不变
    /// 历史最大值和历史其它值的增幅也是本次的增幅
    /// 懒信息需要更新当前范围上的其它信息
    fn lazy(
        &mut self,
        i: usize,
        n: usize,
        max_add: i64,
        oth_add: i64,
        max_h_add: i64,
        oth_h_add: i64,
    ) {
        // 当前范围上历史最大值，之前的值和 这个范围的最大值再加上本次增幅，取最大值
        self.max_h[i] = self.max_h[i].max(self.max[i] + max_h_add);
        // 当前范围上历史最大值增幅，是之前的值和 目前累积的最大值变化加上本次增幅，取最大值
        self.max_h_add[i] = self.max_h_add[i].max(self.max_add[i] + max_h_add);
        // 当前范围上其它值增幅，是之前的值和 目前累积的其它值变化加上本次增幅，取最大值
        self.max_oth_add[i] = self.max_oth_add[i].max(self.oth_add[i] + oth_h_add);
        // 更新范围累加和
        self.sum[i] += max_add * self.max_s[i] as i64 + oth_add * (n - self.max_s[i]) as i64;
        // 更新范围最大值，⚠️范围最大值要后更新，历史的值需要先更新
        self.max[i] += max_add;
        // 如果范围原本有次大值，这个值要变化，否则不需要变化
        self.sec[i] += if self.sec[i] == LOWEST { 0 } else { oth_add };
        // 最大值增加的值要累积，懒信息累积
        self.max_add[i] += max_add;
        // 其它值增加的值要累积，懒信息累积
        self.oth_add[i] += oth_add;
    }

    fn add(&mut self, jobl: usize, jobr: usize, jobv: i64, l: usize, r: usize, i: usize) {
        if jobl <= l && jobr >= r {
            self.lazy(i, r - l + 1, jobv, jobv, jobv, jobv);
        } else {
            let m = l + ((r - l) >> 1);
            self.down(i, m - l + 1, r - m);
            if jobl <= m {
                self.add(jobl, jobr, jobv, l, m, i << 1);
            }
            if jobr > m {
                self.add(jobl, jobr, jobv, m + 1, r, i << 1 | 1);
            }
            self.up(i);
        }
    }

    fn min(&mut self, jobl: usize, jobr: usize, jobv: i64, l: usize, r: usize, i: usize) {
        if jobv >= self.max[i] {
            return;
        }

        if jobl <= l && jobr >= r && jobv > self.sec[i] {
            self.lazy(i, r - l + 1, jobv - self.max[i], 0, jobv - self.max[i], 0);
        } else {
            let m = l + ((r - l) >> 1);
            self.down(i, m - l + 1, r - m);
            if jobl <= m {
                self.min(jobl, jobr, jobv, l, m, i << 1);
            }
            if jobr > m {
                self.min(jobl, jobr, jobv, m + 1, r, i << 1 | 1);
            }
            self.up(i);
        }
    }

    fn q_sum(&mut self, jobl: usize, jobr: usize, l: usize, r: usize, i: usize) -> i64 {
        if jobl <= l && jobr >= r {
            self.sum[i]
        } else {
            let m = l + ((r - l) >> 1);
            self.down(i, m - l + 1, r - m);
            let mut res = 0;
            if jobl <= m {
                res += self.q_sum(jobl, jobr, l, m, i << 1)
            }
            if jobr > m {
                res += self.q_sum(jobl, jobr, m + 1, r, i << 1 | 1)
            }
            res
        }
    }

    fn q_max(&mut self, jobl: usize, jobr: usize, l: usize, r: usize, i: usize) -> i64 {
        if jobl <= l && jobr >= r {
            self.max[i]
        } else {
            let m = l + ((r - l) >> 1);
            self.down(i, m - l + 1, r - m);
            let mut res = i64::MIN;
            if jobl <= m {
                res = res.max(self.q_max(jobl, jobr, l, m, i << 1))
            }
            if jobr > m {
                res = res.max(self.q_max(jobl, jobr, m + 1, r, i << 1 | 1))
            }
            res
        }
    }

    fn q_h_max(&mut self, jobl: usize, jobr: usize, l: usize, r: usize, i: usize) -> i64 {
        if jobl <= l && jobr >= r {
            self.max_h[i]
        } else {
            let m = l + ((r - l) >> 1);
            self.down(i, m - l + 1, r - m);
            let mut res = i64::MIN;
            if jobl <= m {
                res = res.max(self.q_h_max(jobl, jobr, l, m, i << 1))
            }
            if jobr > m {
                res = res.max(self.q_h_max(jobl, jobr, m + 1, r, i << 1 | 1))
            }
            res
        }
    }
}
