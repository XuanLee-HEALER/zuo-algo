use std::collections::BTreeMap;

fn main() {
    let expr1 = "1+2+3";
    assert_eq!(6, Solution::calculate(expr1.into()));
    let expr1 = "3*(3-1)";
    assert_eq!(6, Solution::calculate(expr1.into()));
    let expr1 = "34+2*(3-(16-3*2))";
    assert_eq!(20, Solution::calculate(expr1.into()));
    let expr1 = "(1)";
    assert_eq!(1, Solution::calculate(expr1.into()));
    let expr1 = "3[a]2[bc]";
    assert_eq!("aaabcbc", Solution::decode_string(expr1.into()));
    let expr1 = "siod3[ab]2[bc]ds";
    assert_eq!("siodabababbcbcds", Solution::decode_string(expr1.into()));
    let expr1 = "Mg(OH)2";
    assert_eq!("H2MgO2", Solution::count_of_atoms(expr1.into()));
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
                c if c.is_ascii_digit() => cur = cur * 10 + c.to_digit(10).unwrap() as i32,
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
        } else if let Some(last) = op.pop() {
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

    pub fn decode_string(s: String) -> String {
        let chars = s.chars().collect::<Vec<char>>();
        let mut beg = 0;
        Self::sub(&chars, &mut beg)
    }

    fn sub(x_str: &[char], idx: &mut usize) -> String {
        let mut res = String::new();
        let mut cur_num = 0;
        while *idx < x_str.len() {
            match x_str[*idx] {
                c if c.is_ascii_digit() => {
                    cur_num = cur_num * 10 + c.to_digit(10).unwrap() as i32;
                }
                '[' => {
                    *idx += 1;
                    let nxt_str = Self::sub(x_str, idx);
                    res.push_str(&nxt_str.repeat(cur_num as usize));
                    cur_num = 0;
                }
                ']' => {
                    return res;
                }
                c if c.is_ascii_alphabetic() => {
                    res.push(c);
                }
                _ => panic!("illegal input"),
            }
            *idx += 1;
        }

        res
    }

    pub fn count_of_atoms(formula: String) -> String {
        let chars = formula.chars().collect::<Vec<char>>();
        let mut idx = 0;
        let chart = Self::count(&chars, &mut idx);
        let mut res = String::new();
        chart.iter().for_each(|(k, v)| {
            res.push_str(
                format!(
                    "{}{}",
                    k,
                    if *v == 1 {
                        "".to_owned()
                    } else {
                        v.to_string()
                    }
                )
                .as_str(),
            )
        });
        res
    }

    fn count(x_str: &[char], idx: &mut usize) -> BTreeMap<String, usize> {
        let mut res = BTreeMap::new();
        let mut last_atom = String::new();
        let mut last_sub = BTreeMap::new();
        let mut count = 0;
        while *idx < x_str.len() {
            match x_str[*idx] {
                c if c.is_ascii_uppercase() => {
                    Self::update_chart(&mut res, &last_atom, count, &last_sub);
                    last_atom.clear();
                    count = 0;
                    last_sub.clear();
                    last_atom.push(c);
                }
                c if c.is_ascii_lowercase() => {
                    last_atom.push(c);
                }
                c if c.is_ascii_digit() => {
                    count = count * 10 + c.to_digit(10).unwrap() as usize;
                }
                '(' => {
                    Self::update_chart(&mut res, &last_atom, count, &last_sub);
                    last_atom.clear();
                    count = 0;
                    last_sub.clear();
                    *idx += 1;
                    last_sub = Self::count(x_str, idx);
                }
                ')' => {
                    Self::update_chart(&mut res, &last_atom, count, &last_sub);
                    return res;
                }
                _ => panic!("illegal input"),
            }
            *idx += 1;
        }
        Self::update_chart(&mut res, &last_atom, count, &last_sub);

        res
    }

    fn update_chart(
        res: &mut BTreeMap<String, usize>,
        str: &str,
        count: usize,
        sub: &BTreeMap<String, usize>,
    ) {
        let count = if count == 0 { 1 } else { count };
        if !str.is_empty() {
            res.entry(str.to_owned())
                .and_modify(|e| *e += count)
                .or_insert(count);
        } else {
            for (k, v) in sub {
                res.entry(k.to_owned())
                    .and_modify(|e| *e += *v * count)
                    .or_insert(*v * count);
            }
        }
    }
}
