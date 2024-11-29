use std::i32;

fn main() {}

fn min_delete(a: &str, b: &str) -> i32 {
    let a = a.as_bytes();
    let b = b.as_bytes();
    let n = a.len();
    let m = b.len();
    let mut dp = vec![0_i32; m + 1];
    for i in 1..=n {
        let mut lr = dp[0];
        dp[0] = i as i32;
        for (j, e) in dp.iter_mut().skip(1).enumerate() {
            let bk = *e;
            *e = if a[i - 1] == b[j] { lr } else { 1 + *e };
            lr = bk;
        }
    }
    *dp.iter().min().unwrap()
}

fn min_delete_noob(a: &str, b: &str) -> i32 {
    let mut buf = String::new();
    let mut sub_strs = Vec::new();
    substr(a.as_bytes(), &mut buf, 0, &mut sub_strs);
    let n = a.len();
    let mut res = usize::MAX;
    for s in &sub_strs {
        if b.contains(s) {
            res = res.min(n - s.len())
        }
    }
    res as i32
}

fn substr(s: &[u8], cur_str: &mut String, c: usize, res: &mut Vec<String>) {
    if c == s.len() {
        res.push(cur_str.clone());
    } else {
        substr(s, cur_str, c + 1, res);
        cur_str.push(s[c] as char);
        substr(s, cur_str, c + 1, res);
        cur_str.pop();
    }
}

const MOD: i32 = 1_000_000_007;

fn fill_colour(n: usize, m: usize) -> i32 {
    let mut dp = vec![0; m + 1];
    // 第一行和第一列的结果无意义
    // 第二行开始 1个格子，最多填充1种颜色，有M种可能性
    dp[1] = m as i32;
    // 从第三行开始
    for i in 2..=n {
        // 记录左上答案，因为任意行的第二列都是M（任意多格子填满一种颜色的结果都是M种可能）所以从第三列开始
        let mut lu = dp[1];
        // 从第三列开始截取到i位置，即选择i + 1 - 2个元素
        for (j, e) in dp.iter_mut().skip(2).take(i - 1).enumerate() {
            let bk = *e;
            *e = ((*e as i64 * (j + 2) as i64) % MOD as i64) as i32
                + ((lu as i64 * (m - j - 1) as i64) % MOD as i64) as i32;
            lu = bk;
        }
    }
    dp[m]
}

fn fill_colour_noob(n: usize, m: usize) -> i32 {
    let mut blocks = vec![0; n];
    let mut judge = vec![false; m];
    fill(
        &mut blocks,
        &mut judge,
        0,
        n,
        m,
        &mut (|path: &[usize], judge: &mut [bool], m: usize| -> bool {
            let mut colour = 0;
            for v in path {
                if !judge[*v] {
                    judge[*v] = true;
                    colour += 1;
                }
            }
            judge.fill(false);
            colour == m
        }),
    )
}

fn fill<F: FnMut(&[usize], &mut [bool], usize) -> bool>(
    path: &mut [usize],
    judge: &mut [bool],
    c: usize,
    n: usize,
    m: usize,
    f: &mut F,
) -> i32 {
    if c == n {
        if f(path, judge, m) {
            1
        } else {
            0
        }
    } else {
        let mut res = 0;
        for color in 0..m {
            path[c] = color;
            res += fill(path, judge, c + 1, n, m, f);
        }
        res
    }
}

struct Solution;

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let n = s.len();
        let m = t.len();
        let mut dp = vec![0; m + 1];
        dp[0] = 1;
        for i in 1..=n {
            for j in (1..=m).rev() {
                dp[j] += if s[i - 1] == t[j - 1] { dp[j - 1] } else { 0 }
            }
        }

        dp[m]
    }

    pub fn min_distance(word1: String, word2: String) -> i32 {
        Self::min_distance_1(word1.as_bytes(), word2.as_bytes(), 1, 1, 1)
    }

    // 编辑距离更通用的版本，插入、删除和编辑的代价不需要相同（题目要求相同）
    fn min_distance_1(word1: &[u8], word2: &[u8], ic: i32, dc: i32, ec: i32) -> i32 {
        let n = word1.len();
        let m = word2.len();
        let mut dp = vec![vec![0; m + 1]; n + 1];
        for (i, col) in dp[0].iter_mut().enumerate() {
            *col = ic * i as i32
        }
        for (i, row) in dp.iter_mut().enumerate() {
            row[0] = dc * i as i32
        }
        #[allow(clippy::needless_range_loop)]
        for i in 1..=n {
            for j in 1..=m {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1]
                } else {
                    dp[i][j] = (dp[i - 1][j - 1] + ec)
                        .min(dp[i - 1][j] + dc)
                        .min(dp[i][j - 1] + ic)
                }
            }
        }

        dp[n][m]
    }

    fn min_distance_2(word1: &[u8], word2: &[u8], ic: i32, dc: i32, ec: i32) -> i32 {
        let n = word1.len();
        let m = word2.len();
        // 长度为n+1，0..n每个值是i*ic
        let mut dp = (0..=m).map(|u| u as i32 * ic).collect::<Vec<i32>>();
        for i in 1..=n {
            let mut lu = dp[0];
            dp[0] = i as i32 * dc;
            let mut bk;
            for j in 1..=m {
                bk = dp[j];
                if word1[i - 1] == word2[j - 1] {
                    dp[j] = lu
                } else {
                    dp[j] = (dp[j] + ic).min(dp[j - 1] + dc).min(lu + ec)
                }
                lu = bk;
            }
        }

        dp[m]
    }

    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let n = s1.len();
        let m = s2.len();

        if n + m != s3.len() {
            false
        } else {
            let mut t = true;
            let mut dp = (0..=m)
                .map(|i| {
                    if t && s2[..i] != s3[..i] {
                        t = false
                    }
                    t
                })
                .collect::<Vec<bool>>();

            t = true;
            for i in 1..=n {
                if t && s1[..i] != s3[..i] {
                    t = false;
                }
                dp[0] = t;
                for j in 1..=m {
                    dp[j] = (s1[i - 1] == s3[i + j - 1] && dp[j])
                        || (s2[j - 1] == s3[i + j - 1] && dp[j - 1])
                }
            }
            dp[m]
        }
    }
}

#[cfg(test)]
mod test_match {
    use rand::{rngs::ThreadRng, thread_rng, Rng};

    use crate::{fill_colour, fill_colour_noob, min_delete, min_delete_noob, substr};

    #[test]
    fn test_fill_colour_fn() {
        for n in 1..=9 {
            for m in 1..=9 {
                let res1 = fill_colour(n, m);
                let res2 = fill_colour_noob(n, m);
                assert_eq!(res1, res2, "wrong answer: n:{} m:{}", n, m);
            }
        }
    }

    #[test]
    fn test_substr() {
        let ori_str = "abcd";
        let mut buf = String::new();
        let mut res = Vec::new();
        substr(ori_str.as_bytes(), &mut buf, 0, &mut res);
        for s in res {
            println!("{}", s)
        }
    }

    fn random_str(l: usize, rng: &mut ThreadRng) -> String {
        let mut res = String::new();
        for _ in 0..l {
            res.push(rng.gen_range(b'a'..=b'd') as char);
        }
        res
    }

    #[test]
    fn test_min_del() {
        let mut rng = thread_rng();
        let test_times = 1_000;
        for _ in 0..test_times {
            let a = random_str(12, &mut rng);
            let b = random_str(12, &mut rng);
            let res1 = min_delete(&a, &b);
            let res2 = min_delete_noob(&a, &b);
            println!("a {} b {} res {} {}", a, b, res1, res2);
            assert_eq!(res1, res2, "wrong answer: a:{} b:{}", a, b)
        }
    }
}
