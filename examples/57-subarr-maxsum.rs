use std::i32;

fn main() {
    println!(
        "res: {}",
        Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
    );
    println!(
        "extra res: {:?}",
        Solution::max_sub_array_extra(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
    );
    println!(
        "extra res: {:?}",
        Solution::max_sub_array_extra(vec![5, 4, -1, 7, 8])
    );
    println!(
        "circular res: {}",
        Solution::max_subarray_sum_circular(vec![1, -2, 3, -2])
    );
    println!(
        "max submatrix: {:?}",
        Solution::get_max_matrix(vec![
            vec![9, -8, 1, 3, -2],
            vec![-3, 7, 6, -2, 4],
            vec![6, -4, -4, 8, -7]
        ])
    );
    println!("max product: {}", Solution::max_product(vec![2, 3, -2, 4]));
    println!(
        "k-split: {:?}",
        Solution::max_sum_of_three_subarrays(vec![1, 2, 1, 2, 6, 7, 5, 1], 2)
    )
}

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut pre = nums[0];
        let mut ans = pre;
        for &num in &nums[1..] {
            pre = num.max(num + pre);
            ans = ans.max(pre)
        }
        ans
    }

    fn max_sub_array_extra(nums: Vec<i32>) -> (usize, usize, i32) {
        let mut r = (0, 0, i32::MIN);
        let (mut tl, mut tr, mut sum) = (0, 0, i32::MIN);
        while tr < nums.len() {
            if sum < 0 {
                tl = tr;
                sum = nums[tr];
            } else {
                sum += nums[tr];
            }
            if sum > r.2 {
                r.0 = tl;
                r.1 = tr;
                r.2 = sum;
            }
            tr += 1;
        }
        r
    }

    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            0 => 0,
            1 => nums[0],
            2 => nums[0].max(nums[1]),
            _ => {
                let mut prepre = nums[0];
                let mut pre = prepre.max(nums[1]);
                for &num in &nums[2..] {
                    let cur = num.max(num + prepre).max(pre);
                    prepre = pre;
                    pre = cur;
                }
                pre
            }
        }
    }

    // 题目要求数组内元素全部大于等于0，所以不存在只选当前值不选的情况
    fn rob_1(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, 0), |acc, e| (acc.1, acc.1.max(e + acc.0)))
            .1
    }

    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut all = nums[0];
        let mut max = nums[0];
        let mut min = nums[0];
        let mut max_pre = nums[0];
        let mut min_pre = nums[0];
        for &num in &nums[1..] {
            all += num;
            max_pre = num.max(num + max_pre);
            min_pre = num.min(num + min_pre);
            max = max.max(max_pre);
            min = min.min(min_pre);
        }
        if all == min {
            max
        } else {
            max.max(all - min)
        }
    }

    pub fn rob_2(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            nums[0]
        } else if n == 2 {
            nums[0].max(nums[1])
        } else {
            Self::rob_2_1(&nums[1..]).max(nums[0] + Self::rob_2_1(&nums[2..n - 1]))
        }
    }

    fn rob_2_1(nums: &[i32]) -> i32 {
        let n = nums.len();
        if n == 0 {
            0
        } else if n == 1 {
            nums[0]
        } else if n == 2 {
            nums[0].max(nums[1])
        } else {
            let mut prepre = nums[0];
            let mut pre = nums[0].max(nums[1]);
            for &num in &nums[2..] {
                let cur = pre.max(num + prepre).max(num);
                prepre = pre;
                pre = cur;
            }
            pre
        }
    }

    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        nums.iter().for_each(|&num| {
            min = min.min(num);
            max = max.max(num);
        });
        let mut res = 0;
        while min <= max {
            let mid = min + ((max - min) >> 1);
            if Self::mc(&nums, mid) >= k {
                res = mid;
                if mid > min {
                    max = mid;
                } else {
                    break;
                }
            } else {
                min = mid + 1;
            }
        }
        res
    }

    fn mc(nums: &[i32], cap: i32) -> i32 {
        match nums.len() {
            1 => {
                if nums[0] <= cap {
                    1
                } else {
                    0
                }
            }
            2 => {
                if nums[0] <= cap || nums[1] <= cap {
                    1
                } else {
                    0
                }
            }
            _ => {
                let mut prepre = if nums[0] <= cap { 1 } else { 0 };
                let mut pre = if nums[0] <= cap || nums[1] <= cap {
                    1
                } else {
                    0
                };
                for &num in &nums[2..] {
                    let cur = pre.max(if num <= cap { 1 + prepre } else { 0 });
                    prepre = pre;
                    pre = cur
                }
                pre
            }
        }
    }

    fn mc_1(nums: &[i32], cap: i32) -> i32 {
        let mut res = 0;
        let mut skip = false;
        for &num in nums {
            if !skip {
                if num <= cap {
                    res += 1;
                    skip = true;
                }
            } else {
                skip = false
            }
        }
        res
    }

    pub fn get_max_matrix(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut aid = vec![0; m];
        let sum_to_aid = |aid: &mut [i32], nums: &[i32]| {
            aid.iter_mut().enumerate().for_each(|(i, e)| *e += nums[i]);
        };
        let mut max = i32::MIN;
        let mut res = vec![0, 0, 0, 0];
        for i in 0..n {
            for (ti, sub) in matrix.iter().skip(i).enumerate() {
                sum_to_aid(&mut aid, sub);
                let mut pre = i32::MIN;
                let mut cur_max = pre;
                let mut l = 0;
                let mut r = 0;
                let mut max_l = 0;
                let mut max_r = 0;
                while r < m {
                    if pre <= 0 {
                        pre = aid[r];
                        l = r;
                    } else {
                        pre += aid[r];
                    }
                    if pre > cur_max {
                        cur_max = pre;
                        max_l = l;
                        max_r = r;
                    }
                    r += 1;
                }
                if cur_max > max {
                    max = cur_max;
                    res[0] = i as i32;
                    res[1] = max_l as i32;
                    res[2] = (i + ti) as i32;
                    res[3] = max_r as i32;
                }
            }
            aid.fill(0);
        }
        res
    }

    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut res = nums[0];
        let mut pre_max = res;
        let mut pre_min = res;
        for &i in nums.iter().skip(1) {
            let cur_max = i.max(i * pre_max).max(i * pre_min);
            let cur_min = i.min(i * pre_min).min(i * pre_max);
            pre_max = cur_max;
            pre_min = cur_min;
            if cur_max > res {
                res = cur_max;
            }
        }
        res
    }

    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let mut ksum = vec![0; n];
        let (mut l, mut r) = (0, 0);
        let mut sum = 0;
        while r < n {
            sum += nums[r];
            if r - l + 1 == k {
                ksum[l] = sum;
                sum -= nums[l];
                l += 1;
            }
            r += 1;
        }

        // 0..=i范围上长度为k且累加和最大的子数组的起始位置
        let mut dp1 = vec![0; n];
        let (mut l, mut r) = (1, k);
        while r < n {
            // 这里是小于，因为是往后找，所以尽量不动
            if ksum[l] > ksum[dp1[r - 1]] {
                dp1[r] = l
            } else {
                dp1[r] = dp1[r - 1]
            }
            l += 1;
            r += 1
        }

        // 从i..n-1范围上长度为k且累加和最大的子数组的起始位置
        let mut dp2 = vec![n - k; n];
        for r in (0..n - k).rev() {
            // 这里是大于等于，因为是往前找，所以能动就动
            if ksum[r] >= ksum[dp2[r + 1]] {
                dp2[r] = r
            } else {
                dp2[r] = dp2[r + 1]
            }
        }
        let mut res = vec![0; 3];
        let mut max = i32::MIN;
        for m in k..=n - 2 * k {
            let cur_max = ksum[m] + ksum[dp1[m - 1]] + ksum[dp2[m + k]];
            // println!("{} {} {} {}", dp1[m - 1], m, dp2[m + k], cur_max);
            if cur_max > max {
                max = cur_max;
                res[0] = dp1[m - 1] as i32;
                res[1] = m as i32;
                res[2] = dp2[m + k] as i32;
            }
        }

        res
    }
}

fn sub_seq_mod_by_7(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut dp = vec![vec![-1; 7]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        for j in 0..7 {
            dp[i][j] = dp[i - 1][j];
            if dp[i - 1][(7 + j - (nums[i - 1] as usize % 7)) % 7] != -1 {
                dp[i][j] =
                    dp[i][j].max(dp[i - 1][(7 + j - (nums[i - 1] as usize % 7)) % 7] + nums[i - 1])
            }
        }
    }
    dp[n][0]
}

fn sub_seq_mod_by_7_force(nums: &[i32]) -> i32 {
    let mut sum = 0;
    let mut max = -1;
    sub_seq_max(nums, 0, &mut sum, &mut max)
}

fn sub_seq_max(nums: &[i32], i: usize, sum: &mut i32, max: &mut i32) -> i32 {
    if i == nums.len() {
        if *sum % 7 == 0 && *sum > *max {
            *max = *sum
        }
        *max
    } else {
        let max1 = sub_seq_max(nums, i + 1, sum, max);
        *sum += nums[i];
        let max2 = sub_seq_max(nums, i + 1, sum, max);
        *sum -= nums[i];
        max1.max(max2)
    }
}

fn magic_roll(nums: &[i32]) -> i32 {
    let n = nums.len();
    let p1 = nums.iter().sum::<i32>();
    let mut dp1 = vec![0; n];
    let mut sum = nums[0];
    let mut max_sum = sum.max(0);
    for i in 1..n {
        dp1[i] = (dp1[i - 1] + nums[i]).max(max_sum);
        sum += nums[i];
        max_sum = max_sum.max(sum);
    }
    let p2 = dp1[n - 1];
    let mut dp2 = vec![0; n];
    let mut sum = nums[n - 1];
    max_sum = sum.max(0);
    for i in (0..n - 1).rev() {
        dp2[i] = (dp2[i + 1] + nums[i]).max(max_sum);
        sum += nums[i];
        max_sum = max_sum.max(sum);
    }
    let mut p3 = 0;
    for i in 0..n - 1 {
        p3 = p3.max(dp1[i] + dp2[i + 1]);
    }
    p1.max(p2).max(p3)
}

fn magic_roll_force(nums: &[i32]) -> i32 {
    let mut res = i32::MIN;
    for i in 0..nums.len() {
        res = res.max(magic_roll_sub(nums, 0, i) + magic_roll_sub(nums, i + 1, nums.len() - 1))
    }
    res
}

fn magic_roll_sub(nums: &[i32], i: usize, j: usize) -> i32 {
    let mut max = nums[i..=j].iter().sum::<i32>();
    for ti in i..=j {
        for tj in ti..=j {
            let mut t_sum = 0;
            for k in i..=j {
                if k < ti || k > tj {
                    t_sum += nums[k];
                }
            }
            max = max.max(t_sum)
        }
    }

    max
}

fn reverse_once(nums: &[i32]) -> i32 {
    let n = nums.len();
    // 以i为开头，最大的子数组累加和
    let mut dp1 = vec![0; n];
    dp1[n - 1] = nums[n - 1];
    let mut ori_max = dp1[n - 1];
    for i in (0..n - 1).rev() {
        dp1[i] = nums[i].max(nums[i] + dp1[i + 1]);
        ori_max = ori_max.max(dp1[i]);
    }
    // 以i为结尾，最大的子数组累加和
    let mut dp2 = vec![0; n];
    dp2[0] = nums[0];
    for i in 1..n {
        dp2[i] = nums[i].max(nums[i] + dp2[i - 1])
    }
    let mut pre_max = dp2[0];
    for i in 1..n {
        let cur_max = dp1[i] + pre_max;
        if cur_max > ori_max {
            ori_max = cur_max;
        }
        pre_max = pre_max.max(dp2[i]);
    }
    ori_max
}

fn reverse_once_force(nums: &[i32]) -> i32 {
    let mut o_nums = vec![];
    nums.clone_into(&mut o_nums);
    let mut max = i32::MIN;
    for i in 0..nums.len() {
        for j in i..nums.len() {
            o_nums[i..=j].reverse();
            let mut pre = o_nums[0];
            let mut t_max = pre;
            for &num in o_nums.iter().skip(1) {
                pre = num.max(num + pre);
                t_max = t_max.max(pre)
            }
            o_nums[i..=j].reverse();
            max = max.max(t_max)
        }
    }
    max
}

fn max_sum_delete_one(nums: Vec<i32>, k: i32) -> i32 {
    if nums.len() <= k as usize {
        0
    } else {
        let (mut h, mut t) = (0, 0);
        let mut q = Vec::with_capacity(k as usize);
        let (mut l, mut r) = (0, 0);
        let mut max = i32::MIN;
        let mut sum = 0;
        while r < nums.len() {
            while t > h && nums[r] < nums[q[t - 1]] {
                t -= 1;
            }
            q[t] = r;
            sum += nums[r];
            if r - l + 1 == (k + 1) as usize {
                max = max.max(sum - nums[q[h]]);
                if t >= h && nums[l] == nums[q[h]] {
                    h += 1;
                }
                sum -= nums[l];
                l += 1;
            }

            r += 1;
        }

        max
    }
}

fn max_sum_delete_one_force(nums: Vec<i32>, k: i32) -> i32 {
    if nums.len() <= k as usize {
        return 0;
    }
    let mut max = i32::MIN;
    for i in 0..nums.len() {
        let mut cache = Vec::new();
        cache.extend_from_slice(&nums[0..i]);
        cache.extend_from_slice(&nums[i + 1..nums.len()]);
        let (mut l, mut r) = (0, 0);
        let mut sum = 0;
        let mut t_max = sum;
        while r < cache.len() {
            sum += cache[r];
            if r - l + 1 == k as usize {
                if sum > t_max {
                    t_max = sum;
                }
                sum -= cache[l];
                l += 1;
            }
            r += 1;
        }
        max = max.max(t_max);
        cache.clear();
    }
    max
}

#[cfg(test)]
mod maxsum_test {
    use rand::{thread_rng, Rng};

    #[test]
    fn test_mod_7() {
        let times = 2_000;
        let mut rng = thread_rng();
        for _ in 0..times {
            let arr_len = rng.gen_range(0..=20);
            let mut arr = Vec::with_capacity(arr_len);
            for _ in 0..arr_len {
                arr.push(rng.gen_range(0..=100_000));
            }
            let r1 = super::sub_seq_mod_by_7_force(&arr);
            let r2 = super::sub_seq_mod_by_7(&arr);
            assert_eq!(r1, r2, "correct value: {} verify value: {}", r1, r2)
        }
    }

    #[test]
    fn test_magic_roll() {
        let times = 2_000;
        let mut rng = thread_rng();
        for _ in 0..times {
            let arr_len = rng.gen_range(1..=20);
            let mut arr = Vec::with_capacity(arr_len);
            for _ in 0..arr_len {
                arr.push(rng.gen_range(-100_000..=100_000));
            }
            // println!("arr: {:?}", arr);
            let r1 = super::magic_roll_force(&arr);
            let r2 = super::magic_roll(&arr);
            assert_eq!(r1, r2, "correct value: {} verify value: {}", r1, r2)
        }
    }

    #[test]
    fn test_reverse_once() {
        let times = 20_000;
        let mut rng = thread_rng();
        for _ in 0..times {
            let arr_len = rng.gen_range(1..=30);
            let mut arr = Vec::with_capacity(arr_len);
            for _ in 0..arr_len {
                arr.push(rng.gen_range(-100_000..=100_000));
            }
            // arr = vec![-62, 9];
            let r1 = super::reverse_once_force(&arr);
            let r2 = super::reverse_once(&arr);
            assert_eq!(
                r1, r2,
                "correct value: {} verify value: {}\narr {:?}",
                r1, r2, arr
            )
        }
    }

    #[test]
    fn test_max_sum_delete_one() {
        let times = 2_000;
        let mut rng = thread_rng();
        for _ in 0..times {
            let arr_len = rng.gen_range(1..=50);
            let mut arr = Vec::with_capacity(arr_len);
            for _ in 0..arr_len {
                arr.push(rng.gen_range(-100_000..=100_000));
            }
            // println!("arr is {:?}", arr);
            let k = rng.gen_range(1..=20) as i32;
            // arr = vec![-62, 9];
            let r1 = super::max_sum_delete_one_force(arr.clone(), k);
            let r2 = super::max_sum_delete_one_force(arr.clone(), k);
            assert_eq!(
                r1, r2,
                "correct value: {} verify value: {}\narr {:?}",
                r1, r2, arr
            )
        }
    }
}
