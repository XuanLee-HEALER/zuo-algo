const FAB: [usize; 24] = [
    1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946,
    17711, 28657, 46368, 75025,
];

fn main() {}

fn sg_three_fab(a: i32, b: i32, c: i32) -> bool {
    let max = a.max(b).max(c);
    let mut sg = vec![0; max as usize + 1];
    let mut appear = vec![false; max as usize + 1];
    for i in 1..=(max as usize) {
        appear.fill(false);
        for &f in &FAB {
            if f <= i {
                appear[sg[i - f] as usize] = true
            } else {
                break;
            }
        }
        for (ti, &b) in appear.iter().enumerate() {
            if !b {
                sg[i] = ti as usize;
                break;
            }
        }
    }
    sg[a as usize] ^ sg[b as usize] ^ sg[c as usize] != 0
}

fn dumb_three_fab(a: i32, b: i32, c: i32) -> bool {
    let mut dp = vec![vec![vec![-1; c as usize + 1]; b as usize + 1]; a as usize + 1];
    dtf(a, b, c, &mut dp)
}

fn dtf(a: i32, b: i32, c: i32, dp: &mut [Vec<Vec<i32>>]) -> bool {
    if a + b + c == 0 {
        false
    } else if dp[a as usize][b as usize][c as usize] != -1 {
        if dp[a as usize][b as usize][c as usize] == 1 {
            true
        } else {
            false
        }
    } else {
        for &f in &FAB {
            let f = f as i32;
            if f <= a && !dtf(a - f, b, c, dp) {
                dp[a as usize][b as usize][c as usize] = 1;
                return true;
            }
            if f <= b && !dtf(a, b - f, c, dp) {
                dp[a as usize][b as usize][c as usize] = 1;
                return true;
            }
            if f <= c && !dtf(a, b, c - f, dp) {
                dp[a as usize][b as usize][c as usize] = 1;
                return true;
            }
        }
        dp[a as usize][b as usize][c as usize] = 0;
        false
    }
}

// true 先手
// false 后手
fn sg_two_bash(a: i32, b: i32, m: i32) -> bool {
    a % (m + 1) != b % (m + 1)
}

fn dumb_two_bash(a: i32, b: i32, m: i32) -> bool {
    let mut dp = vec![vec![-1; b as usize + 1]; a as usize + 1];
    dtb(a, b, m, &mut dp)
}

fn dtb(a: i32, b: i32, m: i32, dp: &mut [Vec<i32>]) -> bool {
    if a + b == 0 {
        false
    } else if dp[a as usize][b as usize] != -1 {
        if dp[a as usize][b as usize] == 1 {
            true
        } else {
            false
        }
    } else {
        for i in 1..=m {
            if a >= i {
                if !dtb(a - i, b, m, dp) {
                    dp[a as usize][b as usize] = 1;
                    return true;
                }
            }
            if b >= i {
                if !dtb(a, b - i, m, dp) {
                    dp[a as usize][b as usize] = 1;
                    return true;
                }
            }
        }
        dp[a as usize][b as usize] = 0;
        false
    }
}

#[cfg(test)]
mod test_solution {
    use rand::{Rng, thread_rng};

    use crate::{dumb_three_fab, dumb_two_bash, sg_three_fab, sg_two_bash};

    #[test]
    fn test_two_bash() {
        let mut rng = thread_rng();
        let times = 2_000;
        for _ in 0..times {
            let a = rng.gen_range(1..=100);
            let b = rng.gen_range(1..=100);
            let m = rng.gen_range(1..=100);
            let bright = sg_two_bash(a, b, m);
            let dumb = dumb_two_bash(a, b, m);
            assert_eq!(
                bright, dumb,
                "bright {}\tdumb {}\ta {}\tb {}\tm {}",
                bright, dumb, a, b, m
            );
        }
    }

    #[test]
    fn test_three_fab() {
        let mut rng = thread_rng();
        let times = 200;
        for _ in 0..times {
            let a = rng.gen_range(1..=100);
            let b = rng.gen_range(1..=100);
            let c = rng.gen_range(1..=100);
            let bright = sg_three_fab(a, b, c);
            let dumb = dumb_three_fab(a, b, c);
            assert_eq!(
                bright, dumb,
                "bright {}\tdumb {}\ta {}\tb {}\tc {}",
                bright, dumb, a, b, c
            );
        }
    }
}
