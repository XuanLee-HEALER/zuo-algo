use rand::{Rng, thread_rng};

fn main() {
    const VASE_NUM: usize = 3_000;
    let mut vases = vec![0; VASE_NUM + 1];
    let operations = 1_000_000;
    let mut rng = thread_rng();
    let mut p_vases = Vases::new(VASE_NUM);
    for _ in 0..operations {
        if rng.gen_ratio(2, 3) {
            // 插画
            let from = rng.gen_range(1..=VASE_NUM);
            let flowers = rng.gen_range(1..=10_000);
            assert_eq!(
                ikebana(&mut vases, from, flowers),
                p_vases.ikebana(from, flowers),
                "from {from}, flowers {flowers}"
            )
        } else {
            // 清除花
            let left = rng.gen_range(1..=VASE_NUM);
            let right = rng.gen_range(left..=VASE_NUM);
            assert_eq!(remove(&mut vases, left, right), p_vases.remove(left, right))
        }
    }
}

struct Vases {
    st: STResetSum,
    n: usize,
}

impl Vases {
    fn new(vases_num: usize) -> Self {
        Self {
            st: STResetSum::new(vases_num),
            n: vases_num,
        }
    }

    fn ikebana(&mut self, from: usize, flowers: usize) -> Option<(usize, usize)> {
        let with_f = self.st.query(from, self.n, 1, self.n, 1);
        let blank = self.n - from + 1 - with_f as usize;
        if blank == 0 {
            None
        } else {
            let l = self.find_zero(from, 1);
            let r = self.find_zero(from, blank.min(flowers));
            self.st.reset(l, r, 1, 1, self.n, 1);
            Some((l, r))
        }
    }

    fn find_zero(&mut self, from: usize, nth: usize) -> usize {
        let mut l = from;
        let mut r = self.n;
        let mut res = 0;
        while l <= r {
            let m = l + ((r - l) >> 1);
            // l~m范围上有多少个空瓶？如果大于等于nth，记录m为答案，更新r，往左找，否则将m扩大到右边的中点
            if m - from + 1 - self.st.query(from, m, 1, self.n, 1) as usize >= nth {
                res = m;
                if m - 1 >= l {
                    r = m - 1
                } else {
                    break;
                }
            } else {
                l = m + 1
            }
        }
        res
    }

    fn remove(&mut self, left: usize, right: usize) -> i32 {
        let res = self.st.query(left, right, 1, self.n, 1);
        self.st.reset(left, right, 0, 1, self.n, 1);
        res
    }
}

fn ikebana(vases: &mut [i32], from: usize, mut flowers: usize) -> Option<(usize, usize)> {
    let mut res = (0, 0);
    let mut ct = 0;
    let mut f = false;
    for (i, v) in vases.iter_mut().enumerate().skip(from) {
        if flowers > 0 && *v == 0 {
            ct += 1;
            if !f {
                res.0 = i;
                res.1 = i;
                f = true
            } else {
                res.1 = i
            }
            *v = 1;
            flowers -= 1;
        }
    }
    if ct == 0 { None } else { Some(res) }
}

fn remove(vases: &mut [i32], left: usize, right: usize) -> i32 {
    let res: i32 = vases[left..=right].iter().sum();
    vases[left..=right].iter_mut().for_each(|v| *v = 0);
    res
}

fn m(l: usize, r: usize) -> usize {
    l + ((r - l) >> 1)
}

struct STResetSum {
    info: Vec<i32>,
    change: Vec<i32>,
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
    fn lazy(&mut self, i: usize, n: usize, v: i32) {
        self.update[i] = true;
        self.change[i] = v;
        self.info[i] = v * n as i32
    }

    fn reset(&mut self, jobl: usize, jobr: usize, jobv: i32, l: usize, r: usize, i: usize) {
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

    fn query(&mut self, jobl: usize, jobr: usize, l: usize, r: usize, i: usize) -> i32 {
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
