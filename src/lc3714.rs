use std::collections::HashMap;

struct Solution;

impl Solution {
    // s 中只有 a b c，balanced_string 的定义是每种字符出现次数相等（不要求每种都出现）
    // 分情况讨论：
    // 1. 单个字符：最长连续相同子串
    // 2. 两个字符（ab/ac/bc）：前缀和差值相等时，中间段两种字符数量相等
    // 3. 三个字符（abc）：同时满足 cnt_a-cnt_b 和 cnt_a-cnt_c 的差值组合首次出现的位置
    pub fn longest_balanced(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut res = 0i32;

        // 情况1：只含单个字符的最长连续子串
        let mut i = 0;
        while i < n {
            let mut j = i;
            while j < n && s[j] == s[i] {
                j += 1;
            }
            res = res.max((j - i) as i32);
            i = j;
        }

        // 情况2：只含两种字符，前缀和差值法
        // 对每对字符 (c1, c2)，遍历时只关注这两种字符
        // c1 计 +1，c2 计 -1，其他跳过（遇到第三种字符断开）
        for (c1, c2) in [(b'a', b'b'), (b'a', b'c'), (b'b', b'c')] {
            // map: diff -> 最早出现该 diff 的位置
            let mut map: HashMap<i32, i32> = HashMap::new();
            map.insert(0, -1);
            let mut diff = 0i32;
            let mut seg_start = 0; // 当前有效段的起始位置
            for i in 0..n {
                if s[i] == c1 {
                    diff += 1;
                } else if s[i] == c2 {
                    diff -= 1;
                } else {
                    // 遇到第三种字符，断开，重置
                    diff = 0;
                    seg_start = i + 1;
                    map.clear();
                    map.insert(0, seg_start as i32 - 1);
                    continue;
                }
                if let Some(&first) = map.get(&diff) {
                    res = res.max(i as i32 - first);
                } else {
                    map.insert(diff, i as i32);
                }
            }
        }

        // 情况3：三种字符都出现，前缀和差值对 (cnt_a-cnt_b, cnt_a-cnt_c)
        {
            let mut map: HashMap<(i32, i32), i32> = HashMap::new();
            map.insert((0, 0), -1);
            let mut ca = 0i32;
            let mut cb = 0i32;
            let mut cc = 0i32;
            for i in 0..n {
                match s[i] {
                    b'a' => ca += 1,
                    b'b' => cb += 1,
                    _ => cc += 1,
                }
                let key = (ca - cb, ca - cc);
                if let Some(&first) = map.get(&key) {
                    res = res.max(i as i32 - first);
                } else {
                    map.insert(key, i as i32);
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod test_lc3714 {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::longest_balanced("abcabc".to_string()), 6);
        assert_eq!(Solution::longest_balanced("aabc".to_string()), 3);
        assert_eq!(Solution::longest_balanced("aaa".to_string()), 3);
        assert_eq!(Solution::longest_balanced("abab".to_string()), 4);
        assert_eq!(Solution::longest_balanced("a".to_string()), 1);
        assert_eq!(Solution::longest_balanced("abc".to_string()), 3);
        assert_eq!(Solution::longest_balanced("abcab".to_string()), 3);
    }
}
