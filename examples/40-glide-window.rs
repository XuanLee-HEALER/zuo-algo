use std::i32;

fn main() {
    println!("{}", Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]));
    println!(
        "2 {}",
        Solution::length_of_longest_substring("abcabcbb".into())
    );
    println!(
        "3 {}",
        Solution::min_window("cabwefgewcwaefgcf".into(), "cae".into())
    );
    println!("5 {}", Solution::balanced_string("WQWRQQQW".into()));
    println!(
        "6 {}",
        Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2)
    );
    println!("7 {}", Solution::longest_substring("aacbbbdc".into(), 3))
}

struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let mut ans = i32::MAX;
        let mut sum = 0;
        while r < nums.len() {
            sum += nums[r];
            while sum - nums[l] >= target {
                sum -= nums[l];
                l += 1;
            }
            if sum >= target {
                ans = ans.min((r - l + 1) as i32)
            }
            r += 1;
        }
        if ans == i32::MAX {
            0
        } else {
            ans
        }
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let s: Vec<char> = s.chars().collect();
        let mut tb = [-1; 256];
        let mut ans = 0;
        while r < s.len() {
            l = l.max((tb[s[r] as usize] + 1) as usize);
            tb[s[r] as usize] = r as i32;
            ans = ans.max((r - l) as i32 + 1);
            r += 1;
        }
        ans
    }

    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return "".into();
        }
        let mut occur = t.len();
        let sc: Vec<char> = s.chars().collect();
        let mut tb = [0; 256];
        for c in t.chars() {
            tb[c as usize] -= 1;
        }

        let mut shortest = usize::MAX;
        let mut l = 0;
        let mut r = 0;
        let mut start = 0;
        while r < sc.len() {
            let c_idx = sc[r] as usize;
            tb[c_idx] += 1;
            if tb[c_idx] <= 0 {
                occur -= 1;
            }

            if occur == 0 {
                // 注意这里是-1，所以是大于等于0
                while tb[sc[l] as usize] > 0 {
                    tb[sc[l] as usize] -= 1;
                    l += 1;
                }

                // 只有碰到最短才能更新start
                if shortest > r - l + 1 {
                    start = l;
                    shortest = shortest.min(r - l + 1)
                }
            }

            r += 1;
        }

        if shortest == usize::MAX {
            "".into()
        } else {
            s[start..start + shortest].into()
        }
    }

    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = 0;
        let station_num = gas.len();
        while l < station_num {
            let mut sum = 0;
            while sum + gas[r % station_num] - cost[r % station_num] >= 0 {
                if r - l + 1 == station_num {
                    return l as i32;
                }

                sum += gas[r % station_num] - cost[r % station_num];
                r += 1;
            }

            l = r + 1;
            r = l;
        }

        -1
    }

    pub fn balanced_string(s: String) -> i32 {
        let handle_arr: Vec<i32> = s
            .chars()
            .map(|c| match c {
                'W' => 1,
                'E' => 2,
                'R' => 3,
                _ => 0,
            })
            .collect();
        let mut statis = [0; 4];
        handle_arr.iter().for_each(|&i| {
            statis[i as usize] += 1;
        });

        let avg = handle_arr.len() as i32 / 4;
        let mut debt = 0;

        for v in statis.iter_mut() {
            if *v > avg {
                *v = avg - *v;
                debt -= *v;
            } else {
                *v = 0
            }
        }
        println!("{:?} {}", statis, debt);
        if debt == 0 {
            return 0;
        }
        let mut l = 0;
        let mut r = 0;
        let mut ans = i32::MAX;
        while r < handle_arr.len() {
            if statis[handle_arr[r] as usize] < 0 {
                debt -= 1;
            }
            statis[handle_arr[r] as usize] += 1;

            if debt == 0 {
                while statis[handle_arr[l] as usize] > 0 {
                    statis[handle_arr[l] as usize] -= 1;
                    l += 1;
                }
                ans = ans.min((r - l) as i32 + 1);
            }

            r += 1;
        }

        ans
    }

    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let less_k = |nums: &[i32], k: i32| -> i32 {
            let mut statis = [0; 20_001];

            let mut ans = 0;
            let mut cata = 0;
            let mut l = 0;
            let mut r = 0;
            while r < nums.len() {
                if statis[nums[r] as usize] == 0 {
                    cata += 1;
                }
                statis[nums[r] as usize] += 1;

                while cata > k {
                    if statis[nums[l] as usize] - 1 == 0 {
                        cata -= 1;
                    }
                    statis[nums[l] as usize] -= 1;
                    l += 1;
                }

                ans += (r - l) as i32 + 1;

                r += 1;
            }

            ans
        };

        less_k(&nums, k) - less_k(&nums, k - 1)
    }

    pub fn longest_substring(s: String, k: i32) -> i32 {
        let mut statis = [0; 26];
        let s: Vec<char> = s.chars().collect();
        let mut longest_with_n_tp = |s: &[char], t: i32, k: i32| -> i32 {
            let mut ans = 0;
            let mut l = 0;
            let mut r = 0;
            // 种类数
            let mut typo = 0;
            // 满足的种类数
            // 用单个变量替代直接统计每个数字的出现次数！！！
            let mut satisfied = 0;
            while r < s.len() {
                if statis[(s[r] as u8 - b'a') as usize] == 0 {
                    typo += 1;
                }
                statis[(s[r] as u8 - b'a') as usize] += 1;

                // ★只能修改一次
                if statis[(s[r] as u8 - b'a') as usize] == k {
                    satisfied += 1;
                }

                while typo > t {
                    if statis[(s[l] as u8 - b'a') as usize] - 1 == 0 {
                        typo -= 1;
                    }

                    // ★只能修改一次
                    if statis[(s[l] as u8 - b'a') as usize] == k {
                        satisfied -= 1;
                    }

                    statis[(s[l] as u8 - b'a') as usize] -= 1;
                    l += 1;
                }

                if satisfied == t {
                    ans = ans.max((r - l) as i32 + 1);
                }
                r += 1;
            }
            statis.fill(0);
            ans
        };

        let mut ans = 0;
        for i in 1..=26 {
            ans = ans.max(longest_with_n_tp(&s, i, k));
        }

        ans
    }
}
