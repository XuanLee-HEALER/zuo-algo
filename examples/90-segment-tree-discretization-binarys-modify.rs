fn main() {
    println!(
        "{:?}",
        Solution::falling_squares(vec![vec![1, 2], vec![2, 3], vec![6, 1]])
    )
}

struct Solution;

impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut look_up = Vec::new();
        look_up.push(-1);
        for position in &positions {
            let (lefti, leni) = (position[0], position[1]);
            look_up.push(lefti);
            look_up.push(lefti + leni - 1);
        }
        look_up.sort_unstable();
        look_up.dedup();
        let n = look_up.len() - 1;
        let mut st = STResetMax::new(n);
        let mut res = Vec::with_capacity(positions.len());
        for task in &positions {
            let (lefti, leni) = (task[0], task[1]);
            let righti = lefti + leni - 1;
            let job_left = look_up.binary_search(&lefti).unwrap();
            let job_right = look_up.binary_search(&righti).unwrap();
            let o_h = st.query(job_left, job_right, 1, n, 1);
            let n_h = o_h + leni;
            st.reset(job_left, job_right, n_h, 1, n, 1);
            res.push(st.query(1, n, 1, n, 1));
        }
        res
    }
}

fn m(l: usize, r: usize) -> usize {
    l + ((r - l) >> 1)
}

struct STResetMax {
    info: Vec<i32>,
    change: Vec<i32>,
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

    fn build(&mut self, ori: &[i32], l: usize, r: usize, i: usize) {
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
    fn lazy(&mut self, i: usize, v: i32) {
        self.update[i] = true;
        self.change[i] = v;
        self.info[i] = v
    }

    fn reset(&mut self, jobl: usize, jobr: usize, jobv: i32, l: usize, r: usize, i: usize) {
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

    fn query(&mut self, jobl: usize, jobr: usize, l: usize, r: usize, i: usize) -> i32 {
        if jobl <= l && jobr >= r {
            self.info[i]
        } else {
            let m = m(l, r);
            self.down(i);
            let mut res = i32::MIN;
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
