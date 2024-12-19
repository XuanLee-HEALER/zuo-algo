fn main() {
    println!("match {}", Solution::is_match("aa".into(), "a*".into()))
}

struct Solution;

impl Solution {
    pub fn max_value_of_coins(piles: Vec<Vec<i32>>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![0; k + 1];
        for pile in &piles {
            let mut g = vec![0];
            let mut sum = 0;
            for &p in pile {
                sum += p;
                g.push(sum);
            }
            for j in (0..=k).rev() {
                for (ti, &tk) in g.iter().enumerate() {
                    dp[j] = dp[j].max(if j >= ti { dp[j - ti] + tk } else { 0 })
                }
            }
        }
        dp[k]
    }

    pub fn is_match(s: String, p: String) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let p = p.chars().collect::<Vec<char>>();
        let n = s.len();
        let m = p.len();
        let mut dp = vec![false; m + 1];
        dp[m] = true;
        for j in (0..m).rev() {
            dp[j] = if j + 1 < m && p[j + 1] == '*' {
                if j + 2 < m {
                    dp[j + 2]
                } else {
                    true
                }
            } else {
                false
            }
        }

        for i in (0..n).rev() {
            let mut rd = dp[m];
            dp[m] = false;
            for j in (0..m).rev() {
                let t = dp[j];
                if j + 1 < p.len() && p[j + 1] == '*' {
                    let res1 = j + 2 < p.len() && dp[j + 2];
                    let res2 = (s[i] == p[j] || p[j] == '.') && dp[j];
                    dp[j] = res1 || res2;
                } else {
                    dp[j] = (s[i] == p[j] || p[j] == '.') && rd
                }
                rd = t
            }
        }
        dp[0]
    }

    pub fn is_match_wildcard(s: String, p: String) -> bool {
        let s = s.as_bytes();
        let p = p.as_bytes();
        let n = s.len();
        let m = p.len();
        let mut dp = vec![false; m + 1];
        dp[m] = true;
        for j in (0..m).rev() {
            dp[j] = if p[j] == b'*' { dp[j + 1] } else { false }
        }

        for i in (0..n).rev() {
            let mut rd = dp[m];
            dp[m] = false;
            for j in (0..m).rev() {
                let t = dp[j];
                if p[j] == b'*' {
                    let res1 = dp[j + 1];
                    let res2 = dp[j];
                    dp[j] = res1 || res2;
                } else {
                    dp[j] = (s[i] == p[j] || p[j] == b'?') && rd
                }
                rd = t
            }
        }
        dp[0]
    }
}
