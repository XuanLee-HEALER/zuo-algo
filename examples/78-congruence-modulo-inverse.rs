fn main() {
    println!("res: {}", Solution::num_music_playlists(3, 3, 1))
}

struct Solution;

impl Solution {
    fn exp(mut a: i32, mut b: i32, r: i32) -> i32 {
        let mut res = 1;
        while b > 0 {
            if b & 1 != 0 {
                res = (res as i64 * a as i64 % r as i64) as i32
            }
            a = (a as i64 * a as i64 % r as i64) as i32;
            b >>= 1;
        }
        res
    }

    fn inv(a: i32, r: i32) -> i32 {
        Self::exp(a, r - 2, r)
    }

    fn div(a: i32, b: i32, r: i32) -> i32 {
        Self::multi(a % r, Self::inv(b, r), r)
    }

    fn multi(a: i32, b: i32, r: i32) -> i32 {
        (a as i64 * b as i64 % r as i64) as i32
    }

    fn combination(n: i32, m: i32, r: i32) -> i32 {
        let n_ = (1..=n).reduce(|a, b| Self::multi(a, b, r)).unwrap();
        let n_i = Self::inv(n_, r);
        let mut inv = vec![n_i; n as usize + 1];
        for u in (1..n).rev() {
            inv[u as usize] = (inv[(u + 1) as usize] as i64 * (u + 1) as i64 % r as i64) as i32
        }
        Self::multi(
            Self::multi(n_, inv[m as usize], r),
            inv[(n - m) as usize],
            r,
        )
    }

    fn dumb_combination(n: i32, m: i32, r: i32) -> i32 {
        let m = m;
        let n = n;
        let r = r;
        ((1..=n).reduce(|a, b| a * b).unwrap() as i64
            / (1..=m).reduce(|a, b| a * b).unwrap() as i64
            / (1..=(n - m)).reduce(|a, b| a * b).unwrap() as i64
            % r as i64) as i32
    }

    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        let mut fac = vec![1; (n + 1) as usize];
        let mut inv = vec![1; (n + 1) as usize];
        for i in 1..=n {
            fac[i as usize] = (i as i64 * fac[(i - 1) as usize] as i64 % Self::MOD as i64) as i32;
        }
        inv[n as usize] = Self::exp(fac[n as usize], Self::MOD - 2, Self::MOD);
        for i in (0..n).rev() {
            inv[i as usize] =
                ((i + 1) as i64 * inv[(i + 1) as usize] as i64 % Self::MOD as i64) as i32;
        }
        let mut sign = 1;
        let mut res = 0;
        for i in 0..n - k {
            let mut c = (fac[n as usize] as i64 * Self::exp(n - i - k, goal - k, Self::MOD) as i64
                % Self::MOD as i64) as i32;
            c = (c as i64 * inv[i as usize] as i64 % Self::MOD as i64) as i32;
            c = (c as i64 * inv[(n - i - k) as usize] as i64 % Self::MOD as i64) as i32;
            c = (c as i64 * sign as i64 % Self::MOD as i64) as i32;
            res = ((res as i64 + c as i64) % Self::MOD as i64) as i32;
            sign = if sign == 1 { Self::MOD - 1 } else { 1 };
        }
        res
    }

    const MOD: i32 = 1_000_000_007;
}

#[cfg(test)]
mod test_solution {
    #[test]
    fn test_div() {
        let times = 1_000_000;
        let base = 738291;
        let modx = 1_000_000_007;
        for b in 1..=times {
            assert_eq!(
                ((b as i64 * base as i64) / b as i64 % modx as i64) as i32,
                super::Solution::div(super::Solution::multi(b, base, modx), b, modx),
                "a {} b {}",
                b * base,
                b
            )
        }
    }

    #[test]
    fn test_combination() {
        let num = 10;
        let modx = 1_000_000_007;
        for v in 1..=9 {
            assert_eq!(
                super::Solution::combination(num, v, modx),
                super::Solution::dumb_combination(num, v, modx),
                "n is {} m is {}",
                num,
                v
            )
        }
    }
}
