struct Solution;

impl Solution {
    // valid_number = (integer | decimal) + optional(exponent)
    // integer      = optional(sign) + digits
    // decimal      = optional(sign) + (digits "." | digits "." digits | "." digits)
    // exponent     = ("e" | "E") + integer
    pub fn is_number(s: String) -> bool {
        let s = s.as_bytes();
        if s.is_empty() {
            return false;
        }
        // 从 e/E 分段
        let (left, right) = match s.iter().position(|&c| c == b'e' || c == b'E') {
            Some(pos) => (&s[..pos], Some(&s[pos + 1..])),
            None => (s, None),
        };
        // 左段必须是 integer 或 decimal
        let left_ok = Self::is_integer(left) || Self::is_decimal(left);
        if !left_ok {
            return false;
        }
        // 有指数部分时，右段必须是 integer
        match right {
            Some(r) => Self::is_integer(r),
            None => true,
        }
    }

    // optional(sign) + digits（至少一位数字）
    fn is_integer(s: &[u8]) -> bool {
        let s = Self::skip_sign(s);
        !s.is_empty() && s.iter().all(|c| c.is_ascii_digit())
    }

    // optional(sign) + (digits "." | digits "." digits | "." digits)
    // 即：有且仅有一个点，且点的两侧至少有一侧有数字
    fn is_decimal(s: &[u8]) -> bool {
        let s = Self::skip_sign(s);
        let dot_pos = match s.iter().position(|&c| c == b'.') {
            Some(p) => p,
            None => return false,
        };
        let before = &s[..dot_pos];
        let after = &s[dot_pos + 1..];
        // 点两侧都是纯数字，且至少一侧非空
        (before.is_empty() || before.iter().all(|c| c.is_ascii_digit()))
            && (after.is_empty() || after.iter().all(|c| c.is_ascii_digit()))
            && !(before.is_empty() && after.is_empty())
    }

    fn skip_sign(s: &[u8]) -> &[u8] {
        if !s.is_empty() && (s[0] == b'+' || s[0] == b'-') {
            &s[1..]
        } else {
            s
        }
    }
}

#[cfg(test)]
mod test_lc65 {
    use super::*;

    #[test]
    fn test() {
        let valid = ["2", "0089", "-0.1", "+3.14", "4.", ".9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"];
        let invalid = ["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53", "."];
        for s in valid {
            assert!(Solution::is_number(s.to_string()), "{s} should be valid");
        }
        for s in invalid {
            assert!(!Solution::is_number(s.to_string()), "{s} should be invalid");
        }
    }
}
