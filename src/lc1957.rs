struct Solution;

/// A fancy string is a string where no three consecutive characters are equal.
///
/// Given a string s, delete the minimum possible number of characters from s to make it fancy.
///
/// Return the final string after the deletion. It can be shown that the answer will always be unique.

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        if s.len() < 3 {
            return s;
        }

        let mut ans = String::new();
        let cs = s.chars().collect::<Vec<char>>();
        let mut l = 0;
        let mut r = 1;
        ans.push(cs[l]);
        while r < cs.len() {
            ans.push(cs[r]);
            if cs[l] == cs[r] {
                while r < cs.len() {
                    if cs[r] == cs[l] {
                        r += 1;
                    } else {
                        break;
                    }
                }
                l = r - 1;
            } else {
                l += 1;
                r += 1;
            }
        }

        ans
    }

    pub fn make_fancy_string1(s: String) -> String {
        let mut ans = String::new();
        let mut count = 0;
        let mut last = '\0';
        s.chars().for_each(|c| {
            if c == last {
                count += 1;
                if count < 3 {
                    ans.push(c);
                }
            } else {
                count = 1;
                ans.push(c);
                last = c;
            }
        });

        ans
    }
}

#[cfg(test)]
mod test_lc1957 {
    #[test]
    fn test_lc1957() {
        assert_eq!(
            String::from("leetcode"),
            super::Solution::make_fancy_string("leeetcode".into())
        );
        assert_eq!(
            String::from("aabaa"),
            super::Solution::make_fancy_string("aaabaaaa".into())
        );
        assert_eq!(
            String::from("leetcode"),
            super::Solution::make_fancy_string1("leeetcode".into())
        );
        assert_eq!(
            String::from("aabaa"),
            super::Solution::make_fancy_string1("aaabaaaa".into())
        );
    }
}
