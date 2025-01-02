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
    println!(
        "res {}",
        Solution::minimum_fuel_cost(vec![vec![0, 1], vec![0, 2], vec![0, 3]], 5)
    );
    println!(
        "res {}",
        Solution::longest_path(vec![-1, 0, 0, 1, 1, 2], "abacbe".into())
    );
    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
    })));
    println!("res {:?}", Solution::tree_queries(root, vec![4]));
    println!(
        "res {}",
        Solution::minimum_score(
            vec![5, 5, 2, 4, 4, 2],
            vec![vec![0, 1], vec![1, 2], vec![5, 2], vec![4, 3], vec![1, 3]]
        )
    );
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

struct LpInfo {
    max_path: i32,
    max_root_path: i32,
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

    pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
        let n = roads.len();
        let mut graph = vec![vec![]; n + 1];
        for road in &roads {
            let a = road[0];
            let b = road[1];
            graph[a as usize].push(b);
            graph[b as usize].push(a);
        }
        let mut size = vec![0; n + 1];
        let mut cost = vec![0_i64; n + 1];
        Self::mfc(&graph, seats, 0, -1, &mut size, &mut cost);
        cost[0]
    }

    fn mfc(
        graph: &[Vec<i32>],
        seats: i32,
        node: i32,
        par: i32,
        size: &mut [i32],
        cost: &mut [i64],
    ) {
        let node = node as usize;
        size[node] = 1;
        if graph[node].len() == 1 && graph[node][0] == par as i32 {
            cost[node] = 0;
        } else {
            for &sub in &graph[node] {
                if sub != par {
                    Self::mfc(graph, seats, sub, node as i32, size, cost);
                    let sub = sub as usize;
                    size[node] += size[sub];
                    cost[node] += cost[sub] + ((size[sub] + seats - 1) / seats) as i64;
                }
            }
        }
    }

    pub fn longest_path(parent: Vec<i32>, s: String) -> i32 {
        let n = parent.len();
        let mut graph = vec![vec![]; n];
        for (i, &par) in parent.iter().enumerate().skip(1) {
            graph[par as usize].push(i);
        }
        let s = s.as_bytes();
        let lp_info = Self::lp(&graph, s, 0);
        lp_info.max_path
    }

    fn lp(graph: &[Vec<usize>], s: &[u8], node: usize) -> LpInfo {
        if graph[node].len() == 0 {
            LpInfo {
                max_path: 1,
                max_root_path: 1,
            }
        } else {
            let mut res1 = 0;
            let mut first = 0;
            let mut second = 0;
            for &sub in &graph[node] {
                let sub_lp = Self::lp(graph, s, sub);
                res1 = res1.max(sub_lp.max_path);
                if s[node] != s[sub] {
                    if sub_lp.max_root_path > first {
                        second = first;
                        first = sub_lp.max_root_path
                    } else if sub_lp.max_root_path > second {
                        second = sub_lp.max_root_path
                    }
                }
            }
            let cur_max_root_path = first + 1;
            let cur_max_path = res1.max(first + second + 1);
            LpInfo {
                max_path: cur_max_path,
                max_root_path: cur_max_root_path,
            }
        }
    }

    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        const NODE_SZ: usize = 100_002;
        let mut dfn_cnt = 1_usize;
        let mut dfn = vec![0; NODE_SZ];
        let mut size = vec![0; NODE_SZ];
        let mut height = vec![0; NODE_SZ];
        let mut max_l = vec![0; NODE_SZ + 1];
        let mut max_r = vec![0; NODE_SZ + 1];
        Self::tq(root, 0, &mut dfn_cnt, &mut dfn, &mut size, &mut height);
        let n = size[1] as usize;
        let mut max_h = height[1];
        for (i, &h) in height.iter().enumerate().skip(1).take(n) {
            if h > max_h {
                max_h = h;
            }
            max_l[i] = max_h;
        }
        max_h = height[n as usize];
        for (i, &h) in height.iter().enumerate().skip(1).take(n).rev() {
            if h > max_h {
                max_h = h;
            }
            max_r[i] = max_h;
        }

        queries
            .into_iter()
            .map(|q| {
                let dfn_id = dfn[q as usize];
                let l = dfn_id;
                let r = (l as i32 + size[dfn_id] - 1) as usize;
                max_l[l - 1].max(max_r[r + 1])
            })
            .collect()
    }

    fn tq(
        root: Option<Rc<RefCell<TreeNode>>>,
        h: i32,
        dfn_cnt: &mut usize,
        dfn: &mut [usize],
        size: &mut [i32],
        height: &mut [i32],
    ) {
        if let Some(node) = root {
            let cur = node.borrow();
            let cur_dfn_cnt = *dfn_cnt;
            dfn[cur.val as usize] = *dfn_cnt;
            size[*dfn_cnt] = 1;
            height[*dfn_cnt] = h;
            *dfn_cnt += 1;
            Self::tq(cur.left.clone(), h + 1, dfn_cnt, dfn, size, height);
            Self::tq(cur.right.clone(), h + 1, dfn_cnt, dfn, size, height);
            if let Some(ref left) = cur.left {
                size[cur_dfn_cnt] += size[dfn[left.borrow().val as usize]]
            }
            if let Some(ref right) = cur.right {
                size[cur_dfn_cnt] += size[dfn[right.borrow().val as usize]]
            }
        }
    }

    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut graph = vec![vec![]; n];
        for edge in &edges {
            let n1 = edge[0] as usize;
            let n2 = edge[1] as usize;
            graph[n1].push(n2);
            graph[n2].push(n1);
        }
        let mut dfn = vec![0; n + 1];
        let mut dfn_cnt = 1;
        let mut size = vec![0; n + 1];
        let mut xor = vec![0; n + 1];
        Self::ms_dfn(
            &graph,
            &nums,
            &mut dfn,
            &mut dfn_cnt,
            &mut size,
            &mut xor,
            0,
        );
        let m = edges.len();
        let mut res = i32::MAX;
        for i in 0..m - 1 {
            let max_a = dfn[edges[i][0] as usize].max(dfn[edges[i][1] as usize]);
            for j in i + 1..m {
                let max_b = dfn[edges[j][0] as usize].max(dfn[edges[j][1] as usize]);
                let (pre, pos) = if max_a > max_b {
                    (max_b, max_a)
                } else {
                    (max_a, max_b)
                };
                let sum1 = xor[pos];
                let sum2 = if pos < pre + size[pre] {
                    // b子树是a子树的子树
                    xor[pre] ^ xor[pos]
                } else {
                    xor[pre]
                };
                let sum3 = xor[1] ^ sum1 ^ sum2;
                res = res.min(sum1.max(sum2).max(sum3) - sum1.min(sum2).min(sum3));
            }
        }

        res
    }

    fn ms_dfn(
        graph: &[Vec<usize>],
        nums: &[i32],
        dfn: &mut [usize],
        dfn_cnt: &mut usize,
        size: &mut [usize],
        xor: &mut [i32],
        node: usize,
    ) {
        let cur_dfn_cnt = *dfn_cnt;
        *dfn_cnt += 1;
        dfn[node] = cur_dfn_cnt;
        size[cur_dfn_cnt] = 1;
        xor[cur_dfn_cnt] = nums[node];
        for &sub in &graph[node] {
            if dfn[sub] == 0 {
                Self::ms_dfn(graph, nums, dfn, dfn_cnt, size, xor, sub);
                size[cur_dfn_cnt] += size[dfn[sub]];
                xor[cur_dfn_cnt] ^= xor[dfn[sub]];
            }
        }
    }
}
