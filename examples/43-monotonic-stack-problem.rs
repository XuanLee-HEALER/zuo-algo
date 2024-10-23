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
}
