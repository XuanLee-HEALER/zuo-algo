fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));

    // let res = preorder_traversal(root);
    // println!("{:?}", res);

    // let res = inorder_traversal(root);
    // println!("{:?}", res);

    let res = postorder_traversal_stack1(root);
    println!("result:\n{:?}", res);
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

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();

    if root.is_none() {
        return res;
    }

    let mut aid_stack = Vec::new();
    aid_stack.push(root.unwrap());

    while !aid_stack.is_empty() {
        if let Some(node) = aid_stack.pop() {
            res.push(node.borrow().val);

            if let Some(right) = node.borrow_mut().right.take() {
                aid_stack.push(right);
            }

            if let Some(left) = node.borrow_mut().left.take() {
                aid_stack.push(left);
            }
        }
    }

    res
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();

    if root.is_none() {
        return res;
    }

    let mut root = root;
    let mut aid_stack = Vec::new();
    while let Some(node) = root {
        aid_stack.push(Rc::clone(&node));
        root = node.borrow_mut().left.take();
    }

    while !aid_stack.is_empty() {
        if let Some(left) = aid_stack.pop() {
            res.push(left.borrow().val);

            // 将右子树的左边界全部压入栈中
            let mut right_ref = left.borrow_mut().right.take();
            while let Some(t_node) = right_ref {
                aid_stack.push(Rc::clone(&t_node));
                right_ref = t_node.borrow_mut().left.take();
            }
        }
    }

    res
}

fn postorder_traversal_stack2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();

    if root.is_none() {
        return res;
    }

    let mut aid_stack = Vec::new();
    aid_stack.push(root.unwrap());

    while !aid_stack.is_empty() {
        if let Some(node) = aid_stack.pop() {
            res.push(node.borrow().val);

            if let Some(left) = node.borrow_mut().left.take() {
                aid_stack.push(left);
            }

            if let Some(right) = node.borrow_mut().right.take() {
                aid_stack.push(right);
            }
        }
    }

    res.reverse();
    res
}

fn postorder_traversal_stack1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    if root.is_none() {
        return res;
    }

    let mut aid_stack = Vec::new();
    let root = root.unwrap();
    let mut last_access = Rc::clone(&root);
    aid_stack.push(root);

    while !aid_stack.is_empty() {
        let mut push_left = false;
        let mut push_right = false;
        let mut pop = false;
        if let Some(top) = aid_stack.last() {
            let top_inter = top.borrow();
            if top_inter.left.is_some() && top_inter.right.is_some() {
                if Rc::ptr_eq(&last_access, top_inter.left.as_ref().unwrap()) {
                    push_right = true;
                } else if Rc::ptr_eq(&last_access, top_inter.right.as_ref().unwrap()) {
                    pop = true;
                } else {
                    push_left = true;
                }
            } else if top_inter.left.is_some() {
                if Rc::ptr_eq(&last_access, top_inter.left.as_ref().unwrap()) {
                    pop = true;
                } else {
                    push_left = true;
                }
            } else if top_inter.right.is_some() {
                if Rc::ptr_eq(&last_access, top_inter.right.as_ref().unwrap()) {
                    pop = true;
                } else {
                    push_right = true;
                }
            } else {
                pop = true;
            }
        }

        if push_left {
            let left_sub = Rc::clone(aid_stack.last().unwrap().borrow().left.as_ref().unwrap());
            aid_stack.push(left_sub);
        }

        if push_right {
            let right_sub = Rc::clone(aid_stack.last().unwrap().borrow().right.as_ref().unwrap());
            aid_stack.push(right_sub);
        }

        if pop {
            last_access = aid_stack.pop().unwrap();
            res.push(last_access.borrow().val);
        }
    }

    res
}

fn postorder_traversal_stack1_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    if root.is_none() {
        return res;
    }

    let mut aid_stack = Vec::new();
    let root = root.unwrap();
    let mut last_access = Rc::clone(&root);
    aid_stack.push(root);

    while !aid_stack.is_empty() {
        let mut push_left = false;
        let mut push_right = false;
        let mut pop = false;
        if let Some(top) = aid_stack.last() {
            let top_inter = top.borrow();
            if top_inter.left.is_some() && top_inter.right.is_some() {
                if last_access == *top_inter.left.as_ref().unwrap() {
                    push_right = true;
                } else if last_access == *top_inter.right.as_ref().unwrap() {
                    pop = true;
                } else {
                    push_left = true;
                }
            } else if top_inter.left.is_some() {
                if last_access == *top_inter.left.as_ref().unwrap() {
                    pop = true;
                } else {
                    push_left = true;
                }
            } else if top_inter.right.is_some() {
                if last_access == *top_inter.right.as_ref().unwrap() {
                    pop = true;
                } else {
                    push_right = true;
                }
            } else {
                pop = true;
            }
        }

        if push_left {
            let left_sub = Rc::clone(aid_stack.last().unwrap().borrow().left.as_ref().unwrap());
            aid_stack.push(left_sub);
        }

        if push_right {
            let right_sub = Rc::clone(aid_stack.last().unwrap().borrow().right.as_ref().unwrap());
            aid_stack.push(right_sub);
        }

        if pop {
            last_access = aid_stack.pop().unwrap();
            res.push(last_access.borrow().val);
        }
    }

    res
}
