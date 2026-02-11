struct Solution;

impl Solution {
    // TODO: 对于s来说，求从0开始的前向最长回文子串序列，然后把后面（如果有）剩余的子串移到开头就是结果
    // 最长回文子串可以使用 KMP和rabin-carp hash来做，都实现一遍吧
    // KMP 做法：构造 s + '#' + rev(s)，求 next 数组
    // next 数组最后一个值就是 s 从0开始的最长回文前缀长度
    pub fn shortest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }
        let rev: String = s.chars().rev().collect();
        // 用 '#' 分隔防止 next 值越过 s 本身的长度
        let pat = format!("{s}#{rev}");
        let b = pat.as_bytes();
        let n = b.len();
        // KMP next 数组：next[i] = pat[0..=i] 的最长真前后缀长度
        let mut next = vec![0usize; n];
        let mut j = 0;
        for i in 1..n {
            while j > 0 && b[i] != b[j] {
                j = next[j - 1];
            }
            if b[i] == b[j] {
                j += 1;
            }
            next[i] = j;
        }
        // next 最后一个值 = 从0开始的最长回文前缀长度
        let pal_len = next[n - 1];
        let suffix: String = s[pal_len..].chars().rev().collect();
        format!("{suffix}{s}")
    }

    // Rabin-Karp hash 做法：正向和反向 hash 同时滚动，匹配时说明 s[0..=i] 是回文
    pub fn shortest_palindrome_rk(s: String) -> String {
        if s.len() <= 1 {
            return s;
        }
        let b = s.as_bytes();
        let n = b.len();
        let base: u64 = 131;
        let modp: u64 = 1_000_000_007;
        let mut fwd: u64 = 0; // s[0..=i] 的正向 hash
        let mut bwd: u64 = 0; // s[0..=i] 的反向 hash（当做回文来看）
        let mut power: u64 = 1; // base^i % modp
        let mut pal_len = 0; // 记录最长回文前缀长度
        for i in 0..n {
            let c = (b[i] - b'a' + 1) as u64;
            fwd = (fwd * base + c) % modp;
            bwd = (bwd + c * power) % modp;
            power = power * base % modp;
            if fwd == bwd {
                pal_len = i + 1;
            }
        }
        let suffix: String = s[pal_len..].chars().rev().collect();
        format!("{suffix}{s}")
    }
}

#[cfg(test)]
mod test_lc214 {
    use super::*;

    #[test]
    fn test() {
        // KMP
        assert_eq!(Solution::shortest_palindrome("aacecaaa".to_string()), "aaacecaaa");
        assert_eq!(Solution::shortest_palindrome("abcd".to_string()), "dcbabcd");
        assert_eq!(Solution::shortest_palindrome("a".to_string()), "a");
        assert_eq!(Solution::shortest_palindrome("".to_string()), "");
        assert_eq!(Solution::shortest_palindrome("aba".to_string()), "aba");

        // Rabin-Karp
        assert_eq!(Solution::shortest_palindrome_rk("aacecaaa".to_string()), "aaacecaaa");
        assert_eq!(Solution::shortest_palindrome_rk("abcd".to_string()), "dcbabcd");
        assert_eq!(Solution::shortest_palindrome_rk("a".to_string()), "a");
        assert_eq!(Solution::shortest_palindrome_rk("".to_string()), "");
        assert_eq!(Solution::shortest_palindrome_rk("aba".to_string()), "aba");
    }
}
