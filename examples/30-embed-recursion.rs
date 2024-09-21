fn main() {
    let expr1 = "1+2+3";
    assert_eq!(6, Solution::calculate(expr1.into()));
    let expr1 = "3*(3-1)";
    assert_eq!(6, Solution::calculate(expr1.into()));
    let expr1 = "34+2*(3-(16-3*2))";
    assert_eq!(20, Solution::calculate(expr1.into()));
    let expr1 = "(1)";
    assert_eq!(1, Solution::calculate(expr1.into()));
    // let expr1 = "3*(3-1)";
    // assert_eq!(6, Solution::calculate(expr1.into()));
    // let expr1 = "3*(3-1)";
    // assert_eq!(6, Solution::calculate(expr1.into()));
    // let expr1 = "3*(3-1)";
    // assert_eq!(6, Solution::calculate(expr1.into()));
    // let expr1 = "3*(3-1)";
    // assert_eq!(6, Solution::calculate(expr1.into()));
}

struct Solution;

impl Solution {
    fn calculate(expr: String) -> i32 {
        let mut where_now = 0;
        Self::cal(&expr.chars().collect::<Vec<char>>(), &mut where_now)
    }

    fn cal(expr: &[char], where_now: &mut usize) -> i32 {
        let mut cur = 0;
        let mut num_stack = Vec::new();
        let mut op_stack = Vec::new();
        while *where_now < expr.len() {
            match expr[*where_now] {
                c if c >= '0' && c <= '9' => cur = cur * 10 + c.to_digit(10).unwrap() as i32,
                '(' => {
                    *where_now += 1;
                    cur = Self::cal(expr, where_now);
                }
                ')' => {
                    Self::push_stack(&mut num_stack, &mut op_stack, cur, '+');
                    return Self::dumb_cal(&num_stack, &op_stack);
                }
                c if c == '+' || c == '-' || c == '*' || c == '/' => {
                    Self::push_stack(&mut num_stack, &mut op_stack, cur, c);
                    cur = 0;
                }
                _ => {
                    panic!("illegal expression")
                }
            }
            *where_now += 1;
        }

        Self::push_stack(&mut num_stack, &mut op_stack, cur, '+');
        Self::dumb_cal(&num_stack, &op_stack)
    }

    fn push_stack(nums: &mut Vec<i32>, op: &mut Vec<char>, num: i32, o: char) {
        if nums.is_empty() || o == '*' || o == '/' {
            nums.push(num);
            op.push(o);
        } else {
            if let Some(last) = op.pop() {
                match last {
                    '*' => {
                        let num = num * nums.pop().unwrap();
                        nums.push(num);
                        op.push(o);
                    }
                    '/' => {
                        let num = num / nums.pop().unwrap();
                        nums.push(num);
                        op.push(o);
                    }
                    _ => {
                        nums.push(num);
                        op.push(last);
                        op.push(o);
                    }
                }
            }
        }
    }

    fn dumb_cal(nums: &[i32], op: &[char]) -> i32 {
        let mut idx = 0;
        let mut base = nums[idx];
        idx += 1;
        for o in op[..op.len() - 1].iter() {
            match o {
                '+' => {
                    base += nums[idx];
                    idx += 1;
                }
                '-' => {
                    base -= nums[idx];
                    idx -= 1;
                }
                _ => unreachable!(),
            }
        }
        base
    }
}
