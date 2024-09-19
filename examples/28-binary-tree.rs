fn main() {
    // [1,2,3,null,null,4,5]
    let mut root = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.right.as_mut().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.right.as_mut().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

    let mut codec = Codec::new();
    let ser_result = codec.serialize(Some(Rc::new(RefCell::new(root))));
    println!("{}", ser_result);
    assert_ne!(codec.deserialize(ser_result.to_owned()), None);

    let build_tree = Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]);
    println!("{:?}", codec.serialize(build_tree));
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

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;
use std::{i32, i64};
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        Self::level_order2(root)
    }

    /// 哈希表记录当前节点的层数
    /// 弹出节点时需判断当前节点层数，如果此层数有子链表，则插入进去，否则创建新链表
    fn level_order1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        Vec::new()
    }

    fn level_order2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        const QUEUE_LEN: usize = 2_001;
        let mut queue: [Option<Rc<RefCell<TreeNode>>>; QUEUE_LEN] = [const { None }; QUEUE_LEN];
        let (mut l, mut r) = (0, 0);
        queue[r] = root;
        r += 1;
        let mut res = Vec::new();
        'out: loop {
            let size = r - l;
            if size == 0 {
                break;
            }
            let mut sub = Vec::new();
            for _ in 0..size {
                if let Some(ref mut node) = queue[l] {
                    sub.push(node.borrow().val);
                    l += 1;
                    let left = node.borrow_mut().left.take();
                    let right = node.borrow_mut().right.take();
                    if left.is_some() {
                        queue[r] = left;
                        r += 1;
                    }
                    if right.is_some() {
                        queue[r] = right;
                        r += 1;
                    }
                } else {
                    break 'out;
                }
            }
            res.push(sub);
        }
        res
    }

    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        if root.is_none() {
            return res;
        }
        const QUEUE_LEN: usize = 2_001;
        let mut queue: [Option<Rc<RefCell<TreeNode>>>; QUEUE_LEN] = [const { None }; QUEUE_LEN];
        let (mut l, mut r) = (0, 0);
        queue[r] = root;
        r += 1;
        let mut reverse = false;
        loop {
            let size = r - l;
            if size == 0 {
                break;
            }
            let mut sub = Vec::new();
            if reverse {
                for i in (l..=r - 1).rev() {
                    if let Some(ref node) = queue[i] {
                        sub.push(node.borrow().val);
                    }
                }
            } else {
                for node in queue.iter().take(r).skip(l).flatten() {
                    sub.push(node.borrow().val)
                }
            };
            reverse = !reverse;
            for _ in 0..size {
                if let Some(ref mut node) = queue[l] {
                    l += 1;
                    let left = node.borrow_mut().left.take();
                    let right = node.borrow_mut().right.take();
                    if left.is_some() {
                        queue[r] = left;
                        r += 1;
                    }
                    if right.is_some() {
                        queue[r] = right;
                        r += 1;
                    }
                }
            }
            res.push(sub);
        }
        res
    }

    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 1;
        if root.is_none() {
            return 0;
        }
        const QUEUE_LEN: usize = 3_001;
        let mut queue: [Option<Rc<RefCell<TreeNode>>>; QUEUE_LEN] = [const { None }; QUEUE_LEN];
        let mut i_queue: [usize; QUEUE_LEN] = [0; QUEUE_LEN];
        let (mut l, mut r) = (0, 0);
        queue[r] = root;
        i_queue[r] = 1;
        r += 1;
        loop {
            let size = r - l;
            if size == 0 {
                break;
            }

            res = ((i_queue[r - 1] - i_queue[l]) + 1).max(res);

            for _ in 0..size {
                if let Some(ref mut node) = queue[l] {
                    let cal_basis = i_queue[l];
                    l += 1;
                    let left = node.borrow_mut().left.take();
                    let right = node.borrow_mut().right.take();
                    if left.is_some() {
                        queue[r] = left;
                        i_queue[r] = cal_basis * 2;
                        r += 1;
                    }
                    if right.is_some() {
                        queue[r] = right;
                        i_queue[r] = cal_basis * 2 + 1;
                        r += 1;
                    }
                }
            }
        }
        res as i32
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let root_ref = root.borrow_mut();
            Self::max_depth(root_ref.left.clone()).max(Self::max_depth(root_ref.right.clone())) + 1
        } else {
            0
        }
    }

    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(root) => {
                let root_ref = root.borrow();
                match (root_ref.left.clone(), root_ref.right.clone()) {
                    (None, None) => 1,
                    (Some(left), None) => Self::min_depth(Some(left)) + 1,
                    (None, Some(right)) => Self::min_depth(Some(right)) + 1,
                    (Some(left), Some(right)) => {
                        Self::min_depth(Some(left)).min(Self::min_depth(Some(right))) + 1
                    }
                }
            }
        }
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut mp = HashMap::new();
        for (i, e) in inorder.iter().enumerate() {
            mp.insert(*e, i);
        }

        let l = preorder.len();
        Self::build_tree_aid(&preorder, 0, l - 1, 0, l - 1, &mp)
    }

    fn build_tree_aid(
        preorder: &[i32],
        pre_i: usize,
        pre_j: usize,
        in_i: usize,
        in_j: usize,
        mp: &HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        println!("{} {}", pre_i, pre_j);
        match pre_i.cmp(&pre_j) {
            Ordering::Less => {
                let mut new_node = TreeNode::new(preorder[pre_i]);
                let &head_idx = mp.get(&preorder[pre_i]).unwrap();
                let l_num = head_idx - in_i;
                new_node.left = Self::build_tree_aid(
                    preorder,
                    pre_i + 1,
                    pre_i + l_num,
                    in_i,
                    head_idx - 1,
                    mp,
                );
                let r_num = in_j - head_idx;
                new_node.right = Self::build_tree_aid(
                    preorder,
                    pre_i + l_num + 1,
                    pre_i + l_num + r_num,
                    head_idx + 1,
                    in_j,
                    mp,
                );
                Some(Rc::new(RefCell::new(new_node)))
            }
            Ordering::Equal => Some(Rc::new(RefCell::new(TreeNode::new(preorder[pre_i])))),
            Ordering::Greater => None,
        }
    }

    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        const Q_LEN: usize = 101;
        let mut q: [Option<Rc<RefCell<TreeNode>>>; Q_LEN] = [const { None }; Q_LEN];
        let (mut l, mut r) = (0, 0);
        q[r] = root;
        r += 1;
        let mut last_non_leave = false;
        while l < r {
            if let Some(ref mut node) = q[l] {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                if left.is_none() && right.is_some() {
                    return false;
                }

                if last_non_leave && (left.is_some() || right.is_some()) {
                    return false;
                }

                if left.is_none() || right.is_none() {
                    last_non_leave = true;
                }

                l += 1;
                if left.is_some() {
                    q[r] = left;
                    r += 1;
                }
                if right.is_some() {
                    q[r] = right;
                    r += 1;
                }
            }
        }

        true
    }

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            0
        } else {
            Self::count_nodes_depth(root)
        }
    }

    fn count_nodes_depth(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = node {
            let l_depth = Self::depth(node.borrow().left.clone());
            let r_depth = Self::depth(node.borrow().right.clone());
            match l_depth.cmp(&r_depth) {
                Ordering::Greater => {
                    return (1 << r_depth) + Self::count_nodes_depth(node.borrow().left.clone())
                }
                Ordering::Equal => {
                    return (1 << l_depth) + Self::count_nodes_depth(node.borrow().right.clone())
                }
                Ordering::Less => unreachable!(),
            }
        } else {
            0
        }
    }

    fn depth(mut node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;
        while let Some(n) = node {
            res += 1;
            node = n.borrow().left.clone();
        }
        res
    }

    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            root
        } else if let Some(root) = root {
            let left_sub = root.borrow_mut().left.take();
            let right_sub = root.borrow_mut().right.take();
            let left_node = Self::lowest_common_ancestor(left_sub, p.clone(), q.clone());
            let right_node = Self::lowest_common_ancestor(right_sub, p.clone(), q.clone());

            if left_node.is_none() && right_node.is_none() {
                None
            } else if left_node.is_some() && right_node.is_some() {
                Some(root)
            } else if right_node.is_some() {
                right_node
            } else {
                left_node
            }
        } else {
            unreachable!()
        }
    }

    pub fn lowest_common_ancestor_search(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            root
        } else if let Some(root) = root {
            let p_ref = p.as_ref().unwrap();
            let q_ref = q.as_ref().unwrap();
            let min = p_ref.borrow().val.min(q_ref.borrow().val);
            let max = p_ref.borrow().val.max(q_ref.borrow().val);
            let cur_val = root.borrow().val;

            if cur_val > max {
                Self::lowest_common_ancestor_search(
                    root.borrow_mut().left.take(),
                    p.clone(),
                    q.clone(),
                )
            } else if cur_val < min {
                Self::lowest_common_ancestor_search(
                    root.borrow_mut().right.take(),
                    p.clone(),
                    q.clone(),
                )
            } else {
                Some(root)
            }
        } else {
            unreachable!()
        }
    }

    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut path_rec = Vec::new();
        Self::iter_tree(root, target_sum, 0, &mut path_rec, &mut res);
        res
    }

    fn iter_tree(
        root: Option<Rc<RefCell<TreeNode>>>,
        aim: i32,
        sum: i32,
        path: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        if let Some(root) = root {
            let root_ref = root.borrow();
            path.push(root.borrow().val);
            if root_ref.left.is_none() && root_ref.right.is_none() {
                if root_ref.val + sum == aim {
                    res.push(path.clone());
                }
            } else {
                if root_ref.left.is_some() {
                    Self::iter_tree(root_ref.left.clone(), aim, sum + root_ref.val, path, res);
                }

                if root_ref.right.is_some() {
                    Self::iter_tree(root_ref.right.clone(), aim, sum + root_ref.val, path, res);
                }
            }
            path.pop();
        }
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut res = true;
        Self::depth1(root, &mut res);
        res
    }

    fn depth1(node: Option<Rc<RefCell<TreeNode>>>, ans: &mut bool) -> i32 {
        if let Some(node) = node {
            let left_depth = Self::depth1(node.borrow().left.clone(), ans);
            let right_depth = Self::depth1(node.borrow().right.clone(), ans);
            if (left_depth - right_depth).abs() > 1 {
                *ans = false
            }
            left_depth.max(right_depth) + 1
        } else {
            0
        }
    }

    pub fn is_valid_bst_1(mut root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        const STACK_LEN: usize = 10_001;
        let mut stack: [Option<Rc<RefCell<TreeNode>>>; STACK_LEN] = [const { None }; STACK_LEN];
        let mut min = i64::MIN;
        let (l, mut r) = (0, 0);

        while let Some(node) = root {
            let left_node = node.borrow().left.clone();
            stack[r] = Some(node);
            r += 1;
            root = left_node;
        }

        while l < r {
            if let Some(ref node) = stack[r - 1] {
                r -= 1;
                let cur_val = node.borrow().val;
                if cur_val as i64 > min {
                    min = cur_val as i64;
                } else {
                    return false;
                }

                let mut right_sub = node.borrow().right.clone();
                if right_sub.is_some() {
                    while let Some(node) = right_sub {
                        let left_node = node.borrow().left.clone();
                        stack[r] = Some(node);
                        r += 1;
                        right_sub = left_node;
                    }
                }
            }
        }

        true
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut def_min = i64::MAX;
        let mut def_max = i64::MIN;
        Self::is_valid_bst_x(root, &mut def_min, &mut def_max)
    }

    fn is_valid_bst_x(node: Option<Rc<RefCell<TreeNode>>>, min: &mut i64, max: &mut i64) -> bool {
        if let Some(ref node) = node {
            let l_res = Self::is_valid_bst_x(node.borrow().left.clone(), min, max);
            let (l_min, l_max) = (*min, *max);
            let r_res = Self::is_valid_bst_x(node.borrow().right.clone(), min, max);
            let (r_min, r_max) = (*min, *max);
            let cur_val = node.borrow().val as i64;
            if cur_val > l_max && cur_val < r_min {
                // 更新最大最小值的方法，左边最小取小值，右边最大取大值
                // 左边最小值默认是最大值，即可能为空，则用min更新最小值
                // 右边最大值默认是最小值，即可能为空，则用max更新最大值
                *min = l_min.min(cur_val);
                *max = r_max.max(cur_val);
                l_res && r_res
            } else {
                false
            }
        } else {
            *min = i64::MAX;
            *max = i64::MIN;
            true
        }
    }

    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
    }
}

const QUEUE_LEN: usize = 10_001;

struct Codec {
    queue: [Option<Rc<RefCell<TreeNode>>>; QUEUE_LEN],
}

impl Codec {
    fn new() -> Self {
        Self {
            queue: [const { None }; QUEUE_LEN],
        }
    }

    /// 层序遍历序列化
    fn serialize(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        if root.is_none() {
            return "".to_owned();
        }
        let (mut l, mut r) = (0, 0);
        self.queue[r] = root;
        r += 1;
        let mut res = String::new();
        res.push_str(format!("{},", self.queue[l].as_ref().unwrap().borrow().val).as_str());

        while l < r {
            if let Some(ref mut node) = self.queue[l] {
                l += 1;
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                if left.is_some() {
                    res.push_str(format!("{},", left.as_ref().unwrap().borrow().val).as_str());
                    self.queue[r] = left;
                    r += 1;
                } else {
                    res.push_str("#,");
                }
                if right.is_some() {
                    res.push_str(format!("{},", right.as_ref().unwrap().borrow().val).as_str());
                    self.queue[r] = right;
                    r += 1;
                } else {
                    res.push_str("#,");
                }
            }
        }

        res
    }

    /// 层序遍历反序列化
    fn deserialize(&mut self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            None
        } else {
            let nodes = data
                .split(",")
                .filter(|&seg| !seg.is_empty())
                .collect::<Vec<&str>>();
            let create_node = |s: &str| {
                if s == "#" {
                    None
                } else {
                    Some(Rc::new(RefCell::new(TreeNode::new(s.parse().unwrap()))))
                }
            };
            let mut idx = 0;
            let head = create_node(nodes[idx]);
            idx += 1;
            let (mut l, mut r) = (0, 0);
            self.queue[r] = head.clone();
            r += 1;

            while l < r {
                if let Some(ref mut node) = self.queue[l] {
                    l += 1;
                    let left_node = create_node(nodes[idx]);
                    node.borrow_mut().left = left_node.clone();
                    idx += 1;
                    let right_node = create_node(nodes[idx]);
                    node.borrow_mut().right = right_node.clone();
                    idx += 1;
                    if left_node.is_some() {
                        self.queue[r] = left_node;
                        r += 1;
                    }
                    if right_node.is_some() {
                        self.queue[r] = right_node;
                        r += 1;
                    }
                }
            }

            head
        }
    }

    fn serialize1(&self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut res = String::new();
        Self::f1(root, &mut res);
        res
    }

    fn f1(node: Option<Rc<RefCell<TreeNode>>>, res: &mut String) {
        match node {
            None => res.push_str("#,"),
            Some(node) => {
                let node_ref = node.borrow();
                res.push_str(format!("{},", node_ref.val).as_str());
                Self::f1(node_ref.left.clone(), res);
                Self::f1(node_ref.right.clone(), res);
            }
        }
    }

    fn deserialize1(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() {
            None
        } else {
            let nodes = data
                .split(",")
                .filter(|&seg| !seg.is_empty())
                .collect::<Vec<&str>>();
            Self::g1(&nodes[..], &mut 0)
        }
    }

    fn g1(ser: &[&str], idx: &mut usize) -> Option<Rc<RefCell<TreeNode>>> {
        match ser[*idx] {
            "#" => {
                *idx += 1;
                None
            }
            other => {
                let mut cur_node = TreeNode::new(other.parse().unwrap());
                *idx += 1;
                let left = Self::g1(ser, idx);
                let right = Self::g1(ser, idx);
                cur_node.left = left;
                cur_node.right = right;
                Some(Rc::new(RefCell::new(cur_node)))
            }
        }
    }
}
