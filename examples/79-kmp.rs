use std::{cell::RefCell, rc::Rc};

fn main() {
    let s = "aabaaabaaac";
    let v = "aabaaac";
    println!("res is {:?}", Solution::kmp(s.as_bytes(), v.as_bytes()))
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if let Some(v) = Self::kmp(haystack.as_bytes(), needle.as_bytes()) {
            v as i32
        } else {
            -1
        }
    }

    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut v1 = Vec::with_capacity(2_001);
        let mut v2 = Vec::with_capacity(1_001);
        Self::ser_tree(root, &mut v1);
        Self::ser_tree(sub_root, &mut v2);
        if let Some(_) = Self::kmp(&v1, &v2) {
            true
        } else {
            false
        }
    }

    fn ser_tree(tree: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<Option<i32>>) {
        let mut stack = vec![tree];
        while let Some(node_opt) = stack.pop() {
            if let Some(node) = node_opt {
                res.push(Some(node.borrow().val)); // 假设 TreeNode 有一个 val 字段
                stack.push(node.borrow().right.clone());
                stack.push(node.borrow().left.clone());
            } else {
                res.push(None);
            }
        }
    }

    fn kmp<T: Eq>(seq1: &[T], seq2: &[T]) -> Option<usize> {
        let n = seq1.len();
        let m = seq2.len();
        let mut i = 0;
        let mut j = 0;
        let next = Self::next(seq2);
        while i < n && j < m {
            if seq1[i] == seq2[j] {
                i += 1;
                j += 1;
            } else if next[j] != -1 {
                j = next[j] as usize
            } else {
                i += 1;
                j = 0;
            }
        }
        if j == m { Some(i - m) } else { None }
    }

    fn next<T: Eq>(seq: &[T]) -> Vec<i32> {
        let n = seq.len();
        let mut res = vec![-1; n];
        if n <= 1 {
            return res;
        }
        res[1] = 0;
        let mut incr = 0;
        let mut i = 2;
        while i < n {
            if seq[i - 1] == seq[incr as usize] {
                incr += 1;
                res[i] = incr;
                i += 1;
            } else if incr > 0 {
                incr = res[incr as usize]
            } else {
                res[i] = 0;
                i += 1;
            }
        }
        res
    }
}
