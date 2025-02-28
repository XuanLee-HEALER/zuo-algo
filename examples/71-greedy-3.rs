use std::collections::{BTreeSet, BinaryHeap, HashMap};

fn main() {}

struct Solution;

impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        let mut l = nums.len() as i32;
        let mut r = -1;
        let mut max = i32::MIN;
        let mut min = i32::MAX;
        for (i, &v) in nums.iter().enumerate() {
            if v < max {
                r = i as i32;
            } else {
                max = v;
            }
        }
        for (i, &v) in nums.iter().enumerate().rev() {
            if v > min {
                l = i as i32
            } else {
                min = v
            }
        }
        if r - l < 0 {
            0
        } else {
            r - l + 1
        }
    }

    pub fn smallest_range(nums: Vec<Vec<i32>>) -> Vec<i32> {
        struct Node(i32, usize, usize);
        impl Ord for Node {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                if self.0 == other.0 {
                    self.1.cmp(&other.1)
                } else {
                    self.0.cmp(&other.0)
                }
            }
        }
        impl PartialOrd for Node {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Eq for Node {}
        impl PartialEq for Node {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0 && self.1 == other.1
            }
        }
        let mut tm = BTreeSet::new();
        for (i, n) in nums.iter().enumerate() {
            tm.insert(Node(n[0], i, 0));
        }
        let mut res = vec![tm.first().unwrap().0, tm.last().unwrap().0];
        loop {
            let p = tm.pop_first().unwrap();
            if p.2 + 1 < nums[p.1].len() {
                tm.insert(Node(nums[p.1][p.2 + 1], p.1, p.2 + 1));
                let l = tm.first().unwrap().0;
                let r = tm.last().unwrap().0;
                if r - l < res[1] - res[0] {
                    res[0] = l;
                    res[1] = r;
                }
            } else {
                break;
            }
        }
        res
    }

    fn buy_tickets(costs: Vec<Vec<i32>>, n: i32) -> i32 {
        struct Proj {
            ki: i32,
            bi: i32,
            p: i32,
        }
        impl Proj {
            fn earn(&self) -> i32 {
                self.bi - (1 + self.p) * self.ki - self.p * self.ki
            }
        }
        impl Ord for Proj {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.earn().cmp(&other.earn())
            }
        }
        impl PartialOrd for Proj {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Eq for Proj {}
        impl PartialEq for Proj {
            fn eq(&self, other: &Self) -> bool {
                self.earn() == other.earn()
            }
        }
        let mut max_heap = BinaryHeap::new();
        for cost in costs {
            max_heap.push(Proj {
                ki: cost[0],
                bi: cost[1],
                p: 0,
            });
        }
        let mut n = n;
        let mut res = 0;
        while n > 0 {
            let mut p = max_heap.pop().unwrap();
            if p.earn() <= 0 {
                break;
            }
            res += p.earn();
            p.p += 1;
            max_heap.push(p);
            n -= 1;
        }
        res
    }

    fn dumb_buy_tickets(costs: Vec<Vec<i32>>, n: i32) -> i32 {
        let mut mem = 0;
        let mut ct = HashMap::new();
        for i in 0..costs.len() {
            ct.insert(i, 0);
        }
        Self::dbt(1, n, &costs, &mut ct, &mut mem)
    }

    fn dbt(
        i: i32,
        n: i32,
        costs: &[Vec<i32>],
        cur_p: &mut HashMap<usize, i32>,
        t_res: &mut i32,
    ) -> i32 {
        if i > n {
            if *t_res > 0 {
                *t_res
            } else {
                0
            }
        } else {
            let mut p = Self::dbt(i + 1, n, costs, cur_p, t_res);
            for (ti, c) in costs.iter().enumerate() {
                let xp = cur_p.get(&ti).unwrap();
                let earn = c[1] - c[0] * (*xp + 1) - c[0] * *xp;
                *t_res += earn;
                cur_p.entry(ti).and_modify(|v| *v += 1);
                p = p.max(Self::dbt(i + 1, n, costs, cur_p, t_res));
                *t_res -= earn;
                cur_p.entry(ti).and_modify(|v| *v -= 1);
            }
            p
        }
    }

    fn min_avg_sum(arr: Vec<i32>, k: i32) -> i32 {
        let mut arr = arr;
        arr.sort_unstable();
        let mut res = arr.iter().take((k - 1) as usize).sum();
        res += (arr.iter().skip((k - 1) as usize).sum::<i32>())
            / (arr.len() - (k as usize - 1)) as i32;
        res
    }

    fn dumb_min_avg_sum(arr: Vec<i32>, k: i32) -> i32 {
        let mut g_sum = vec![0; k as usize];
        let mut g_cnt = vec![0; k as usize];
        Self::dmas(0, &arr, k, &mut g_sum, &mut g_cnt)
    }

    fn dmas(i: usize, arr: &[i32], k: i32, t_sum: &mut [i32], t_cnt: &mut [i32]) -> i32 {
        if i == arr.len() {
            let mut res = 0;
            for j in 0..k as usize {
                if t_cnt[j] == 0 {
                    return i32::MAX;
                } else {
                    res += t_sum[j] / t_cnt[j]
                }
            }
            res
        } else {
            let mut res = i32::MAX;
            for j in 0..k as usize {
                t_sum[j] += arr[i];
                t_cnt[j] += 1;
                res = res.min(Self::dmas(i + 1, arr, k, t_sum, t_cnt));
                t_sum[j] -= arr[i];
                t_cnt[j] -= 1;
            }
            res
        }
    }

    fn min_battery(tasks: Vec<Vec<i32>>) -> i32 {
        let mut tasks = tasks;
        tasks.sort_unstable_by(|a, b| (b[0] - b[1]).cmp(&(a[0] - a[1])));
        let mut res = 0;
        for task in tasks {
            res = (res + task[0]).max(task[1])
        }
        res
    }

    fn dumb_min_battery(tasks: Vec<Vec<i32>>) -> i32 {
        let mut tasks = tasks;
        Self::dmb(0, tasks.len(), &mut tasks)
    }

    fn dmb(i: usize, n: usize, tasks: &mut [Vec<i32>]) -> i32 {
        if i == n {
            let mut res = 0;
            let mut cur = 0;
            for task in tasks.iter() {
                if task[1] > cur {
                    res += task[1] - cur;
                    cur = task[1];
                }
                if task[0] > cur {
                    res += task[0] - cur;
                    cur = task[0];
                }
                cur -= task[0];
            }
            res
        } else {
            let mut res = i32::MAX;
            for ti in i..n {
                tasks.swap(i, ti);
                res = res.min(Self::dmb(i + 1, n, tasks));
                tasks.swap(i, ti);
            }
            res
        }
    }

    fn max_01_substr(s: String) -> i32 {
        let mut l0 = 0;
        let mut l1 = 0;
        let mut r0 = 0;
        let mut r1 = 0;
        let mut f1 = false;
        let mut f2 = false;
        let s = s.chars().collect::<Vec<char>>();
        for (i, &c) in s.iter().enumerate() {
            if !f1 && c == '0' {
                l0 = i;
                f1 = true
            }
            if !f2 && c == '1' {
                l1 = i;
                f2 = true
            }
            if f1 && f2 {
                break;
            }
        }
        f1 = false;
        f2 = false;
        for (i, &c) in s.iter().enumerate().rev() {
            if !f1 && c == '0' {
                r0 = i;
                f1 = true
            }
            if !f2 && c == '1' {
                r1 = i;
                f2 = true
            }
            if f1 && f2 {
                break;
            }
        }
        (r0 - l0).max(r1 - l1) as i32
    }

    fn dumb_max_01_substr(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut res = 0;
        for i in 0..s.len() {
            for j in i..s.len() {
                let p1 = Self::counter(&s[i..=j]);
                for m in 0..s.len() {
                    for n in m..s.len() {
                        if i != m
                            && j != n
                            && (n - m + 1) == (j - i + 1)
                            && p1 == Self::counter(&s[m..=n])
                        {
                            res = res.max(j - i + 1)
                        }
                    }
                }
            }
        }
        res as i32
    }

    fn counter(cs: &[char]) -> (usize, usize) {
        let mut res = (0, 0);
        for &c in cs {
            if c == '0' {
                res.0 += 1;
            }
            if c == '1' {
                res.1 += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod test_solution {
    use rand::{thread_rng, Rng};

    use crate::Solution;

    #[test]
    fn test_buy_tickets() {
        let mut rng = thread_rng();
        for _ in 0..1_000 {
            let mut costs = vec![];
            let m = rng.gen_range(1..=6);
            let n = rng.gen_range(1..=6);
            for _ in 0..m {
                let v1 = rng.gen_range(1..=500);
                let v2 = rng.gen_range(1..=500);
                costs.push(vec![v1, v2]);
            }
            let bright = Solution::buy_tickets(costs.clone(), n);
            let dumb = Solution::dumb_buy_tickets(costs.clone(), n);
            assert_eq!(
                bright, dumb,
                "bright: {} dumb: {}\ncosts: {:?}\nm: {} n: {}",
                bright, dumb, costs, m, n
            )
        }
    }

    #[test]
    fn test_min_avg_sum() {
        let mut rng = thread_rng();
        for _ in 0..1_000 {
            let n = rng.gen_range(2..=8);
            let mut v = vec![];
            for _ in 0..n {
                v.push(rng.gen_range(1..=20_000));
            }
            let k = rng.gen_range(1..=n);
            let bright = super::Solution::min_avg_sum(v.clone(), k);
            let dumb = super::Solution::dumb_min_avg_sum(v.clone(), k);
            assert_eq!(
                bright, dumb,
                "bright {}, dumb {}, n {}, k {}, v {:?}",
                bright, dumb, n, k, v
            )
        }
    }

    #[test]
    fn test_min_battery() {
        let mut rng = thread_rng();
        for _ in 0..1_000 {
            let n = rng.gen_range(2..=8);
            let mut v = vec![];
            for _ in 0..n {
                v.push(vec![rng.gen_range(1..=20_000), rng.gen_range(1..=20_000)]);
            }
            let bright = super::Solution::min_battery(v.clone());
            let dumb = super::Solution::dumb_min_battery(v.clone());
            assert_eq!(bright, dumb, "bright {}, dumb {}, v {:?}", bright, dumb, v)
        }
    }

    #[test]
    fn test_max_01_substr() {
        let mut rng = thread_rng();
        for _ in 0..1_000 {
            let n = rng.gen_range(5..=30);
            let mut v = String::with_capacity(n);
            for _ in 0..n {
                if rng.gen_bool(0.5) {
                    v.push('0');
                } else {
                    v.push('1');
                }
            }
            let bright = super::Solution::max_01_substr(v.clone());
            let dumb = super::Solution::dumb_max_01_substr(v.clone());
            assert_eq!(bright, dumb, "bright {}, dumb {}, v {:?}", bright, dumb, v)
        }
    }
}
