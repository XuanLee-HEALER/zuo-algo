use std::{cell::RefCell, collections::HashMap, i32, rc::Rc};

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
    })));
    println!("res {}", Solution::max_bst(root.clone()));
    println!("res {}", Solution::max_sum_bst(root));
}

struct Solution;

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

struct MbInfo {
    min: i64,
    max: i64,
    is_bst: bool,
    bst_size: i32,
}

struct MsbInfo {
    min: i32,
    max: i32,
    is_bst: bool,
    sum: i32,
    bst_sum: i32,
}

struct DobtInfo {
    height: i32,
    diameter: i32,
}

struct DcInfo {
    size: i32,
    coin: i32,
    min_times: i32,
}

impl Solution {
    pub fn max_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::mb(root).bst_size
    }

    fn mb(root: Option<Rc<RefCell<TreeNode>>>) -> MbInfo {
        match root {
            Some(ref node) => {
                let l_info = Self::mb(node.borrow().left.clone());
                let r_info = Self::mb(node.borrow().right.clone());
                let cur = node.borrow();
                let mut is_bst = false;
                let mut res = l_info.bst_size.max(r_info.bst_size);
                if l_info.is_bst
                    && r_info.is_bst
                    && (cur.val as i64) > l_info.max
                    && (cur.val as i64) < r_info.min
                {
                    is_bst = true;
                    res = res.max(l_info.bst_size + r_info.bst_size + 1)
                }
                MbInfo {
                    min: l_info.min.min(r_info.min).min(cur.val as i64),
                    max: l_info.max.max(r_info.max).max(cur.val as i64),
                    is_bst,
                    bst_size: res,
                }
            }
            None => MbInfo {
                min: i64::MAX,
                max: i64::MIN,
                is_bst: true,
                bst_size: 0,
            },
        }
    }

    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::msb(root).bst_sum
    }

    fn msb(root: Option<Rc<RefCell<TreeNode>>>) -> MsbInfo {
        match root {
            Some(ref node) => {
                let l_info = Self::msb(node.borrow().left.clone());
                let r_info = Self::msb(node.borrow().right.clone());
                let cur = node.borrow();
                let mut is_bst = false;
                let mut res = l_info.bst_sum.max(r_info.bst_sum);
                if l_info.is_bst && r_info.is_bst && cur.val > l_info.max && cur.val < r_info.min {
                    is_bst = true;
                    res = res.max(l_info.sum + r_info.sum + cur.val)
                }
                MsbInfo {
                    min: l_info.min.min(r_info.min).min(cur.val),
                    max: l_info.max.max(r_info.max).max(cur.val),
                    is_bst,
                    sum: l_info.sum + r_info.sum + cur.val,
                    bst_sum: res,
                }
            }
            None => MsbInfo {
                min: i32::MAX,
                max: i32::MIN,
                is_bst: true,
                sum: 0,
                bst_sum: 0,
            },
        }
    }

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dobt(root).diameter
    }

    fn dobt(root: Option<Rc<RefCell<TreeNode>>>) -> DobtInfo {
        match root {
            Some(ref node) => {
                let l_info = Self::dobt(node.borrow().left.clone());
                let r_info = Self::dobt(node.borrow().right.clone());
                let mut res = l_info.diameter.max(r_info.diameter);
                res = res.max(l_info.height + r_info.height);
                DobtInfo {
                    height: l_info.height.max(r_info.height) + 1,
                    diameter: res,
                }
            }
            None => DobtInfo {
                height: 0,
                diameter: 0,
            },
        }
    }

    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::dc(root).min_times
    }

    fn dc(root: Option<Rc<RefCell<TreeNode>>>) -> DcInfo {
        match root {
            Some(ref node) => {
                let l_info = Self::dc(node.borrow().left.clone());
                let r_info = Self::dc(node.borrow().right.clone());
                let lft_sub = (l_info.coin - l_info.size).abs();
                let rt_sub = (r_info.coin - r_info.size).abs();
                let res = if lft_sub != 0 || rt_sub != 0 {
                    lft_sub + rt_sub + l_info.min_times + r_info.min_times
                } else {
                    l_info.min_times + r_info.min_times
                };
                DcInfo {
                    size: l_info.size + r_info.size + 1,
                    coin: l_info.coin + r_info.coin + node.borrow().val,
                    min_times: res,
                }
            }
            None => DcInfo {
                size: 0,
                coin: 0,
                min_times: 0,
            },
        }
    }

    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        let root_status = Self::mcc(root, &mut ans);
        if root_status == 0 {
            ans += 1
        }
        ans
    }

    fn mcc(root: Option<Rc<RefCell<TreeNode>>>, ans: &mut i32) -> i32 {
        match root {
            Some(ref node) => {
                let node = node.borrow();
                let left = Self::mcc(node.left.clone(), ans);
                let right = Self::mcc(node.right.clone(), ans);
                if left == 0 || right == 0 {
                    *ans += 1;
                    2
                } else if left == 1 && right == 1 {
                    0
                } else {
                    1
                }
            }
            None => 1,
        }
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut tb = HashMap::new();
        tb.insert(0, 1);
        Self::ps(root, target_sum, 0, &mut tb)
    }

    fn ps(
        root: Option<Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        sum: i64,
        presum: &mut HashMap<i64, i32>,
    ) -> i32 {
        match root {
            Some(ref node) => {
                let node = node.borrow();
                let cur_sum = node.val as i64 + sum;
                let mut res = *presum.get(&(cur_sum - target_sum as i64)).unwrap_or(&0);
                presum.entry(cur_sum).and_modify(|e| *e += 1).or_insert(1);
                res += Self::ps(node.left.clone(), target_sum, cur_sum, presum);
                res += Self::ps(node.right.clone(), target_sum, cur_sum, presum);
                presum.entry(cur_sum).and_modify(|e| *e -= 1);
                res
            }
            None => 0,
        }
    }
}
