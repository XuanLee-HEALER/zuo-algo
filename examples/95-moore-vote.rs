fn main() {
    println!("res {}", Solution::majority_element_i(vec![3, 3, 4]));
    println!("res {}", Solution::minimum_index(vec![1, 1, 1, 2]));
    println!(
        "res {:?}",
        Solution::majority_element(vec![2, 1, 1, 3, 1, 4, 5, 6])
    );
    let mc = MajorityChecker::new(vec![
        1, 1, 1, 2, 2, 1, 1, 2, 1, 2, 2, 2, 2, 1, 2, 2, 2, 1, 1, 2, 1, 2, 2, 1, 2, 2, 2, 2, 1, 1,
        2, 1, 2, 1, 1, 2, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 1,
    ]);
    println!("op1 -> {}", mc.query(6, 40, 18));
}

struct Solution;

impl Solution {
    pub fn majority_element_i(nums: Vec<i32>) -> i32 {
        let (mut cand, mut hp) = (0, 0);
        nums.iter().for_each(|&v| {
            if hp == 0 {
                cand = v;
                hp += 1;
            } else if cand == v {
                hp += 1;
            } else {
                hp -= 1;
            }
        });
        if hp == 0 {
            -1
        } else {
            if nums.iter().filter(|&v| cand == *v).count() > nums.len() >> 1 {
                cand
            } else {
                -1
            }
        }
    }

    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let (mut cand, mut hp) = (0, 0);
        nums.iter().for_each(|&v| {
            if hp == 0 {
                cand = v;
                hp += 1
            } else if v == cand {
                hp += 1;
            } else {
                hp -= 1;
            }
        });
        let ct = nums.iter().filter(|&v| *v == cand).count();
        let (mut lc, mut rc, mut i) = (0, ct, 0);
        let n = nums.len();
        while i < n - 1 {
            if nums[i] == cand {
                lc += 1;
                rc -= 1
            }
            if lc > (i + 1) >> 1 && rc > (n - i - 1) >> 1 {
                return i as i32;
            }
            i += 1
        }
        -1
    }

    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        Self::me(&nums, 3)
    }

    fn me(nums: &[i32], k: i32) -> Vec<i32> {
        let cmp = nums.len() / k as usize;
        let n = k as usize - 1;
        let mut cands: Vec<(Option<i32>, i32)> = vec![(None, 0); n];
        let is_full = |cands: &[(Option<i32>, i32)]| -> bool {
            cands.iter().filter(|v| v.0.is_none() || v.1 == 0).count() == 0
        };
        let contains = |cands: &[(Option<i32>, i32)], n: i32| -> bool {
            for (cand, hp) in cands {
                if let Some(c) = cand {
                    if *hp != 0 && *c == n {
                        return true;
                    }
                }
            }
            false
        };
        let add = |cands: &mut [(Option<i32>, i32)], n: i32| {
            for (cand, hp) in cands.iter_mut() {
                if let Some(c) = cand {
                    if *c == n {
                        *hp += 1;
                        return;
                    }
                }
            }

            for (cand, hp) in cands.iter_mut() {
                if cand.is_none() || *hp == 0 {
                    *cand = Some(n);
                    *hp = 1;
                    break;
                }
            }
        };
        let remove = |cands: &mut [(Option<i32>, i32)]| {
            for (_, hp) in cands.iter_mut() {
                *hp -= 1;
            }
        };
        for &n in nums {
            if contains(&cands, n) {
                add(&mut cands, n)
            } else if is_full(&cands) {
                remove(&mut cands)
            } else {
                add(&mut cands, n)
            }
        }
        cands
            .iter()
            .filter(|v| v.1 != 0 && nums.iter().filter(|&tv| *tv == v.0.unwrap()).count() > cmp)
            .map(|v| v.0.unwrap())
            .collect()
    }
}

struct MajorityChecker {
    cand: Vec<i32>,
    hp: Vec<usize>,
    ori: Vec<i32>,
    query: Vec<(usize, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MajorityChecker {
    fn new(arr: Vec<i32>) -> Self {
        let n = arr.len();
        let mut query: Vec<(usize, i32)> =
            arr.iter().enumerate().map(|(i, v)| (i + 1, *v)).collect();
        query.sort_unstable_by(|a, b| {
            if a.1 == b.1 {
                a.0.cmp(&b.0)
            } else {
                a.1.cmp(&b.1)
            }
        });
        let mut r = Self {
            cand: vec![0; n << 2],
            hp: vec![0; n << 2],
            ori: arr,
            query,
        };
        r.build(1, n, 1);
        r
    }

    fn build(&mut self, l: usize, r: usize, i: usize) {
        if l == r {
            self.cand[i] = self.ori[l - 1];
            self.hp[i] = 1;
        } else {
            let m = l + ((r - l) >> 1);
            self.build(l, m, i << 1);
            self.build(m + 1, r, i << 1 | 1);
            self.up(i);
        }
    }

    fn up(&mut self, i: usize) {
        let l = i << 1;
        let r = i << 1 | 1;
        if self.cand[l] == self.cand[r] {
            self.cand[i] = self.cand[l];
            self.hp[i] = self.hp[l] + self.hp[r]
        } else {
            match self.hp[l].cmp(&self.hp[r]) {
                std::cmp::Ordering::Less => {
                    self.cand[i] = self.cand[r];
                    self.hp[i] = self.hp[r] - self.hp[l]
                }
                std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => {
                    self.cand[i] = self.cand[l];
                    self.hp[i] = self.hp[l] - self.hp[r]
                }
            }
        }
    }

    fn bs(&self, v: i32, beg: usize) -> usize {
        let (mut l, mut r) = (0, self.query.len() - 1);
        let mut res = None;
        while l <= r {
            let m = l + ((r - l) >> 1);
            if self.query[m].1 < v || (self.query[m].1 == v && self.query[m].0 <= beg) {
                res = Some(m);
                l = m + 1;
            } else {
                if m > l {
                    r = m - 1;
                } else {
                    break;
                }
            }
        }
        res.map(|v| v + 1).unwrap_or(0)
    }

    fn occurrence(&self, l: usize, r: usize, v: i32) -> usize {
        self.bs(v, r) - self.bs(v, l - 1)
    }

    // 范围上的水王数和hp
    fn q(&self, jobl: usize, jobr: usize, l: usize, r: usize, i: usize) -> (i32, usize) {
        if jobl <= l && jobr >= r {
            (self.cand[i], self.hp[i])
        } else {
            let m = l + ((r - l) >> 1);
            if jobr <= m {
                self.q(jobl, jobr, l, m, i << 1)
            } else if jobl > m {
                self.q(jobl, jobr, m + 1, r, i << 1 | 1)
            } else {
                let (lv, lh) = self.q(jobl, jobr, l, m, i << 1);
                let (rv, rh) = self.q(jobl, jobr, m + 1, r, i << 1 | 1);
                if lv == rv {
                    (lv, lh + rh)
                } else {
                    match lh.cmp(&rh) {
                        std::cmp::Ordering::Less => (rv, rh - lh),
                        std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => (lv, lh - rh),
                    }
                }
            }
        }
    }

    fn query(&self, left: i32, right: i32, threshold: i32) -> i32 {
        let left = left as usize + 1;
        let right = right as usize + 1;
        let (v, _) = self.q(left, right, 1, self.ori.len(), 1);
        let r = self.occurrence(left, right, v);
        if r >= threshold as usize { v } else { -1 }
    }
}
