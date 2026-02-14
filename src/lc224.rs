struct Solution;

impl Solution {
    // 求解表达式，支持 + - () 这几种操作符
    // 使用栈：遇到 '(' 时把当前结果和符号压栈，进入子表达式
    // 遇到 ')' 时弹出栈顶，恢复外层的结果和符号
    pub fn calculate(s: String) -> i32 {
        let s = s.as_bytes();
        let mut stack: Vec<(i32, i32)> = Vec::new(); // (外层结果, 外层符号)
        let mut res = 0i32;
        let mut sign = 1i32;
        let mut i = 0;
        while i < s.len() {
            match s[i] {
                b' ' => {}
                b'+' => sign = 1,
                b'-' => sign = -1,
                b'(' => {
                    // 把当前结果和符号压栈，重新开始计算括号内
                    stack.push((res, sign));
                    res = 0;
                    sign = 1;
                }
                b')' => {
                    // 弹出外层，outer_res + outer_sign * 括号内结果
                    let (outer_res, outer_sign) = stack.pop().unwrap();
                    res = outer_res + outer_sign * res;
                }
                _ => {
                    // 数字：解析完整的多位数
                    let mut num = 0i32;
                    while i < s.len() && s[i].is_ascii_digit() {
                        num = num * 10 + (s[i] - b'0') as i32;
                        i += 1;
                    }
                    res += sign * num;
                    continue; // i 已经指向下一个字符，跳过末尾的 i += 1
                }
            }
            i += 1;
        }
        res
    }
}

#[cfg(test)]
mod test_lc224 {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::calculate("1 + 1".to_string()), 2);
        assert_eq!(Solution::calculate(" 2-1 + 2 ".to_string()), 3);
        assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
        assert_eq!(Solution::calculate("- (3 + (4 + 5))".to_string()), -12);
        assert_eq!(Solution::calculate("0".to_string()), 0);
    }
}
