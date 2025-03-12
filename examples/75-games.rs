fn main() {}

struct Solution;

impl Solution {
    // true: 先手
    // false: 后手
    fn bash_games(n: i32, m: i32) -> bool {
        n % (m + 1) != 0
    }

    fn dumb_bash_games(n: i32, m: i32) -> bool {
        Self::dbg(n, m, &mut vec![-1; n as usize + 1])
    }

    // 还剩n个石子，一次拿1～m个，当前人是否能赢
    fn dbg(n: i32, m: i32, dp: &mut [i32]) -> bool {
        if n == 0 {
            false
        } else if dp[n as usize] != -1 {
            if dp[n as usize] == 0 { false } else { true }
        } else {
            for i in 1..=m {
                if n >= i && !Self::dbg(n - i, m, dp) {
                    dp[n as usize] = 1;
                    return true;
                }
            }
            dp[n as usize] = 0;
            false
        }
    }
}

#[cfg(test)]
mod test_solution {
    use rand::{Rng, thread_rng};

    #[test]
    fn test_bash_games() {
        let times = 2_000;
        let mut rng = thread_rng();
        for _ in 0..times {
            let n = rng.gen_range(1..=100);
            let m = rng.gen_range(1..=100);
            let bright = super::Solution::bash_games(n, m);
            let dumb = super::Solution::dumb_bash_games(n, m);
            assert_eq!(
                bright, dumb,
                "bright({}) dumb({}), n({}) m({})",
                bright, dumb, n, m
            )
        }
    }
}
