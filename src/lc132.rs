struct Solution;

impl Solution {
    // 首先遍历所有子数组，做一个快速判断回文的数组
    // 定义子问题是 min_cut(i)，在j位置切分
    // 如果 j = n-1，那么就是整个数组是否是回文 如果不是就直接进 min_cut(i-1)
    // 否则，就是 min_cut(i-1) = min(1, min_cut(j-1))
    pub fn min_cut(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        if n <= 1 {
            return 0;
        }
        // is_pal[i][j] = s[i..=j] 是否是回文，小dp
        let mut is_pal = vec![vec![false; n]; n];
        // 从短到长填表：长度1和2是base case，长度>=3依赖 is_pal[i+1][j-1]
        for i in (0..n).rev() {
            for j in i..n {
                is_pal[i][j] = s[i] == s[j] && (j - i < 2 || is_pal[i + 1][j - 1]);
            }
        }
        // dp[i] = s[0..=i] 的最少切割次数，数组从二维压缩成一维
        let mut dp = vec![0i32; n];
        for i in 0..n {
            if is_pal[0][i] {
                // 整段就是回文，不需要切
                dp[i] = 0;
            } else {
                // 枚举最后一段回文 s[j..=i]，在 j 前面切一刀
                dp[i] = i as i32; // 最坏情况：每个字符切一刀
                for j in 1..=i {
                    if is_pal[j][i] {
                        dp[i] = dp[i].min(dp[j - 1] + 1);
                    }
                }
            }
        }
        dp[n - 1]
    }

    pub fn min_cut_2(s: String) -> i32 {
        // 不需要回文数组，每个位置做中心向两侧扩散，直接更新 dp
        let s = s.as_bytes();
        let n = s.len();
        if n <= 1 {
            return 0;
        }
        // dp[i] = s[0..=i] 的最少切割次数，初始化为最坏情况（每个字符切一刀）
        let mut dp: Vec<i32> = (0..n as i32).collect();
        for mid in 0..n {
            // 奇数长度回文：以 mid 为中心
            Self::expand(s, &mut dp, mid as i32, mid as i32);
            // 偶数长度回文：以 mid, mid+1 为中心
            Self::expand(s, &mut dp, mid as i32, mid as i32 + 1);
        }
        dp[n - 1]
    }

    // 从 (l, r) 向两侧扩散，每发现一个回文 s[l..=r] 就更新 dp[r]
    fn expand(s: &[u8], dp: &mut [i32], mut l: i32, mut r: i32) {
        let n = s.len() as i32;
        while l >= 0 && r < n && s[l as usize] == s[r as usize] {
            if l == 0 {
                dp[r as usize] = 0;
            } else {
                dp[r as usize] = dp[r as usize].min(dp[l as usize - 1] + 1);
            }
            l -= 1;
            r += 1;
        }
    }
}

#[cfg(test)]
mod test_lc132 {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::min_cut("aab".to_string()), 1);
        assert_eq!(Solution::min_cut("a".to_string()), 0);
        assert_eq!(Solution::min_cut("ab".to_string()), 1);
        assert_eq!(Solution::min_cut("aba".to_string()), 0);
        assert_eq!(Solution::min_cut("aabb".to_string()), 1);
        assert_eq!(Solution::min_cut("abcba".to_string()), 0);
        assert_eq!(Solution::min_cut("abcd".to_string()), 3);

        assert_eq!(Solution::min_cut_2("aab".to_string()), 1);
        assert_eq!(Solution::min_cut_2("a".to_string()), 0);
        assert_eq!(Solution::min_cut_2("ab".to_string()), 1);
        assert_eq!(Solution::min_cut_2("aba".to_string()), 0);
        assert_eq!(Solution::min_cut_2("aabb".to_string()), 1);
        assert_eq!(Solution::min_cut_2("abcba".to_string()), 0);
        assert_eq!(Solution::min_cut_2("abcd".to_string()), 3);
    }
}
