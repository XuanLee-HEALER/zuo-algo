use std::collections::{BinaryHeap, HashMap};

use rand::{thread_rng, Rng};

fn main() {
    for _ in 0..1000 {
        let r = Solution::random_array(5, 4, 4);
        assert_eq!(
            Solution::dumb_min_dic_seq(r.clone()),
            Solution::min_dic_seq(r.clone())
        );
    }
}

struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums = nums.iter().map(|i| i.to_string()).collect::<Vec<String>>();
        nums.sort_by(|a, b| format!("{}{}", b, a).cmp(&format!("{}{}", a, b)));
        let res = nums.join("");
        if res.starts_with("0") {
            "0".into()
        } else {
            res
        }
    }

    fn dumb_min_dic_seq(ori: Vec<String>) -> String {
        let mut all_perm = Vec::new();
        Self::permutation(&mut ori.to_vec(), 0, &mut all_perm);
        let mut all_perm = all_perm.iter().map(|v| v.join("")).collect::<Vec<String>>();
        all_perm.sort();
        all_perm.first().unwrap().clone()
    }

    fn permutation(old: &mut Vec<String>, i: usize, res: &mut Vec<Vec<String>>) {
        if i == old.len() {
            res.push(old.clone());
        } else {
            for j in i..old.len() {
                old.swap(i, j);
                Self::permutation(old, i + 1, res);
                old.swap(i, j);
            }
        }
    }

    fn min_dic_seq(ori: Vec<String>) -> String {
        let mut ori = ori;
        ori.sort_by(|a, b| format!("{}{}", a, b).cmp(&format!("{}{}", b, a)));
        ori.join("")
    }

    // m 是数组最长长度
    // n 是数组字符串的最长长度
    // t 是数组字符种类的个数
    fn random_array(m: usize, n: usize, t: usize) -> Vec<String> {
        let mut rng = thread_rng();
        let rm = rng.gen_range(1..=m);
        let rn = rng.gen_range(1..=n);
        let rt = rng.gen_range(1..=t);
        let mut res = Vec::new();
        for _ in 0..rm {
            let mut t_str = String::new();
            for _ in 0..rn {
                let c = b'a' + rng.gen_range(0..rt) as u8;
                t_str.push(c as char);
            }
            res.push(t_str);
        }
        res
    }

    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut dif = vec![];
        for cost in &costs {
            res += cost[0];
            dif.push(cost[1] - cost[0]);
        }
        dif.sort();
        res + dif.iter().take(dif.len() / 2).sum::<i32>()
    }

    pub fn min_days(n: i32) -> i32 {
        Self::md(n, &mut HashMap::new())
    }

    fn md(n: i32, dp: &mut HashMap<i32, i32>) -> i32 {
        if n <= 1 {
            n
        } else if dp.contains_key(&n) {
            *dp.get(&n).unwrap()
        } else {
            let res = (n % 2 + 1 + Self::md(n / 2, dp)).min(n % 3 + 1 + Self::md(n / 3, dp));
            dp.insert(n, res);
            res
        }
    }

    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut courses = courses;
        courses.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut max_heap = BinaryHeap::new();
        let mut beg = 0;
        for course in courses {
            let (dur, last) = (course[0], course[1]);
            if beg + dur <= last {
                max_heap.push(dur);
                beg += dur;
            } else if !max_heap.is_empty() && dur < *max_heap.peek().unwrap() {
                beg += dur - max_heap.pop().unwrap();
                max_heap.push(dur);
            }
        }
        max_heap.len() as i32
    }
}
