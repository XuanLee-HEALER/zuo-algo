use std::i64;

fn main() {}

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

struct STResetSum {
    info: Vec<i64>,
    change: Vec<i64>,
    update: Vec<bool>,
}

impl STResetSum {
    fn new(n: usize) -> Self {
        Self {
            info: vec![0; n << 2],
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
        self.info[i] = self.info[i << 1] + self.info[i << 1 | 1]
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
    fn lazy(&mut self, i: usize, n: usize, v: i64) {
        self.update[i] = true;
        self.change[i] = v;
        self.info[i] = v * n as i64
    }

    fn reset(&mut self, jobl: usize, jobr: usize, jobv: i64, l: usize, r: usize, i: usize) {
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

struct STAddMax {
    info: Vec<i64>,
    add: Vec<i64>,
}

impl STAddMax {
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
        self.info[i] = self.info[i << 1].max(self.info[i << 1 | 1])
    }

    // 将某个位置的懒信息传递下去
    fn down(&mut self, i: usize) {
        // 如果有懒信息
        if self.add[i] != 0 {
            let v = self.add[i];
            self.lazy(i << 1, v);
            self.lazy(i << 1 | 1, v);
            // 重置当前的懒信息
            self.add[i] = 0;
        }
    }

    // 懒信息要加到某个范围上，需要同步更新结果信息
    fn lazy(&mut self, i: usize, v: i64) {
        self.add[i] += v;
        self.info[i] += v
    }

    fn add(&mut self, jobl: usize, jobr: usize, jobv: i64, l: usize, r: usize, i: usize) {
        // 任务范围完全包括当前范围，那么整个范围只要累积懒信息即可
        if jobl <= l && jobr >= r {
            self.lazy(i, jobv);
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

struct STResetMax {
    info: Vec<i64>,
    change: Vec<i64>,
    update: Vec<bool>,
}

impl STResetMax {
    fn new(n: usize) -> Self {
        Self {
            info: vec![0; n << 2],
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
        // 如果有懒信息
        if self.update[i] {
            let v = self.change[i];
            self.lazy(i << 1, v);
            self.lazy(i << 1 | 1, v);
            // 重置当前的懒信息
            self.update[i] = false;
        }
    }

    // 懒信息要加到某个范围上，需要同步更新结果信息
    fn lazy(&mut self, i: usize, v: i64) {
        self.update[i] = true;
        self.change[i] = v;
        self.info[i] = v
    }

    fn reset(&mut self, jobl: usize, jobr: usize, jobv: i64, l: usize, r: usize, i: usize) {
        // 任务范围完全包括当前范围，那么整个范围只要累积懒信息即可
        if jobl <= l && jobr >= r {
            self.lazy(i, jobv);
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

struct STAddResetSum {
    info: Vec<i64>,
    add: Vec<i64>,
    change: Vec<i64>,
    update: Vec<bool>,
}

impl STAddResetSum {
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
        self.info[i] = self.info[i << 1] + self.info[i << 1 | 1]
    }

    // 将某个位置的懒信息传递下去
    fn down(&mut self, i: usize, ln: usize, rn: usize) {
        if self.update[i] {
            let v = self.change[i];
            self.lazy_reset(i << 1, ln, v);
            self.lazy_reset(i << 1 | 1, rn, v);
            // 重置当前的懒信息
            self.update[i] = false;
        }

        if self.add[i] != 0 {
            let v = self.add[i];
            self.lazy_add(i << 1, ln, v);
            self.lazy_add(i << 1 | 1, rn, v);
            self.add[i] = 0;
        }
    }

    fn lazy_add(&mut self, i: usize, n: usize, v: i64) {
        self.add[i] += v;
        self.info[i] += v * n as i64
    }

    fn lazy_reset(&mut self, i: usize, n: usize, v: i64) {
        // 之前的增加懒信息要被清除
        self.add[i] = 0;
        self.update[i] = true;
        self.change[i] = v;
        self.info[i] = v * n as i64;
    }

    fn add(&mut self, jobl: usize, jobr: usize, jobv: i64, l: usize, r: usize, i: usize) {
        // 任务范围完全包括当前范围，那么整个范围只要累积懒信息即可
        if jobl <= l && jobr >= r {
            self.lazy_add(i, r - l + 1, jobv);
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

    fn reset(&mut self, jobl: usize, jobr: usize, jobv: i64, l: usize, r: usize, i: usize) {
        // 任务范围完全包括当前范围，那么整个范围只要累积懒信息即可
        if jobl <= l && jobr >= r {
            self.lazy_reset(i, r - l + 1, jobv);
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

#[cfg(test)]
mod tests {
    use rand::{Rng, rngs::ThreadRng, thread_rng};

    use crate::{STAddMax, STAddResetMax, STAddResetSum, STAddSum, STResetMax, STResetSum};

    fn range_add(arr: &mut [i64], jobl: usize, jobr: usize, v: i64) {
        arr[jobl - 1..jobr].iter_mut().for_each(|e| *e += v)
    }

    fn range_reset(arr: &mut [i64], jobl: usize, jobr: usize, v: i64) {
        arr[jobl - 1..jobr].iter_mut().for_each(|e| *e = v)
    }

    fn range_sum(arr: &[i64], jobl: usize, jobr: usize) -> i64 {
        arr[jobl - 1..jobr].iter().sum()
    }

    fn range_max(arr: &[i64], jobl: usize, jobr: usize) -> i64 {
        *arr[jobl - 1..jobr].iter().max().unwrap()
    }

    fn random_arr(rng: &mut ThreadRng, n: usize) -> Vec<i64> {
        let mut res = Vec::with_capacity(n);
        for _ in 0..n {
            res.push(rng.gen_range(-10_000_000..=10_000_000))
        }
        res
    }

    #[test]
    fn test_st_add_sum() {
        let test_time = 300;
        let arr_len = 2_000;
        let task_len = 5_000;
        let mut rng = thread_rng();
        let mut a = random_arr(&mut rng, arr_len);
        let mut st = STAddSum::new(arr_len);
        st.build(&a, 1, arr_len, 1);
        for _ in 0..test_time {
            for _ in 0..task_len {
                let r1 = rng.gen_range(1..=arr_len);
                let r2 = rng.gen_range(r1..=arr_len);
                let op = rng.gen_ratio(2, 3);
                if op {
                    let v = rng.gen_range(-1_000_000..=1_000_000);
                    st.add(r1, r2, v, 1, arr_len, 1);
                    range_add(&mut a, r1, r2, v);
                } else {
                    assert_eq!(range_sum(&a, r1, r2), st.query(r1, r2, 1, arr_len, 1))
                }
            }
        }
    }

    #[test]
    fn test_st_add_max() {
        let test_time = 300;
        let arr_len = 2_000;
        let task_len = 5_000;
        let mut rng = thread_rng();
        let mut a = random_arr(&mut rng, arr_len);
        let mut st = STAddMax::new(arr_len);
        st.build(&a, 1, arr_len, 1);
        for _ in 0..test_time {
            for _ in 0..task_len {
                let r1 = rng.gen_range(1..=arr_len);
                let r2 = rng.gen_range(r1..=arr_len);
                let op = rng.gen_ratio(2, 3);
                if op {
                    let v = rng.gen_range(-1_000_000..=1_000_000);
                    st.add(r1, r2, v, 1, arr_len, 1);
                    range_add(&mut a, r1, r2, v);
                } else {
                    assert_eq!(range_max(&a, r1, r2), st.query(r1, r2, 1, arr_len, 1))
                }
            }
        }
    }

    #[test]
    fn test_st_reset_sum() {
        let test_time = 300;
        let arr_len = 2_000;
        let task_len = 5_000;
        let mut rng = thread_rng();
        let mut a = random_arr(&mut rng, arr_len);
        let mut st = STResetSum::new(arr_len);
        st.build(&a, 1, arr_len, 1);
        for _ in 0..test_time {
            for _ in 0..task_len {
                let r1 = rng.gen_range(1..=arr_len);
                let r2 = rng.gen_range(r1..=arr_len);
                let op = rng.gen_ratio(2, 3);
                if op {
                    let v = rng.gen_range(-1_000_000..=1_000_000);
                    st.reset(r1, r2, v, 1, arr_len, 1);
                    range_reset(&mut a, r1, r2, v);
                } else {
                    assert_eq!(range_sum(&a, r1, r2), st.query(r1, r2, 1, arr_len, 1))
                }
            }
        }
    }

    #[test]
    fn test_st_reset_max() {
        let test_time = 300;
        let arr_len = 2_000;
        let task_len = 5_000;
        let mut rng = thread_rng();
        let mut a = random_arr(&mut rng, arr_len);
        let mut st = STResetMax::new(arr_len);
        for _ in 0..test_time {
            st.build(&a, 1, arr_len, 1);
            for _ in 0..task_len {
                let r1 = rng.gen_range(1..=arr_len);
                let r2 = rng.gen_range(r1..=arr_len);
                let op = rng.gen_ratio(2, 3);
                if op {
                    let v = rng.gen_range(-1_000_000..=1_000_000);
                    st.reset(r1, r2, v, 1, arr_len, 1);
                    range_reset(&mut a, r1, r2, v);
                } else {
                    assert_eq!(range_max(&a, r1, r2), st.query(r1, r2, 1, arr_len, 1))
                }
            }
        }
    }

    #[test]
    fn test_st_add_reset_sum() {
        let test_time = 300;
        let arr_len = 2_000;
        let task_len = 5_000;
        let mut rng = thread_rng();
        let mut a = random_arr(&mut rng, arr_len);
        let mut st = STAddResetSum::new(arr_len);
        st.build(&a, 1, arr_len, 1);
        for _ in 0..test_time {
            for _ in 0..task_len {
                let r1 = rng.gen_range(1..=arr_len);
                let r2 = rng.gen_range(r1..=arr_len);
                let op = rng.gen_ratio(2, 3);
                if op {
                    let v = rng.gen_range(-1_000_000..=1_000_000);
                    st.reset(r1, r2, v, 1, arr_len, 1);
                    range_reset(&mut a, r1, r2, v);
                } else {
                    if rng.gen_ratio(1, 2) {
                        let v = rng.gen_range(-1_000_000..=1_000_000);
                        st.add(r1, r2, v, 1, arr_len, 1);
                        range_add(&mut a, r1, r2, v);
                    } else {
                        assert_eq!(range_sum(&a, r1, r2), st.query(r1, r2, 1, arr_len, 1))
                    }
                }
            }
        }
    }

    #[test]
    fn test_st_add_reset_max() {
        let test_time = 300;
        let arr_len = 2_000;
        let task_len = 5_000;
        let mut rng = thread_rng();
        let mut a = random_arr(&mut rng, arr_len);
        let mut st = STAddResetMax::new(arr_len);
        st.build(&a, 1, arr_len, 1);
        for _ in 0..test_time {
            for _ in 0..task_len {
                let r1 = rng.gen_range(1..=arr_len);
                let r2 = rng.gen_range(r1..=arr_len);
                let op = rng.gen_ratio(2, 3);
                if op {
                    let v = rng.gen_range(-1_000_000..=1_000_000);
                    st.reset(r1, r2, v, 1, arr_len, 1);
                    range_reset(&mut a, r1, r2, v);
                } else {
                    if rng.gen_ratio(1, 2) {
                        let v = rng.gen_range(-1_000_000..=1_000_000);
                        st.add(r1, r2, v, 1, arr_len, 1);
                        range_add(&mut a, r1, r2, v);
                    } else {
                        assert_eq!(range_max(&a, r1, r2), st.query(r1, r2, 1, arr_len, 1))
                    }
                }
            }
        }
    }
}
