use std::collections::{BTreeSet, BinaryHeap};

fn main() {
    println!("res {}", Solution::minimum_deviation(vec![3, 5]));
    let mut v = vec![3, 2, 2, 2, 1];
    println!("res {} {:?}", Solution::split(&mut v), v);
    let mut arr = vec![1, 3, 2, 1, 2, 2, 5, 6];
    arr.sort();
    println!("arr is {:?}", arr);
    println!("res {}", Solution::is_split_with_k_group(arr.to_vec(), 4))
}

struct Solution;

impl Solution {
    pub fn minimum_deviation(nums: Vec<i32>) -> i32 {
        let mut ts = BTreeSet::new();
        nums.iter()
            .map(|&v| if v & 1 == 1 { v * 2 } else { v })
            .for_each(|v| {
                ts.insert(v);
            });
        let mut res = i32::MAX;
        while let Some(&n) = ts.last() {
            res = res.min(n - ts.first().unwrap());
            if n & 1 == 1 {
                break;
            }
            ts.pop_last();
            ts.insert(n / 2);
        }
        res
    }

    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut answers = answers;
        answers.sort_unstable();
        let mut i = 0;
        let mut j = i;
        let mut res = 0;
        while i < answers.len() {
            let cur = answers[i];
            while j < answers.len() && answers[j] == cur {
                j += 1;
            }
            res += ((j - i) as i32 + cur) / (cur + 1) * (cur + 1);
            i = j;
        }
        res
    }

    pub fn make_similar(nums: Vec<i32>, target: Vec<i32>) -> i64 {
        let mut nums = nums;
        let mut target = target;
        let k = Solution::split(&mut nums);
        Solution::split(&mut target);
        nums[..k].sort_unstable();
        nums[k..].sort_unstable();
        target[..k].sort_unstable();
        target[k..].sort_unstable();
        let mut i = 0;
        let mut res = 0_i64;
        while i < nums.len() {
            res += (target[i] - nums[i]).abs() as i64;
            i += 1;
        }
        res / 4
    }

    // 将数组的奇偶分开，并且返回第一个偶数的索引
    fn split(nums: &mut [i32]) -> usize {
        let mut f = 0;
        let mut l = nums.len() - 1;
        while f <= l {
            if nums[f] & 1 == 1 {
                f += 1;
            } else {
                nums.swap(f, l);
                if l > 0 {
                    l -= 1;
                } else {
                    break;
                }
            }
        }
        f
    }

    fn is_split_with_k_group(nums: Vec<i32>, k: i32) -> bool {
        let mut max_freq = 1;
        let mut freq = 1;
        for (i, &v) in nums.iter().enumerate().skip(1) {
            if v == nums[i - 1] {
                freq += 1;
            } else {
                max_freq = max_freq.max(freq);
                freq = 1;
            }
        }
        // 最后一个词频的比较
        max_freq = max_freq.max(freq);
        nums.len() as i32 / max_freq >= k
    }

    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        if start_fuel >= target {
            0
        } else {
            let mut max_heap = BinaryHeap::new();
            let mut cur = start_fuel;
            let mut res = 0;
            for station in stations {
                let pos = station[0];
                let fuel = station[1];
                if cur >= pos {
                    if cur >= target {
                        return res;
                    }
                } else {
                    while let Some(f) = max_heap.pop() {
                        cur += f;
                        res += 1;
                        if cur >= target {
                            return res;
                        }
                        if cur >= pos {
                            break;
                        }
                    }
                    if cur < pos {
                        return -1;
                    }
                }
                max_heap.push(fuel);
            }
            while let Some(f) = max_heap.pop() {
                cur += f;
                res += 1;
                if cur >= target {
                    return res;
                }
            }
            -1
        }
    }
}
