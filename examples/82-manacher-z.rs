fn main() {}

struct Solution;

impl Solution {
    pub fn minimum_time_to_initial_state(word: String, k: i32) -> i32 {
        let z = Self::z(&word);
        let k = k as usize;
        let n = z.len();
        let mut i = k;
        while i < z.len() {
            if z[i as usize] == n - i {
                return (i / k) as i32;
            }
            i += k;
        }
        ((n + k - 1) / k) as i32
    }

    fn z(s: &str) -> Vec<usize> {
        let s = s.as_bytes();
        let n = s.len();
        let mut z = vec![n; n];
        let (mut c, mut r) = (1, 1);
        let mut i = 1;
        while i < n {
            let mut len = if r > i { z[i - c].min(r - i) } else { 0 };
            while i + len < n && s[i + len] == s[len] {
                len += 1
            }
            if i + len > r {
                r = i + len;
                c = i
            }
            z[i] = len;
            i += 1;
        }
        z
    }
}
