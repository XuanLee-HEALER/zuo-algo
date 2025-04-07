fn main() {
    println!("res: {}", Solution::longest_palindrome("a".into()));
    println!("res: {}", Solution::count_substrings("aaa".into()));
    println!(
        "res: {}",
        Solution::max_palindromes("fttfjofpnpfydwdwdnns".into(), 2)
    )
}

struct Solution;

impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let mut res = 0;
        let mut beg = 0;
        let s = Self::manacher_s(&s);
        let mut p = vec![1; s.len()];
        while let Some(nxt) = Self::mp(&s, beg, k, &mut p) {
            res += 1;
            beg = nxt;
        }
        res
    }

    fn mp(s: &str, beg: usize, k: i32, p: &mut [usize]) -> Option<usize> {
        let s = s.as_bytes();
        let n = s.len();
        let (mut i, mut c, mut r) = (beg, beg, beg);
        while i < n {
            let mut len = if r > i { p[c * 2 - i].min(r - i) } else { 1 };
            while i + len < n && i >= beg + len && s[i + len] == s[i - len] {
                len += 1;
                if len > k as usize {
                    return Some(if s[i + len - 1] == b'#' {
                        i + len - 1
                    } else {
                        i + len
                    });
                }
            }
            if i + len > r {
                r = i + len;
                c = i;
            }
            i += 1;
        }
        None
    }

    pub fn count_substrings(s: String) -> i32 {
        Self::cs(&s) as i32
    }

    fn cs(s: &str) -> usize {
        let s = Self::manacher_s(s);
        let s = s.as_bytes();
        let n = s.len();
        let mut p = vec![1; n];
        let (mut i, mut c, mut r, mut res) = (0, 0, 0, 0);
        while i < n {
            let mut len = if r > i { p[c * 2 - i].min(r - i) } else { 1 };
            while i + len < n && i >= len && s[i + len] == s[i - len] {
                len += 1;
            }
            if i + len > r {
                r = i + len;
                c = i;
            }
            p[i] = len;
            res += if s[i] == b'#' { (len - 1) / 2 } else { len / 2 };
            i += 1
        }
        res
    }

    pub fn longest_palindrome(s: String) -> String {
        let (end, l) = Self::lp(&s);
        println!("end is {}, l is {}", end, l);
        s[end - l..end].into()
    }

    fn lp(s: &str) -> (usize, usize) {
        let s = Self::manacher_s(s);
        let s = s.as_bytes();
        let n = s.len();
        let mut p = vec![1; n];
        let (mut i, mut c, mut r) = (0, 0, 0);
        let (mut max_e, mut max_l) = (0, 0);
        while i < n {
            let mut len = if r > i { p[c * 2 - i].min(r - i) } else { 1 };
            while i + len < n && i >= len && s[i + len] == s[i - len] {
                len += 1;
            }
            if i + len > r {
                c = i;
                r = i + len;
                if len - 1 > max_l {
                    max_e = (i + len - 1) / 2;
                    max_l = len - 1;
                }
            }
            p[i] = len;
            i += 1;
        }
        (max_e, max_l)
    }

    fn manacher_s(s: &str) -> String {
        let mut res = vec![b'#'; s.len() * 2 + 1];
        for (i, &b) in s.as_bytes().iter().enumerate() {
            res[2 * i + 1] = b;
        }
        String::from_utf8(res).unwrap()
    }
}
