fn main() {
    println!(
        "{}",
        Solution::min_eating_speed(
            vec![
                873375536, 395271806, 617254718, 970525912, 634754347, 824202576, 694181619,
                20191396, 886462834, 442389139, 572655464, 438946009, 791566709, 776244944,
                694340852, 419438893, 784015530, 588954527, 282060288, 269101141, 499386849,
                846936808, 92389214, 385055341, 56742915, 803341674, 837907634, 728867715,
                20958651, 167651719, 345626668, 701905050, 932332403, 572486583, 603363649,
                967330688, 484233747, 859566856, 446838995, 375409782, 220949961, 72860128,
                998899684, 615754807, 383344277, 36322529, 154308670, 335291837, 927055440,
                28020467, 558059248, 999492426, 991026255, 30205761, 884639109, 61689648,
                742973721, 395173120, 38459914, 705636911, 30019578, 968014413, 126489328,
                738983100, 793184186, 871576545, 768870427, 955396670, 328003949, 786890382,
                450361695, 994581348, 158169007, 309034664, 388541713, 142633427, 390169457,
                161995664, 906356894, 379954831, 448138536
            ],
            943223529
        )
    );
    println!("{}", Solution::split_array(vec![7, 2, 5, 10, 8], 2));
    println!(
        "{}",
        Solution::smallest_distance_pair(vec![9, 10, 7, 10, 6, 1, 5, 4, 9, 8], 18)
    );
    println!("{}", Solution::max_run_time(2, vec![3, 3, 3]));
}

struct Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut l = 0;
        let mut r = *piles.iter().max().unwrap();
        let elapse = |piles: &[i32], k: i32| -> i64 {
            let mut times = 0;
            // 边界条件，注意数值越界
            for &pile in piles {
                if pile == 0 {
                    times += 1_i64;
                } else if k == 0 {
                    return i64::MAX;
                } else {
                    times += (pile as i64 + k as i64 - 1) / k as i64;
                }
            }
            times
        };
        let mut ans = 0;
        while l <= r {
            let m = l + ((r - l) >> 1);
            if elapse(&piles, m) <= h as i64 {
                ans = m;
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        ans
    }

    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut l = 0;
        let mut r = nums.iter().sum();
        let mut ans = 0;
        let painters = |nums: &[i32], limit: i32| -> i32 {
            let mut sum = 0;
            let mut p = 0;
            for &num in nums {
                if num > limit {
                    return i32::MAX;
                } else if sum + num > limit {
                    p += 1;
                    sum = num;
                } else {
                    sum += num;
                }
            }
            p + 1
        };
        while l <= r {
            let m = l + ((r - l) >> 1);
            if painters(&nums, m) <= k {
                ans = m;
                r = m - 1;
            } else {
                l = m + 1;
            }
        }
        ans
    }

    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut l = 0;
        let mut r = nums.last().unwrap() - nums.first().unwrap();
        let mut ans = 0;
        let pairs = |nums: &[i32], limit: i32| -> i32 {
            let mut l = 0;
            let mut r = 1;
            let mut ans = 0;
            while l < nums.len() {
                // 特殊情况，r没动，l动了，r要能动
                if l == r {
                    r += 1;
                }
                while r < nums.len() && nums[r] - nums[l] <= limit {
                    r += 1;
                }

                ans += (r - 1 - l) as i32;
                l += 1;
            }
            // let mut l = 0;
            // let mut r = 0;
            // let mut ans = 0;
            // while l < nums.len() {
            //     ！！！这里用r+1，那么总能走到 r+1==l的时候，把r加回来
            //     while r + 1 < nums.len() && nums[r + 1] - nums[l] <= limit {
            //         r += 1;
            //     }
            //     ans += (r - l) as i32;
            //     l += 1;
            // }
            ans
        };
        while l <= r {
            let m = l + ((r - l) >> 1);
            if pairs(&nums, m) >= k {
                // 如果大于等于k组，更短的数对距离，更少的对数
                ans = m;
                r = m - 1;
            } else {
                l = m + 1;
            }
        }

        ans
    }

    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let mut l = 0;
        let mut r: i64 = batteries.iter().map(|&i| i as i64).sum();
        let mut ans = 0;
        let meet = |bat: &[i32], limit: i64| -> bool {
            let mut n = n as i64;
            let mut sum = 0;
            for &b in bat {
                if b as i64 >= limit {
                    n -= 1;
                } else {
                    sum += b as i64;
                }

                if sum >= limit * n {
                    return true;
                }
            }

            false
        };
        while l <= r {
            let m = l + ((r - l) >> 1);
            if meet(&batteries, m) {
                ans = m;
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        ans
    }

    pub fn max_run_time_1(n: i32, batteries: Vec<i32>) -> i64 {
        let mut l = 0;
        let mut sum = 0;
        let mut max: i64 = 0;
        batteries.iter().for_each(|&i| {
            sum += i as i64;
            if i as i64 > max {
                max = i as i64
            }
        });

        if sum > max * n as i64 {
            return sum / n as i64;
        }

        let mut r = max as i32;

        let mut ans = 0;
        let meet = |bat: &[i32], limit: i32| -> bool {
            let mut n = n;
            let mut sum: i64 = 0;
            for &b in bat {
                if b >= limit {
                    n -= 1;
                } else {
                    sum += b as i64;
                }

                // n和limit都需要转为i64计算
                if sum >= n as i64 * limit as i64 {
                    return true;
                }
            }
            false
        };
        while l <= r {
            let m = l + ((r - l) >> 1);
            if meet(&batteries, m) {
                ans = m;
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        ans as i64
    }
}
