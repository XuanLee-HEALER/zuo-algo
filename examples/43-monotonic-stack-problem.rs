fn main() {
    println!("{}", Solution::sum_subarray_mins(vec![3, 1, 2, 4]));
    println!(
        "{}",
        Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3])
    );
    println!(
        "{}",
        Solution::maximal_rectangle(vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0']
        ])
    );
    println!(
        "{}",
        Solution::num_submat(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0],])
    )
}

struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut top = 0;
        let mut stack = vec![0; temperatures.len()];
        let mut ans = vec![0; temperatures.len()];

        for (i, &e) in temperatures.iter().enumerate() {
            if top == 0 || e <= temperatures[stack[top - 1] as usize] {
                stack[top] = i as i32;
                top += 1;
            } else {
                while top > 0 && e > temperatures[stack[top - 1] as usize] {
                    ans[stack[top - 1] as usize] = i as i32 - stack[top - 1];
                    top -= 1;
                }
                stack[top] = i as i32;
                top += 1;
            }
        }

        ans
    }

    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        const MOD: i32 = 1_000_000_000 + 7;
        let mut top = 0;
        let mut stack = vec![0; arr.len()];
        let mut res = 0;

        for (i, &e) in arr.iter().enumerate() {
            if top == 0 || e > arr[stack[top - 1]] {
                stack[top] = i;
                top += 1;
            } else {
                while top > 0 && e <= arr[stack[top - 1]] {
                    let cur_idx = stack[top - 1] as i32;
                    let cur = arr[cur_idx as usize];
                    let left: i32 = if top > 1 { stack[top - 2] as i32 } else { -1 };
                    let right: i32 = i as i32;
                    res = (res
                        + ((cur_idx - left) as i64 * (right - cur_idx) as i64 * cur as i64
                            % MOD as i64) as i32)
                        % MOD;
                    top -= 1;
                }

                stack[top] = i;
                top += 1;
            }
        }

        while top > 0 {
            let cur_idx = stack[top - 1] as i32;
            let cur = arr[cur_idx as usize];
            let left: i32 = if top > 1 { stack[top - 2] as i32 } else { -1 };
            let right: i32 = arr.len() as i32;
            res = (res
                + ((cur_idx - left) as i64 * (right - cur_idx) as i64 * cur as i64 % MOD as i64)
                    as i32)
                % MOD;
            top -= 1;
        }

        res
    }

    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut top = 0;
        let mut stack = vec![0; heights.len()];
        let mut ans = 0;

        for (i, &e) in heights.iter().enumerate() {
            if top == 0 || e > heights[stack[top - 1]] {
                stack[top] = i;
                top += 1;
            } else {
                while top > 0 && e <= heights[stack[top - 1]] {
                    let cur_idx = stack[top - 1] as i32;
                    let cur = heights[cur_idx as usize];
                    let left: i32 = if top > 1 { stack[top - 2] as i32 } else { -1 };
                    let right: i32 = i as i32;
                    ans = ans.max((right - left - 1) * cur);
                    top -= 1;
                }

                stack[top] = i;
                top += 1;
            }
        }

        while top > 0 {
            let cur_idx = stack[top - 1] as i32;
            let cur = heights[cur_idx as usize];
            let left: i32 = if top > 1 { stack[top - 2] as i32 } else { -1 };
            let right: i32 = heights.len() as i32;
            ans = ans.max((right - left - 1) * cur);
            top -= 1;
        }

        ans
    }

    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let rec_area = |sides: &[i32]| -> i32 {
            let mut top = 0;
            let mut stack = vec![0; sides.len()];
            let mut res = 0;

            for (i, &side) in sides.iter().enumerate() {
                if top != 0 && side < sides[stack[top - 1]] {
                    while top > 0 && side <= sides[stack[top - 1]] {
                        let cur = sides[stack[top - 1]];
                        let left = if top > 1 { stack[top - 2] as i32 } else { -1 };
                        let right = i as i32;
                        res = res.max((right - left - 1) * cur);
                        top -= 1;
                    }
                }
                stack[top] = i;
                top += 1;
            }

            while top > 0 {
                let cur = sides[stack[top - 1]];
                let left = if top > 1 { stack[top - 2] as i32 } else { -1 };
                let right = sides.len() as i32;
                res = res.max((right - left - 1) * cur);
                top -= 1;
            }

            res
        };

        let mut ans = 0;
        let mut cur_side = vec![0; matrix[0].len()];
        for row in &matrix {
            let mut counter = 0;
            row.iter().for_each(|&c| {
                let v = (c as u8 - b'0') as i32;
                if v == 0 {
                    cur_side[counter] = 0;
                } else {
                    cur_side[counter] += 1;
                }
                counter += 1;
            });
            ans = ans.max(rec_area(&cur_side));
        }

        ans
    }

    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut top = 0;
        let mut stack = vec![0; 50_001];

        for (idx, &num) in nums.iter().enumerate() {
            if top == 0 || num < nums[stack[top - 1]] {
                stack[top] = idx;
                top += 1;
            }
        }

        let mut ans = 0;
        for idx in (0..nums.len()).rev() {
            if top == 0 {
                break;
            } else {
                while top > 0 && nums[idx] >= nums[stack[top - 1]] {
                    ans = ans.max(idx - stack[top - 1]);
                    top -= 1;
                }
            }
        }

        ans as i32
    }

    pub fn remove_duplicate_letters(s: String) -> String {
        let mut top = 0;
        let mut stack = ['\0'; 26];
        let mut cnts = [0; 26];
        let mut enter = [false; 26];
        let cal_idx = |c: char| -> usize { (c as u8 - b'a') as usize };

        for c in s.chars() {
            cnts[cal_idx(c)] += 1;
        }

        for c in s.chars() {
            if !enter[cal_idx(c)] {
                while top > 0 && c < stack[top - 1] && cnts[cal_idx(stack[top - 1])] > 0 {
                    enter[cal_idx(stack[top - 1])] = false;
                    top -= 1;
                }
                enter[cal_idx(c)] = true;
                stack[top] = c;
                top += 1;
            }
            cnts[cal_idx(c)] -= 1;
        }

        stack[0..top].iter().collect::<String>()
    }

    pub fn total_steps(nums: Vec<i32>) -> i32 {
        let mut top = 0;
        let mut stack = vec![(0, 0); nums.len()];
        let mut ans = 0;

        for &num in nums.iter().rev() {
            let mut cur_turn = 0;
            while top > 0 && num > stack[top - 1].0 {
                cur_turn += 1;
                cur_turn = cur_turn.max(stack[top - 1].1);
                top -= 1;
                ans = ans.max(cur_turn)
            }
            stack[top] = (num, cur_turn);
            top += 1;
        }

        ans
    }

    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let mut top = 0;
        let mut stack = vec![0; mat[0].len()];
        let mut num_mat = |heights: &[i32]| -> i32 {
            top = 0;
            let mut res = 0;

            for (idx, &height) in heights.iter().enumerate() {
                while top > 0 && height <= heights[stack[top - 1]] {
                    let left_idx: i32 = if top < 2 { -1 } else { stack[top - 2] as i32 };
                    let right_idx = idx as i32;
                    let lowest = (if left_idx == -1 {
                        0
                    } else {
                        heights[left_idx as usize]
                    })
                    .max(heights[right_idx as usize]);
                    let cur = heights[stack[top - 1]];
                    res +=
                        (cur - lowest) * ((right_idx - left_idx - 1) * (right_idx - left_idx)) / 2;
                    top -= 1;
                }

                stack[top] = idx;
                top += 1;
            }

            while top > 0 {
                let left_idx: i32 = if top < 2 { -1 } else { stack[top - 2] as i32 };
                let right_idx = heights.len() as i32;
                let lowest = if left_idx == -1 {
                    0
                } else {
                    heights[left_idx as usize]
                };
                let cur = heights[stack[top - 1]];
                res += (cur - lowest) * ((right_idx - left_idx - 1) * (right_idx - left_idx)) / 2;
                top -= 1;
            }

            res
        };

        let mut ans = 0;
        let mut cur_row = vec![0; mat[0].len()];
        for row in &mat {
            let mut counter = 0;
            row.iter().for_each(|&i| {
                if i == 1 {
                    cur_row[counter] += i
                } else {
                    cur_row[counter] = i
                };
                counter += 1;
            });
            ans += num_mat(&cur_row);
        }

        ans
    }
}
