use std::{
    collections::{BinaryHeap, HashMap},
    mem,
};

fn main() {
    println!(
        "res: {}",
        Solution::find_target_sum_ways_2(vec![1, 1, 1, 1, 1], 3)
    );
}

struct Solution;

impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        // Self::ftsw_force(&nums, 0, 0, target)
        let mut dp = HashMap::new();
        Self::ftsw_rem(&nums, 0, 0, target, &mut dp)
    }

    fn ftsw_force(nums: &[i32], i: usize, sum: i32, target: i32) -> i32 {
        if i == nums.len() {
            if sum == target {
                1
            } else {
                0
            }
        } else {
            Self::ftsw_force(nums, i + 1, sum + nums[i], target)
                + Self::ftsw_force(nums, i + 1, sum - nums[i], target)
        }
    }

    fn ftsw_rem(
        nums: &[i32],
        i: usize,
        sum: i32,
        target: i32,
        dp: &mut HashMap<usize, HashMap<i32, i32>>,
    ) -> i32 {
        if i == nums.len() {
            if sum == target {
                1
            } else {
                0
            }
        } else if dp.contains_key(&i) && dp.get(&i).unwrap().contains_key(&sum) {
            *dp.get(&i).unwrap().get(&sum).unwrap()
        } else {
            let res = Self::ftsw_rem(nums, i + 1, sum + nums[i], target, dp)
                + Self::ftsw_rem(nums, i + 1, sum - nums[i], target, dp);
            dp.entry(i)
                .and_modify(|v| {
                    v.insert(sum, res);
                })
                .or_insert({
                    let mut m = HashMap::new();
                    m.insert(sum, res);
                    m
                });
            res
        }
    }

    pub fn find_target_sum_ways_1(nums: Vec<i32>, target: i32) -> i32 {
        let t_sum = nums.iter().sum::<i32>();
        if t_sum < target || -t_sum > target {
            0
        } else {
            let new_w = t_sum * 2 + 1;
            let mut dp = vec![0; new_w as usize];
            let mut dp1 = vec![0; new_w as usize];
            dp1[(target + t_sum) as usize] = 1;
            for num in nums {
                for (j, e) in dp.iter_mut().enumerate() {
                    let tj = j as i32;
                    if tj - num >= 0 {
                        *e += dp1[(tj - num) as usize]
                    }
                    if tj + num < new_w {
                        *e += dp1[(tj + num) as usize]
                    }
                }
                mem::swap(&mut dp, &mut dp1);
                dp.fill(0);
            }

            dp1[t_sum as usize]
        }
    }

    pub fn find_target_sum_ways_2(nums: Vec<i32>, target: i32) -> i32 {
        let t_sum = nums.iter().sum::<i32>();
        if t_sum < target || -t_sum > target || ((t_sum & 1) ^ (target & 1) == 1) {
            0
        } else {
            let limits = (target + t_sum) / 2;
            let mut dp = vec![0; limits as usize + 1];
            dp[0] = 1;
            for i in 1..=nums.len() {
                for j in (0..=limits as usize).rev() {
                    dp[j] += if j >= nums[i - 1] as usize {
                        dp[j - nums[i - 1] as usize]
                    } else {
                        0
                    };
                }
            }
            dp[limits as usize]
        }
    }

    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum_all = stones.iter().sum::<i32>();
        let targets = sum_all / 2;
        let mut dp = vec![0; targets as usize + 1];
        for stone in stones {
            for j in (0..=targets).rev() {
                dp[j as usize] = dp[j as usize].max(if j >= stone {
                    dp[(j - stone) as usize] + stone
                } else {
                    0
                })
            }
        }
        sum_all - dp[targets as usize] - dp[targets as usize]
    }

    fn top_k_sub_force(nums: &[i32], k: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut sum = 0;
        Self::sub_sum(nums, 0, &mut sum, &mut res);
        res.sort();
        (&res[..k as usize]).into()
    }

    fn sub_sum(nums: &[i32], i: usize, t_sum: &mut i32, sum: &mut Vec<i32>) {
        if i >= nums.len() {
            sum.push(*t_sum);
        } else {
            Self::sub_sum(nums, i + 1, t_sum, sum);
            *t_sum += nums[i];
            Self::sub_sum(nums, i + 1, t_sum, sum);
            *t_sum -= nums[i];
        }
    }

    fn top_k_sub_01_packet(nums: &[i32], k: i32) -> Vec<i32> {
        let sum = nums.iter().sum::<i32>();
        let mut dp = vec![0; sum as usize + 1];
        dp[0] = 1;
        for &num in nums {
            for j in (0..=sum as usize).rev() {
                dp[j] += if j as i32 >= num {
                    dp[(j as i32 - num) as usize]
                } else {
                    0
                }
            }
        }
        let mut k = k;
        let mut res = vec![];
        'out: for (j, e) in dp.iter_mut().enumerate() {
            while *e > 0 {
                res.push(j as i32);
                k -= 1;
                *e -= 1;
                if k == 0 {
                    break 'out;
                }
            }
        }

        res
    }

    fn top_k_sub_heap(nums: &[i32], k: i32) -> Vec<i32> {
        let mut nums: Vec<i32> = nums.into();
        nums.sort();
        let mut min_heap = BinaryHeap::new();
        let mut res = vec![0];
        let mut k = k - 1;
        min_heap.push(Unit(0, nums[0]));
        while k > 0 {
            let cur = min_heap.pop().unwrap();
            res.push(cur.1);
            min_heap.push(Unit(cur.0 + 1, nums[cur.0 + 1]));
            min_heap.push(Unit(cur.0 + 1, cur.1 + nums[cur.0 + 1]));
            k -= 1;
        }
        res
    }
}

struct Unit(usize, i32);

impl Eq for Unit {}

impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Ord for Unit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test_top_k_sub {
    use rand::{thread_rng, Rng};

    #[test]
    fn test_top_k_sub_sum() {
        let mut rng = thread_rng();
        let times = 100;
        for _ in 0..times {
            let len = rng.gen_range(10..=20);
            let mut nums = Vec::with_capacity(len);
            for _ in 0..len {
                nums.push(rng.gen_range(25..=50));
            }
            let k = rng.gen_range(1..=len as i32);
            // println!("{:?} {k}", nums);
            let r1 = super::Solution::top_k_sub_force(&nums, k);
            let r2 = super::Solution::top_k_sub_01_packet(&nums, k);
            let r3 = super::Solution::top_k_sub_heap(&nums, k);
            assert_eq!(
                r1, r2,
                "01packet -> correct value {:?} verified value {:?}",
                r1, r2
            );
            assert_eq!(
                r1, r3,
                "heap -> correct value {:?} verified value {:?}",
                r1, r3
            );
        }
    }
}
