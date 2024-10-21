fn main() {
    println!("trap: {}", Solution::trap2(vec![2, 0, 2]));
    println!(
        "find radius: {}",
        Solution::find_radius(vec![1, 5], vec![2])
    );
    println!(
        "missing: {}",
        Solution::first_missing_positive(vec![3, 4, -1, 1])
    );
}

struct Solution;

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut even = 0;
        let mut look = nums.len() - 1;
        while even < look {
            if (nums[look] & 1) == 1 {
                look -= 1;
            } else {
                nums.swap(look, even);
                even += 1;
            }
        }
        nums
    }

    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut even = 0;
        let mut odd = 1;
        let l = nums.len();
        while even < l && odd < l {
            if (nums[l - 1] & 1) == 1 {
                nums.swap(odd, l - 1);
                odd += 2;
            } else {
                nums.swap(even, l - 1);
                even += 2;
            }
        }
        nums
    }

    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut fast = 0;

        loop {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
            if slow == fast {
                break;
            }
        }

        fast = 0;
        loop {
            fast = nums[fast] as usize;
            slow = nums[slow] as usize;
            if fast == slow {
                break;
            }
        }

        fast as i32
    }

    pub fn trap1(height: Vec<i32>) -> i32 {
        let mut lr_max = vec![0; height.len()];
        let mut rl_max = vec![0; height.len()];
        for (i, &v) in height.iter().enumerate() {
            if i == 0 {
                lr_max[i] = v;
            } else {
                lr_max[i] = v.max(lr_max[i - 1]);
            }
        }

        for (i, &v) in height.iter().rev().enumerate() {
            if height.len() - i - 1 == height.len() - 1 {
                rl_max[height.len() - i - 1] = v;
            } else {
                rl_max[height.len() - i - 1] = v.max(rl_max[height.len() - i])
            }
        }

        let mut ans = 0;
        for i in 1..height.len() - 1 {
            ans += 0.max(lr_max[i - 1].min(rl_max[i + 1]) - height[i]);
        }

        ans
    }

    /// 因为usize不能是负数，注意越界，所以需要判断边界条件
    /// 对于双指针同时移动，如果正在处理的是同一个位置，那么要判断不能同时移动，因为会加两次
    pub fn trap2(height: Vec<i32>) -> i32 {
        if height.len() < 3 {
            return 0;
        }
        let mut l = 1;
        let mut r = height.len() - 2;
        let mut lmax = *height.first().unwrap();
        let mut rmax = *height.last().unwrap();
        let mut ans = 0;
        while l <= r {
            match lmax.cmp(&rmax) {
                std::cmp::Ordering::Greater => {
                    ans += 0.max(rmax - height[r]);
                    rmax = rmax.max(height[r]);
                    r -= 1;
                }
                std::cmp::Ordering::Less => {
                    ans += 0.max(lmax - height[l]);
                    lmax = lmax.max(height[l]);
                    l += 1;
                }
                std::cmp::Ordering::Equal => {
                    if l < r {
                        ans += 0.max(lmax - height[l]);
                        lmax = lmax.max(height[l]);
                        l += 1;
                    }
                    ans += 0.max(rmax - height[r]);
                    rmax = rmax.max(height[r]);
                    r -= 1;
                }
            }
        }
        ans
    }

    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort_unstable();
        let mut l = 0;
        let mut r = people.len() - 1;
        let mut ans = 0;
        while l <= r {
            let sum = if l == r {
                people[l]
            } else {
                people[l] + people[r]
            };

            if r == 0 {
                return ans + 1;
            }

            if sum > limit {
                r -= 1;
            } else {
                l += 1;
                r -= 1;
            }
            ans += 1;
        }

        ans
    }

    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut ans = 0;
        while l < r {
            ans = ans.max(height[l].min(height[r]) * (r - l) as i32);
            if height[l] <= height[r] {
                l += 1;
            } else {
                if r == 0 {
                    break;
                }
                r -= 1;
            }
        }
        ans
    }

    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut houses = houses;
        let mut heaters = heaters;
        houses.sort_unstable();
        heaters.sort_unstable();
        let mut l = 0;
        let mut r = 0;
        let best_heater = |i: usize, j: usize| -> bool {
            if j == heaters.len() - 1 {
                true
            } else {
                (heaters[j] - houses[i]).abs() < (heaters[j + 1] - houses[i]).abs()
            }
        };
        let mut ans = 0;
        while l < houses.len() {
            while !best_heater(l, r) {
                r += 1;
            }
            ans = ans.max((heaters[r] - houses[l]).abs());
            l += 1;
        }

        ans
    }

    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut l = 0;
        let mut r = nums.len();
        while l < r {
            if nums[l] == l as i32 + 1 {
                l += 1;
            } else if nums[l] > r as i32
                || nums[l] <= l as i32
                || nums[l] == nums[nums[l] as usize - 1]
            {
                r -= 1;
                nums.swap(l, r);
            } else {
                let swap_idx = nums[l] as usize - 1;
                nums.swap(l, swap_idx);
            }
        }
        l as i32 + 1
    }
}
