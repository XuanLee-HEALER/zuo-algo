use std::collections::HashMap;

fn main() {
    println!(
        "res {}",
        Solution::can_partition_k_subsets_1(vec![4, 3, 2, 3, 5, 2, 1], 4)
    );
    println!(
        "res {}",
        Solution::number_ways(vec![
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4]
        ])
    );
    println!(
        "res {}",
        Solution::min_transfers(vec![
            vec![1, 2, 3],
            vec![2, 3, 1],
            vec![1, 3, 5],
            vec![2, 4, 3],
            vec![4, 2, 3],
            vec![3, 5, 6],
            vec![5, 1, 1],
            vec![1, 6, 13],
            vec![6, 3, 3]
        ])
    );
    // println!(
    //     "res {}",
    //     Solution::number_of_good_subsets(vec![4, 2, 3, 15])
    // );
    println!(
        "res {}",
        Solution::number_of_good_subsets_1(vec![4, 2, 3, 15])
    );
    println!(
        "res {}",
        Solution::can_distribute(vec![1, 2, 3, 4], vec![2])
    )
}

struct Solution;

impl Solution {
    pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
        if desired_total <= 0 {
            true
        } else if ((1 + max_choosable_integer) / 2 * max_choosable_integer) < desired_total {
            false
        } else {
            let status = (1 << (max_choosable_integer + 1)) - 1;
            let mut dp = vec![0; status as usize + 1];
            Self::ciw(status, max_choosable_integer, desired_total, &mut dp)
        }
    }

    fn ciw(status: i32, n: i32, remain: i32, dp: &mut [i32]) -> bool {
        if remain <= 0 {
            false
        } else if dp[status as usize] != 0 {
            if dp[status as usize] == 1 {
                true
            } else {
                false
            }
        } else {
            let mut ans = false;
            for c in 1..=n {
                if status & (1 << c) > 0 && !Self::ciw(status ^ (1 << c), n, remain - c, dp) {
                    ans = true;
                    break;
                }
            }
            dp[status as usize] = if ans { 1 } else { -1 };
            ans
        }
    }

    pub fn makesquare(matchsticks: Vec<i32>) -> bool {
        let sum = matchsticks.iter().sum::<i32>();
        if sum % 4 != 0 {
            false
        } else {
            let limit = sum / 4;
            let n = matchsticks.len();
            let status = (1 << n) - 1;
            let mut dp = vec![0; status as usize + 1];
            Self::ms(&matchsticks, status, limit, 0, 4, &mut dp)
        }
    }

    fn ms(
        matchsticks: &[i32],
        status: i32,
        limit: i32,
        cur: i32,
        remain: i32,
        dp: &mut [i32],
    ) -> bool {
        if status == 0 && remain == 0 {
            true
        } else if dp[status as usize] != 0 {
            if dp[status as usize] == 1 {
                true
            } else {
                false
            }
        } else {
            let mut ans = false;
            for (i, &stick) in matchsticks.iter().enumerate() {
                if (1 << i) & status > 0 && cur + stick <= limit {
                    ans = if cur + stick < limit {
                        Self::ms(
                            matchsticks,
                            status ^ (1 << i),
                            limit,
                            cur + stick,
                            remain,
                            dp,
                        )
                    } else {
                        Self::ms(matchsticks, status ^ (1 << i), limit, 0, remain - 1, dp)
                    };
                    if ans {
                        break;
                    }
                }
            }
            dp[status as usize] = if ans { 1 } else { -1 };
            ans
        }
    }

    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % k != 0 {
            false
        } else {
            let limit = sum / k;
            let n = nums.len();
            let status = (1 << n) - 1;
            let mut dp = vec![0; status as usize + 1];
            Self::ms(&nums, status, limit, 0, k, &mut dp)
        }
    }

    pub fn can_partition_k_subsets_1(nums: Vec<i32>, k: i32) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % k != 0 {
            false
        } else {
            let mut nums = nums;
            nums.sort();
            let mut res = vec![0; k as usize];
            Self::cpks(&mut res, &nums, sum / k, nums.len() as i32 - 1)
        }
    }

    fn cpks(res: &mut [i32], nums: &[i32], limit: i32, idx: i32) -> bool {
        if idx < 0 {
            true
        } else {
            let mut i = 0;
            while i < res.len() {
                let cur = nums[idx as usize];
                if res[i] + cur <= limit {
                    res[i] += cur;
                    if Self::cpks(res, nums, limit, idx - 1) {
                        return true;
                    }
                    res[i] -= cur;
                }

                if i + 1 < res.len() && res[i + 1] == res[i] {
                    i += 1;
                }
                i += 1;
            }
            false
        }
    }

    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        let n = hats.len();
        let mut m = 0;
        hats.iter().for_each(|e| {
            e.iter().for_each(|&v| {
                if v > m {
                    m = v
                }
            })
        });
        let m = m as usize;
        let mut hat_people = vec![0; m + 1];
        for (i, sub) in hats.iter().enumerate() {
            for &hat in sub {
                let hat = hat as usize;
                hat_people[hat] |= 1 << i
            }
        }
        let mut dp = vec![vec![-1; m + 1]; 1 << n];
        Self::nw(&hat_people, n, m, 0, 1, &mut dp)
    }

    fn nw(hats: &[i32], n: usize, m: usize, status: i32, idx: usize, dp: &mut [Vec<i32>]) -> i32 {
        if status == (1 << n) - 1 {
            1
        } else if idx == m + 1 {
            0
        } else if dp[status as usize][idx] != -1 {
            dp[status as usize][idx]
        } else {
            let mut ans = Self::nw(hats, n, m, status, idx + 1, dp);
            let mut cur = hats[idx];
            // 只遍历这个帽子适合的那些人
            let mut np = cur & -cur;
            while np > 0 {
                if status & np == 0 {
                    ans = (ans + Self::nw(hats, n, m, status | np, idx + 1, dp)) % Self::MOD;
                }
                cur ^= np;
                np = cur & -cur;
            }
            // for i in 0..n {
            //     if hats[idx] & (1 << i) != 0 && status & (1 << i) == 0 {
            //         ans = (ans + Self::nw(hats, n, m, status | (1 << i), idx + 1, dp)) % Self::MOD;
            //     }
            // }
            dp[status as usize][idx] = ans;
            dp[status as usize][idx]
        }
    }

    fn min_transfers(transactions: Vec<Vec<i32>>) -> i32 {
        let mut mp = HashMap::new();
        for sub in &transactions {
            let m_out = sub[0];
            let m_in = sub[1];
            let m = sub[2];
            mp.entry(m_out).and_modify(|v| *v -= m).or_insert(-m);
            mp.entry(m_in).and_modify(|v| *v += m).or_insert(m);
        }
        let mut debt = vec![];
        mp.iter().filter(|&(_, v)| *v != 0).for_each(|(_, &v)| {
            debt.push(v);
        });
        let n = debt.len();
        let mut dp = vec![-1; 1 << n];
        n as i32 - Self::mt(&debt, n, 0, (1 << n) - 1, &mut dp)
    }

    fn mt(debt: &[i32], n: usize, sum: i32, status: i32, dp: &mut [i32]) -> i32 {
        if dp[status as usize] != -1 {
            dp[status as usize]
        } else if status & (status - 1) == 0 {
            // 只有1个元素无法分组
            0
        } else {
            let mut ans = 0;
            if sum == 0 {
                for i in 0..n {
                    if status & (1 << i) != 0 {
                        ans = Self::mt(debt, n, sum - debt[i], status ^ (1 << i), dp) + 1;
                        break;
                    }
                }
            } else {
                for i in 0..n {
                    if status & (1 << i) != 0 {
                        ans = Self::mt(debt, n, sum - debt[i], status ^ (1 << i), dp);
                    }
                }
            }
            dp[status as usize] = ans;
            dp[status as usize]
        }
    }

    pub fn number_of_good_subsets(nums: Vec<i32>) -> i32 {
        let mut cnts = vec![0; 31];
        let mut max = 0;
        nums.iter().for_each(|&v| {
            cnts[v as usize] += 1;
            if v > max {
                max = v
            }
        });
        let limit = 1 << 10;
        let mut res = 0;
        let mut dp = vec![vec![-1; limit]; max as usize + 1];
        for i in 1..limit {
            res =
                ((res as i64 + Self::nogs(&cnts, max, i as i32, &mut dp)) % Self::MOD as i64) as i32
        }
        res
    }

    fn nogs(cnts: &[i32], i: i32, status: i32, dp: &mut [Vec<i64>]) -> i64 {
        if dp[i as usize][status as usize] != -1 {
            dp[i as usize][status as usize]
        } else {
            let mut res = 0;
            if i == 1 {
                if status == 0 {
                    res = 1;
                    for _ in 0..cnts[i as usize] {
                        res = (res << 1) % Self::MOD as i64
                    }
                }
            } else {
                res = Self::nogs(cnts, i - 1, status, dp);
                if cnts[i as usize] > 0
                    && CK[i as usize] != 0
                    && CK[i as usize] & status == CK[i as usize]
                {
                    res = (res
                        + ((cnts[i as usize] as i64
                            * Self::nogs(cnts, i - 1, status ^ CK[i as usize], dp))
                            % Self::MOD as i64))
                        % Self::MOD as i64
                }
            }
            dp[i as usize][status as usize] = res;
            res
        }
    }

    pub fn number_of_good_subsets_1(nums: Vec<i32>) -> i32 {
        let mut cnts = vec![0; 31];
        let mut max = 0;
        nums.iter().for_each(|&v| {
            cnts[v as usize] += 1;
            if v > max {
                max = v
            }
        });
        let limit = 1 << 10;
        let mut dp = vec![0; limit];
        dp[0] = 1;
        for _ in 0..cnts[1] {
            dp[0] = (dp[0] << 1) % Self::MOD;
        }
        for i in 2..=max {
            if cnts[i as usize] > 0 && CK[i as usize] != 0 {
                for j in (0..limit).rev() {
                    if CK[i as usize] & j as i32 == CK[i as usize] {
                        dp[j] = (dp[j]
                            + ((cnts[i as usize] as i64
                                * dp[(j as i32 ^ CK[i as usize]) as usize] as i64)
                                % Self::MOD as i64) as i32)
                            % Self::MOD
                    }
                }
            }
        }
        let mut res = 0;
        for &v in dp.iter().skip(1) {
            res = (res + v) % Self::MOD
        }
        res
    }

    pub fn can_distribute(nums: Vec<i32>, quantity: Vec<i32>) -> bool {
        let mut nums = nums;
        nums.sort();
        let mut cnts = [0; 50];
        let mut last = nums[0];
        let mut i = 1;
        let mut j = 0;
        cnts[j] += 1;
        while i < nums.len() {
            if nums[i] == last {
                cnts[j] += 1;
            } else {
                last = nums[i];
                j += 1;
                cnts[j] += 1;
            }
            i += 1;
        }
        let n = quantity.len();
        let limit = 1 << n;
        let mut order_nums = vec![0; limit];
        // 订单的顺序也不重要
        for i in 0..n {
            let l = 1 << i;
            let v = quantity[i];
            for j in 0..l {
                order_nums[l | j] = order_nums[j] + v;
            }
        }

        let mut dp = vec![vec![0; limit]; j as usize + 1];
        Self::cd(&cnts, &order_nums, limit as i32 - 1, j as i32, &mut dp)
    }

    fn cd(cnts: &[i32], order_num: &[i32], status: i32, i: i32, dp: &mut [Vec<i32>]) -> bool {
        if status == 0 {
            true
        } else if i < 0 {
            false
        } else if dp[i as usize][status as usize] != 0 {
            if dp[i as usize][status as usize] == 1 {
                true
            } else {
                false
            }
        } else {
            let mut res = false;
            let cur_nums = cnts[i as usize];
            let mut j = status;
            while j > 0 {
                if cur_nums >= order_num[j as usize] {
                    res = Self::cd(cnts, order_num, status ^ j, i - 1, dp);
                    if res {
                        break;
                    }
                }
                j = (j - 1) & status
            }
            if !res {
                res = Self::cd(cnts, order_num, status, i - 1, dp)
            }
            dp[i as usize][status as usize] = if res { 1 } else { -1 };
            res
        }
    }

    const MOD: i32 = 1_000_000_007;
}

// 29 23 19 17 13 11 7 5 3 2
const CK: [i32; 31] = [
    0b0000000000,
    // 1
    0b0000000000,
    // 2
    0b0000000001,
    // 3
    0b0000000010,
    // 4
    0b0000000000,
    // 5
    0b0000000100,
    // 6 2*3
    0b0000000011,
    // 7
    0b0000001000,
    // 8
    0b0000000000,
    // 9
    0b0000000000,
    // 10 2*5
    0b0000000101,
    // 11
    0b0000010000,
    // 12
    0b0000000000,
    // 13
    0b0000100000,
    // 14 2*7
    0b0000001001,
    // 15 3*5
    0b0000000110,
    // 16
    0b0000000000,
    // 17
    0b0001000000,
    // 18
    0b0000000000,
    // 19
    0b0010000000,
    // 20
    0b0000000000,
    // 21 3*7
    0b0000001010,
    // 22 2*11
    0b0000010001,
    // 23
    0b0100000000,
    // 24
    0b0000000000,
    // 25
    0b0000000000,
    // 26 2*13
    0b0000100001,
    // 27
    0b0000000000,
    // 28
    0b0000000000,
    // 29
    0b1000000000,
    // 30 2*3*5
    0b0000000111,
];
