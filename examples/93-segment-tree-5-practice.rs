use std::{i64, usize};

fn main() {
    let mut ci = CountIntervals::new();
    ci.add(2, 3);
    ci.add(7, 10);
    println!("{:?}", ci.sum);
    println!("{}", ci.count())
}

struct CountIntervals {
    cnt: usize,
    left: Vec<usize>,
    right: Vec<usize>,
    sum: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CountIntervals {
    const N: usize = 4_000_000;
    const MAX_N: usize = 1_000_000_001;

    fn new() -> Self {
        let mut left = Vec::with_capacity(Self::N);
        let mut right = Vec::with_capacity(Self::N);
        let mut sum = Vec::with_capacity(Self::N);
        left.push(0);
        left.push(0);
        right.push(0);
        right.push(0);
        sum.push(0);
        sum.push(0);
        Self {
            cnt: 1,
            left,
            right,
            sum,
        }
    }

    fn set_v(&mut self, jobl: usize, jobr: usize, l: usize, r: usize, i: usize) {
        if self.sum[i] == (r - l + 1) as i32 {
            return;
        }
        if jobl <= l && jobr >= r {
            self.sum[i] = (r - l + 1) as i32
        } else {
            let m = l + ((r - l) >> 1);
            if jobl <= m {
                if self.left[i] == 0 {
                    self.cnt += 1;
                    self.left[i] = self.cnt;
                    self.left.push(0);
                    self.right.push(0);
                    self.sum.push(0);
                }
                self.set_v(jobl, jobr, l, m, self.left[i]);
            }

            if jobr > m {
                if self.right[i] == 0 {
                    self.cnt += 1;
                    self.right[i] = self.cnt;
                    self.left.push(0);
                    self.right.push(0);
                    self.sum.push(0);
                }
                self.set_v(jobl, jobr, m + 1, r, self.right[i]);
            }

            self.sum[i] = self.sum[self.left[i]] + self.sum[self.right[i]]
        }
    }

    fn add(&mut self, left: i32, right: i32) {
        self.set_v(left as usize, right as usize, 1, Self::MAX_N, 1);
    }

    fn count(&self) -> i32 {
        self.sum[1]
    }
}

/**
 * 区间最值线段树，求一个范围上的最大值和累加和
 * 每次会将一个区域更新为 当前值和指定值的最小值
 */
struct RangeSegmentTree {
    sum: Vec<i64>,
    max: Vec<i64>,
    max_s: Vec<usize>,
    sec: Vec<i64>,
}

impl RangeSegmentTree {
    fn new(n: usize, min_val: i64) -> Self {
        Self {
            sum: vec![0; n << 2],
            max: vec![0; n << 2],
            max_s: vec![0; n << 2],
            sec: vec![min_val - 1; n << 2],
        }
    }

    fn build(&mut self, ori: &[i64], l: usize, r: usize, i: usize) {
        if l == r {
            self.sum[i] = ori[l - 1];
            self.max[i] = ori[l - 1];
            self.max_s[i] = 1;
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
        self.sum[i] = self.sum[l] + self.sum[r];
        self.max[i] = self.max[l].max(self.max[r]);
        if self.max[l] > self.max[r] {
            self.max_s[i] = self.max_s[l];
            self.sec[i] = self.max[r].max(self.sec[l])
        } else if self.max[l] < self.max[r] {
            self.max_s[i] = self.max_s[r];
            self.sec[i] = self.max[l].max(self.sec[r])
        } else {
            self.max_s[i] = self.max_s[l] + self.max_s[r];
            self.sec[i] = self.sec[l].max(self.sec[r])
        }
    }

    fn lazy(&mut self, i: usize, v: i64) {
        if v < self.max[i] {
            self.sum[i] -= (self.max[i] - v) * self.max_s[i] as i64;
            self.max[i] = v;
        }
    }

    fn down(&mut self, i: usize) {
        self.lazy(i << 1, self.max[i]);
        self.lazy(i << 1 | 1, self.max[i]);
    }

    fn min(&mut self, jobl: usize, jobr: usize, jobv: i64, l: usize, r: usize, i: usize) {
        if jobv >= self.max[i] {
            return;
        }
        if jobl <= l && jobr >= r && jobv > self.sec[i] {
            self.lazy(i, jobv);
        } else {
            let m = l + ((r - l) >> 1);
            self.down(i);
            if jobl <= m {
                self.min(jobl, jobr, jobv, l, m, i << 1);
            }
            if jobr > m {
                self.min(jobl, jobr, jobv, m + 1, r, i << 1 | 1);
            }
            self.up(i);
        }
    }

    fn query_max(&mut self, jobl: usize, jobr: usize, l: usize, r: usize, i: usize) -> i64 {
        if jobl <= l && jobr >= r {
            self.max[i]
        } else {
            let m = l + ((r - l) >> 1);
            self.down(i);
            let mut res = i64::MIN;
            if jobl <= m {
                res = res.max(self.query_max(jobl, jobr, l, m, i << 1));
            }
            if jobr > m {
                res = res.max(self.query_max(jobl, jobr, m + 1, r, i << 1 | 1))
            }
            res
        }
    }

    fn query_sum(&mut self, jobl: usize, jobr: usize, l: usize, r: usize, i: usize) -> i64 {
        if jobl <= l && jobr >= r {
            self.sum[i]
        } else {
            let m = l + ((r - l) >> 1);
            self.down(i);
            let mut res = 0;
            if jobl <= m {
                res += self.query_sum(jobl, jobr, l, m, i << 1);
            }
            if jobr > m {
                res += self.query_sum(jobl, jobr, m + 1, r, i << 1 | 1)
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{Rng, thread_rng};

    use crate::RangeSegmentTree;

    fn batch_set(arr: &mut [i64], mut l: usize, mut r: usize, v: i64) {
        l = l - 1;
        r = r - 1;
        arr.iter_mut().enumerate().for_each(|(i, tv)| {
            if i >= l && i <= r && *tv > v {
                *tv = v
            }
        });
    }

    fn q_sum(arr: &[i64], l: usize, r: usize) -> i64 {
        arr.iter().skip(l - 1).take(r - l + 1).sum()
    }

    fn q_max(arr: &[i64], l: usize, r: usize) -> i64 {
        *arr.iter().skip(l - 1).take(r - l + 1).max().unwrap()
    }

    #[test]
    fn test_range_segment_tree() {
        const ARR_LEN: usize = 1000;
        const MIN_VAL: i64 = -10;
        const MAX_VAL: i64 = 100;
        const TEST_TIMES: usize = 1000;
        const X_MIN_VAL: i64 = -5000;
        const OP_TIMES: usize = 1000;
        let mut rng = thread_rng();
        for _ in 0..TEST_TIMES {
            let mut ori = Vec::with_capacity(ARR_LEN);
            for _ in 0..ARR_LEN {
                ori.push(rng.gen_range(MIN_VAL..=MAX_VAL));
            }
            let mut rst = RangeSegmentTree::new(ARR_LEN, X_MIN_VAL);
            rst.build(&ori, 1, ARR_LEN, 1);
            for _ in 0..OP_TIMES {
                let l: usize = rng.gen_range(1..=ARR_LEN);
                let r: usize = rng.gen_range(l..=ARR_LEN);
                let v: i64 = rng.gen_range(X_MIN_VAL..=MAX_VAL);
                if rng.gen_ratio(1, 3) {
                    rst.min(l, r, v, 1, ARR_LEN, 1);
                    batch_set(&mut ori, l, r, v);
                } else if rng.gen_ratio(1, 2) {
                    assert_eq!(
                        q_sum(&ori, l, r),
                        rst.query_sum(l, r, 1, ARR_LEN, 1),
                        "[query sum] ori array is {:?}, l is {l}, r is {r}",
                        ori
                    )
                } else {
                    assert_eq!(
                        q_max(&ori, l, r),
                        rst.query_max(l, r, 1, ARR_LEN, 1),
                        "[query max] ori array is {:?}, l is {l}, r is {r}",
                        ori
                    )
                }
            }
        }
    }

    #[test]
    fn test_single_rst() {
        let ori: Vec<i64> = vec![1, 1, 1, 1, 1];
        let mut rst = RangeSegmentTree::new(5, -5);
        rst.build(&ori, 1, 5, 1);
        // 0 0 0 1 1
        rst.min(1, 3, 0, 1, 5, 1);
        // 0 -2 -2 1 1
        rst.min(2, 3, -2, 1, 5, 1);
        // -3 -2 -2 1 1
        rst.min(1, 1, -3, 1, 5, 1);
        println!("sum {}", rst.query_sum(1, 1, 1, 5, 1));
        println!("max {}", rst.query_max(1, 3, 1, 5, 1))
    }
}
