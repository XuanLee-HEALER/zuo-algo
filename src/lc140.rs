use std::collections::HashSet;

struct Solution;

impl Solution {
    // 定义子问题 dfs(i) 表示 s[i..] 的所有合法切分，word_dict 存 hashset 做快速确认
    // 枚举 j 从 i+1 到 n，如果 s[i..j] 在词典中：
    //   j == n → 直接收集 s[i..j] 作为一个完整结果
    //   j < n  → 收集 s[i..j] + " " + dfs(j) 的每个结果
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let dict: HashSet<&str> = word_dict.iter().map(|w| w.as_str()).collect();
        let mut res = Vec::new();
        Self::dfs(&s, 0, &dict, &mut Vec::new(), &mut res);
        res
    }

    fn dfs(s: &str, i: usize, dict: &HashSet<&str>, path: &mut Vec<String>, res: &mut Vec<String>) {
        if i == s.len() {
            res.push(path.join(" "));
            return;
        }
        for j in i + 1..=s.len() {
            if dict.contains(&s[i..j]) {
                path.push(s[i..j].to_string());
                Self::dfs(s, j, dict, path, res);
                path.pop();
            }
        }
    }
}

#[cfg(test)]
mod test_lc140 {
    use super::*;

    #[test]
    fn test() {
        let mut r1 = Solution::word_break(
            "catsanddog".to_string(),
            vec!["cat", "cats", "and", "sand", "dog"].into_iter().map(String::from).collect(),
        );
        r1.sort();
        assert_eq!(r1, vec!["cat sand dog", "cats and dog"]);

        let mut r2 = Solution::word_break(
            "pineapplepenapple".to_string(),
            vec!["apple", "pen", "applepen", "pine", "pineapple"].into_iter().map(String::from).collect(),
        );
        r2.sort();
        assert_eq!(r2, vec![
            "pine apple pen apple",
            "pine applepen apple",
            "pineapple pen apple",
        ]);

        let r3 = Solution::word_break(
            "catsandog".to_string(),
            vec!["cats", "dog", "sand", "and", "cat"].into_iter().map(String::from).collect(),
        );
        assert!(r3.is_empty());
    }
}
