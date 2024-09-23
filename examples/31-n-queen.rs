fn main() {
    assert_eq!(0, Solution::total_n_queens(0));
    assert_eq!(1, Solution::total_n_queens(1));
    assert_eq!(0, Solution::total_n_queens(2));
    assert_eq!(0, Solution::total_n_queens(3));
    assert_eq!(2, Solution::total_n_queens(4));
    assert_eq!(10, Solution::total_n_queens(5));
    assert_eq!(4, Solution::total_n_queens(6));
    assert_eq!(40, Solution::total_n_queens(7));
    assert_eq!(92, Solution::total_n_queens(8));
    assert_eq!(352, Solution::total_n_queens(9));
}

struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        if n < 1 {
            0
        } else {
            let limit = (1 << n) - 1;
            Self::next_row(limit, 0, 0, 0)
        }
    }

    fn next_row(limit: i32, col: i32, left: i32, right: i32) -> i32 {
        if limit == col {
            1
        } else {
            // 当前行可以放的列有哪些，1表示可以放的位置
            let mut total_limit = !(col | left | right) & limit;
            // 更新当前行选择的列的位置，获取最后一个1
            let mut loc = total_limit & (-total_limit);
            let mut res = 0;
            while loc > 0 {
                res += Self::next_row(
                    limit,
                    col | loc,
                    ((left | loc) >> 1) & 0x7fff_ffff,
                    (right | loc) << 1,
                );
                // 对于可以选择的内容，要去掉那个选择的1
                total_limit ^= loc;
                loc = total_limit & (-total_limit);
            }
            res
        }
    }

    pub fn total_n_queens1(n: i32) -> i32 {
        if n < 1 {
            0
        } else {
            let mut path_rec = vec![0; n as usize];
            Self::put(&mut path_rec, 0, n as usize)
        }
    }

    /// 将皇后摆放到当前行`cur_row`中，棋盘大小为`N*N`
    /// 如果当前行已经超过棋盘，那么这个路径是可行的，返回1
    /// 否则在当前行任意列放置，根据之前的放置方式`path`来判断是否可以
    /// 如果可以放置，则递归获取下一行的可放置可能次数
    fn put(path: &mut Vec<usize>, cur_row: usize, total: usize) -> i32 {
        if cur_row == total {
            return 1;
        }

        let mut res = 0;
        for col in 0..total {
            path[cur_row] = col;
            if Self::check(path, cur_row, col) {
                res += Self::put(path, cur_row + 1, total);
            }
        }
        res
    }

    fn check(path: &[usize], cur_row: usize, cur_col: usize) -> bool {
        for (last_row, &last_col) in path.iter().take(cur_row).enumerate() {
            if cur_col == last_col || cur_row.abs_diff(last_row) == cur_col.abs_diff(last_col) {
                return false;
            }
        }
        true
    }
}
