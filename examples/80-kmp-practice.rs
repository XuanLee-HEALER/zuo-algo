fn main() {
    println!(
        "res: {}",
        Solution::find_good_strings(8, "leetcode".into(), "leetgoes".into(), "leet".into())
    );
    println!(
        "res: {}",
        Solution::find_good_strings(2, "aa".into(), "da".into(), "b".into())
    );
    // 398724010
    println!(
        "res: {}",
        Solution::find_good_strings(7, "xqrnpsg".into(), "yygblrq".into(), "plnl".into())
    );
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
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
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;

impl Solution {
    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut list = vec![];
        let mut head = head;
        while let Some(h) = head {
            list.push(h.val);
            head = h.next;
        }
        let nxt = Self::next(&list);
        Self::find(root, &list, &nxt, 0, list.len())
    }

    fn find(
        node: Option<Rc<RefCell<TreeNode>>>,
        arr: &[i32],
        nxt: &[i32],
        mut i: i32,
        m: usize,
    ) -> bool {
        if i as usize == m {
            true
        } else if node.is_none() {
            false
        } else {
            let node = node.unwrap();
            let node = node.borrow();
            while i != -1 && node.val != arr[i as usize] {
                i = nxt[i as usize]
            }
            Self::find(node.left.clone(), arr, nxt, i + 1, m)
                || Self::find(node.right.clone(), arr, nxt, i + 1, m)
        }
    }

    const MOD: i32 = 1_000_000_007;

    pub fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let evil = evil.as_bytes();
        let x = evil.len() as i32;
        let nxt = Self::next(evil);
        let mut dp = vec![vec![vec![-1; 2]; x as usize]; n as usize];
        let r1 = Self::fgs(s1, evil, &nxt, n, x, 0, 0, 0, &mut dp);
        for m in dp.iter_mut() {
            for m in m.iter_mut() {
                for m in m.iter_mut() {
                    *m = -1;
                }
            }
        }
        let r2 = Self::fgs(s2, evil, &nxt, n, x, 0, 0, 0, &mut dp);
        let mut r = (r2 - r1 + Self::MOD) % Self::MOD;
        if Self::kmp(s1, evil).is_none() {
            r = (r + 1) % Self::MOD
        }

        r
    }

    fn fgs(
        s1: &[u8],
        s2: &[u8],
        nxt: &[i32],
        n: i32,
        m: i32,
        i: i32,
        j: i32,
        k: i32,
        dp: &mut [Vec<Vec<i32>>],
    ) -> i32 {
        if j == m {
            0
        } else if i == n {
            1
        } else if dp[i as usize][j as usize][k as usize] != -1 {
            dp[i as usize][j as usize][k as usize]
        } else {
            let mut res = 0;
            if k == 0 {
                // 不能自由选
                for ti in b'a'..s1[i as usize] {
                    res =
                        (res + Self::fgs(
                            s1,
                            s2,
                            nxt,
                            n,
                            m,
                            i + 1,
                            Self::jump(s2, nxt, j, ti) + 1,
                            1,
                            dp,
                        )) % Self::MOD
                }
                res =
                    (res + Self::fgs(
                        s1,
                        s2,
                        nxt,
                        n,
                        m,
                        i + 1,
                        Self::jump(s2, nxt, j, s1[i as usize]) + 1,
                        0,
                        dp,
                    )) % Self::MOD
            } else {
                for ti in b'a'..=b'z' {
                    res =
                        (res + Self::fgs(
                            s1,
                            s2,
                            nxt,
                            n,
                            m,
                            i + 1,
                            Self::jump(s2, nxt, j, ti) + 1,
                            1,
                            dp,
                        )) % Self::MOD
                }
            }
            dp[i as usize][j as usize][k as usize] = res;
            res
        }
    }

    fn jump(s2: &[u8], nxt: &[i32], mut j: i32, b: u8) -> i32 {
        while j != -1 && b != s2[j as usize] {
            j = nxt[j as usize]
        }
        j
    }

    fn kmp<T: Eq>(arr1: &[T], arr2: &[T]) -> Option<usize> {
        let (n, m) = (arr1.len(), arr2.len());
        let (mut i, mut j) = (0, 0);
        let nxt = Self::next(arr2);
        while i < n && j < m {
            if arr1[i] == arr2[j] {
                i += 1;
                j += 1;
            } else if j == 0 {
                i += 1;
            } else {
                j = nxt[j] as usize
            }
        }
        if j == m { Some(i - j) } else { None }
    }

    fn next<T: Eq>(arr: &[T]) -> Vec<i32> {
        let n = arr.len();
        let mut res = vec![-1; n];
        if n <= 1 {
            return res;
        }
        res[1] = 0;
        let mut cn = 0;
        let mut i = 2;
        while i < n {
            if arr[i - 1] == arr[cn as usize] {
                cn += 1;
                res[i] = cn;
                i += 1;
            } else if cn == 0 {
                res[i] = cn;
                i += 1;
            } else {
                cn = res[cn as usize]
            }
        }
        res
    }
}
