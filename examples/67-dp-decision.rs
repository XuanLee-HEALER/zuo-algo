use std::collections::HashMap;

fn main() {
    println!(
        "res {:?}",
        Solution::smallest_sufficient_team(
            vec!["java".into(), "nodejs".into(), "reactjs".into()],
            vec![
                vec!["java".into()],
                vec!["nodejs".into()],
                vec!["nodejs".into(), "reactjs".into()]
            ]
        )
    )
}

struct Solution;

impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let n = req_skills.len();
        let m = people.len();
        let mut hp = HashMap::new();
        let mut id = 0_usize;
        for skill in req_skills {
            hp.insert(skill, id);
            id += 1;
        }
        let mut people_skill = vec![0; m];
        for (i, p) in people.iter().enumerate() {
            let mut c_skil = 0;
            for sk in p {
                hp.get(sk).map(|&v| {
                    c_skil |= 1 << v;
                });
            }
            people_skill[i] = c_skil;
        }
        let mut dp = vec![vec![-1; 1 << n]; m];
        let t_res = Self::sst_dp(&people_skill, n, m, 0, 0, &mut dp);
        let mut res = vec![0; t_res as usize];
        if dp[0][0] != -1 && dp[0][0] != i32::MAX {
            let mut i = 0;
            let mut j = 0;
            let mut k = 0;
            while k < t_res as usize {
                if i == m - 1 || dp[i][j] != dp[i + 1][j] {
                    res[k] = i as i32;
                    j |= people_skill[i] as usize;
                    i += 1;
                    k += 1;
                } else {
                    i += 1;
                }
            }
        }
        res
    }

    fn sst_dp(skills: &[i32], n: usize, m: usize, i: usize, s: i32, dp: &mut [Vec<i32>]) -> i32 {
        if s == (1 << n) - 1 {
            0
        } else if i == m {
            i32::MAX
        } else if dp[i][s as usize] != -1 {
            dp[i][s as usize]
        } else {
            let p1 = Self::sst_dp(skills, n, m, i + 1, s, dp);
            let mut p2 = Self::sst_dp(skills, n, m, i + 1, s | skills[i], dp);
            if p2 != i32::MAX {
                p2 = 1 + p2;
            }
            dp[i][s as usize] = p1.min(p2);
            dp[i][s as usize]
        }
    }
}
