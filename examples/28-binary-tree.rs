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
