fn main() {
    let root = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode {
            val: 2,
            left: Some(Box::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 5,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            })),
        })),
    }));

    pre_order(&root);
    println!();
    in_order(&root);
    println!();
    post_order(&root);
    println!();
}

struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

fn pre_order(node: &Option<Box<TreeNode>>) {
    if let Some(node) = node {
        print!("{} ", node.val);
        pre_order(&node.left);
        pre_order(&node.right);
    }
}

fn in_order(node: &Option<Box<TreeNode>>) {
    if let Some(node) = node {
        in_order(&node.left);
        print!("{} ", node.val);
        in_order(&node.right);
    }
}

fn post_order(node: &Option<Box<TreeNode>>) {
    if let Some(node) = node {
        post_order(&node.left);
        post_order(&node.right);
        print!("{} ", node.val);
    }
}
