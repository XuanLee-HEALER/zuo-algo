fn main() {
    println!(
        "res {:?}",
        Solution::jump(vec![7, 0, 9, 6, 9, 6, 1, 7, 9, 0, 1, 2, 9, 0, 3])
    );
    println!("res {:?}", Solution::jump(vec![2, 3, 1, 1, 4]));
    println!("res {:?}", Solution::min_taps(3, vec![0, 0, 0, 0]));
    println!("res {:?}", Solution::min_taps(5, vec![3, 4, 1, 1, 0, 0]));
    assert_eq!(
        Solution::str_transform_to_other_str("abc".into(), "abc".into()),
        true
    );
    assert_eq!(
        Solution::str_transform_to_other_str(
            "abcdefhgijklmnopqrstuvwxyz".into(),
            "abcdefghijklmnopqrstuvwxyz".into()
        ),
        false
    );
    assert_eq!(
        Solution::str_transform_to_other_str("aabcdds".into(), "iilvssa".into()),
        true
    );
}

struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        // 最远能到达的位置
        let mut nxt = 0;
        let mut cur = 0;
        // 跳了多少次
        let mut res = 0;
        for (i, v) in nums.into_iter().enumerate() {
            let i = i as i32;
            // 如果当前位置大于等于目前的位置，说明这个位置能到，更新一下最远的位置
            if cur < i as i32 {
                // 当前位置到不了，必须要跳一次，此时需要更新当前位置到最远的位置
                cur = nxt;
                res += 1
            }
            // 保证当前位置能到，再更新最远
            nxt = nxt.max(i + v);
        }

        res
    }

    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        // 每个点开一个水龙头能到最右的位置
        let mut right = vec![0; n as usize + 1];
        for (i, &v) in ranges.iter().enumerate() {
            let l = ((i as i32 - v).max(0)) as usize;
            let r = i as i32 + v;
            right[l] = right[l].max(r);
        }
        let mut nxt = 0;
        let mut cur = 0;
        let mut res = 0;
        for (i, _) in ranges.into_iter().enumerate() {
            if cur < i as i32 {
                res += 1;
                cur = nxt;
                if cur < i as i32 {
                    return -1;
                }
            }
            // 能走的最右位置
            nxt = nxt.max(right[i]);
        }
        res
    }

    fn str_transform_to_other_str(str1: String, str2: String) -> bool {
        if str1 == str2 {
            true
        } else {
            let mut counts = vec![0_i32; 26];
            let str1 = str1.as_bytes();
            let str2 = str2.as_bytes();
            let mut freq = 0;
            for &c in str2 {
                let idx = (c - b'a') as usize;
                if counts[idx] == 0 {
                    freq += 1;
                }
                counts[idx] += 1;
            }
            if freq == 26 {
                false
            } else {
                counts.fill(-1);
                for (xi, &c) in str1.iter().enumerate() {
                    let idx = (c - b'a') as usize;
                    if counts[idx] != -1 {
                        if str2[counts[idx] as usize] != str2[xi] {
                            return false;
                        }
                    }
                    counts[idx] = xi as i32;
                }
                true
            }
        }
    }

    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        let sum = machines.iter().sum::<i32>();
        let n = machines.len() as i32;
        if sum % n != 0 {
            -1
        } else {
            let avg = sum / n;
            let mut res = 0;
            let mut left_sum = 0;
            for (i, &machine) in machines.iter().enumerate() {
                let i = i as i32;
                let left_need = i * avg;
                let right_need = (n - i - 1) * avg;
                let right_sum = sum - left_sum - machine;
                if left_sum < left_need && right_sum < right_need {
                    res = res.max(left_need - left_sum + right_need - right_sum)
                } else {
                    res = res.max(
                        (left_need - left_sum)
                            .abs()
                            .max((right_need - right_sum).abs()),
                    )
                }
                left_sum += machine;
            }
            res
        }
    }
}
