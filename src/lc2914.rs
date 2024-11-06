struct Solution;

/// You are given a 0-indexed binary string s having an even length.
///
/// A string is beautiful if it's possible to partition it into one or more substrings such that:
///
/// Each substring has an even length.
/// Each substring contains only 1's or only 0's.
/// You can change any character in s to 0 or 1.
///
/// Return the minimum number of changes required to make the string s beautiful.

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut groups = vec![0; s.len()];
        let mut last = '\0';
        let mut idx = 0;
        let mut ans = 0;
        for c in s.chars() {
            if last == '\0' {
                last = c;
                groups[idx] += 1;
            } else if c == last {
                groups[idx] += 1;
            } else {
                last = c;
                idx += 1;
                groups[idx] += 1;
            }
        }

        let mut lend = 0;
        for &g in groups.iter().take(idx + 1) {
            if (g - lend) & 1 == 1 {
                ans += 1;
                lend = 1;
            } else {
                lend = 0;
            }
        }

        ans
    }
}

#[cfg(test)]
mod test_lc2914 {

    #[test]
    fn test_lc2914() {
        assert_eq!(2, super::Solution::min_changes("1001".into()));
    }
}
