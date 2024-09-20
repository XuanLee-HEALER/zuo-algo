use std::collections::HashSet;

use rand::{seq::SliceRandom, thread_rng};

fn main() {
    // let sol = Solution;
    // let res = sol.generatePermutation(String::from("abcde"));
    // res.iter().for_each(|o| println!("{}", o));
    // let res = Solution::subsets_with_dup(vec![1, 2, 2]);
    // res.iter().for_each(|o| println!("{:?}", o));
    let mut rng = thread_rng();
    let mut random_stack = (1..10).collect::<Vec<i32>>();
    random_stack.shuffle(&mut rng);
    println!("original stack: {:?}", random_stack);
    reverse_stack(&mut random_stack);
    println!("reversed stack: {:?}", random_stack);
    sort_stack(&mut random_stack);
    println!("sorted   stack: {:?}", random_stack);

    hanoi(5);
}

fn hanoi(n: i32) {
    print_hanoi(n, "left", "right", "middle");
}

fn print_hanoi(n: i32, from: &str, to: &str, aid: &str) {
    if n == 1 {
        println!("plate {:>4} from {} to {}", n, from, to)
    } else {
        print_hanoi(n - 1, from, aid, to);
        println!("plate {:>4} from {} to {}", n, from, to);
        print_hanoi(n - 1, aid, to, from);
    }
}

fn reverse_stack(stack: &mut Vec<i32>) {
    if !stack.is_empty() {
        let bottom = bottom_to_top(stack);
        reverse_stack(stack);
        stack.push(bottom);
    }
}

/// 不考虑栈为空的情况
fn bottom_to_top(stack: &mut Vec<i32>) -> i32 {
    let temp = stack.pop().unwrap();
    if stack.is_empty() {
        // 如果栈为空，那么这次就是栈底元素，返回掉
        temp
    } else {
        let res = bottom_to_top(stack);
        // 将当前元素重新入栈
        stack.push(temp);
        res
    }
}

fn sort_stack(stack: &mut Vec<i32>) {
    let mut ori_depth = depth_stack(stack);
    while ori_depth > 0 {
        let cur_max = max_stack(stack, ori_depth);
        let cur_max_num = max_num_stack(stack, ori_depth, cur_max);
        down_stack(stack, ori_depth, cur_max, cur_max_num);
        ori_depth -= cur_max_num;
    }
}

fn depth_stack(stack: &mut Vec<i32>) -> i32 {
    if stack.is_empty() {
        0
    } else {
        let temp = stack.pop().unwrap();
        let rest_depth = depth_stack(stack);
        stack.push(temp);
        rest_depth + 1
    }
}

fn max_stack(stack: &mut Vec<i32>, depth: i32) -> i32 {
    if depth == 0 {
        i32::MIN
    } else {
        let temp = stack.pop().unwrap();
        let rest_max = max_stack(stack, depth - 1);
        stack.push(temp);
        temp.max(rest_max)
    }
}

fn max_num_stack(stack: &mut Vec<i32>, depth: i32, max: i32) -> i32 {
    if depth == 0 {
        0
    } else {
        let temp = stack.pop().unwrap();
        let rest_max_num = max_num_stack(stack, depth - 1, max);
        stack.push(temp);
        if temp == max {
            rest_max_num + 1
        } else {
            rest_max_num
        }
    }
}

fn down_stack(stack: &mut Vec<i32>, depth: i32, max: i32, max_num: i32) {
    if depth == 0 {
        for _ in 0..max_num {
            stack.push(max);
        }
    } else {
        let temp = stack.pop().unwrap();
        down_stack(stack, depth - 1, max, max_num);
        if temp != max {
            stack.push(temp);
        }
    }
}

struct Solution;

impl Solution {
    pub fn generatePermutation(&self, s: String) -> Vec<String> {
        let mut sub = vec![' '; s.len()];
        // let mut sub = String::new();
        let mut filter = HashSet::new();
        // Self::collect(&s.chars().collect::<Vec<char>>(), 0, &mut sub, &mut filter);
        Self::collect1(
            &s.chars().collect::<Vec<char>>(),
            0,
            &mut sub,
            0,
            &mut filter,
        );
        filter.into_iter().collect::<Vec<String>>()
    }

    fn collect(chars: &[char], i: usize, sub: &mut String, filter: &mut HashSet<String>) {
        if i >= chars.len() {
            filter.insert(sub.clone());
        } else {
            sub.push(chars[i]);
            Self::collect(chars, i + 1, sub, filter);
            sub.pop();
            Self::collect(chars, i + 1, sub, filter);
        }
    }

    fn collect1(
        chars: &[char],
        i: usize,
        sub: &mut Vec<char>,
        size: usize,
        filter: &mut HashSet<String>,
    ) {
        if i >= chars.len() {
            filter.insert(sub[..size].iter().collect::<String>());
        } else {
            // 一定要作为数组使用，不能push，因为这种做法没有对应pop
            sub[size] = chars[i];
            Self::collect1(chars, i + 1, sub, size + 1, filter);
            // 不包含这个数只需要索引值退1
            Self::collect1(chars, i + 1, sub, size, filter);
        }
    }

    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut res = Vec::new();
        let mut sub = vec![0; nums.len()];
        Self::collect_subset(&nums, 0, &mut sub, 0, &mut res);
        res
    }

    /// 需要`size`来记录当前的子路径到哪里是有效的
    fn collect_subset(
        nums: &[i32],
        gi: usize,
        sub: &mut Vec<i32>,
        mut size: usize,
        res: &mut Vec<Vec<i32>>,
    ) {
        if gi >= nums.len() {
            res.push(sub[..size].into());
        } else {
            let mut i = gi;
            while i < nums.len() {
                if nums[i] == nums[gi] {
                    i += 1;
                } else {
                    break;
                }
            }
            Self::collect_subset(nums, i, sub, size, res);
            for loc in gi..i {
                sub[size] = nums[loc];
                size += 1;
                Self::collect_subset(nums, i, sub, size, res);
            }
        }
    }

    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::per(&mut nums, 0, &mut res);
        res
    }

    /// 使用原数组作为路径记录容器
    fn per(nums: &mut Vec<i32>, i: usize, res: &mut Vec<Vec<i32>>) {
        if i >= nums.len() {
            res.push(nums.clone());
        } else {
            for j in i..nums.len() {
                nums.swap(i, j);
                Self::per(nums, i + 1, res);
                nums.swap(i, j);
            }
        }
    }

    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::per_ii(&mut nums, 0, &mut res);
        res
    }

    fn per_ii(nums: &mut Vec<i32>, i: usize, res: &mut Vec<Vec<i32>>) {
        if i >= nums.len() {
            res.push(nums.clone());
        } else {
            let mut filter = HashSet::new();
            for j in i..nums.len() {
                if !filter.contains(&nums[j]) {
                    filter.insert(nums[j]);
                    nums.swap(i, j);
                    Self::per_ii(nums, i + 1, res);
                    nums.swap(i, j);
                }
            }
        }
    }
}
