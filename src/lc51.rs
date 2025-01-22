struct Solution;

// The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.

// Given an integer n, return all distinct solutions to the n-queens puzzle. You may return the answer in any order.

// Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space, respectively.

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut cache = vec![];
        let mut res = vec![];
        Self::queens(n, 0, 0, 0, 0, &mut cache, &mut res);
        res
    }

    fn queens(
        n: i32,
        last_status: i32,
        left: i32,
        right: i32,
        cur: i32,
        cache: &mut Vec<i32>,
        res: &mut Vec<Vec<String>>,
    ) {
        if cur == n {
            let new_res = cache.iter().map(|&v| Self::status_to_str(n, v)).collect();
            res.push(new_res);
        } else {
            for i in 0..n {
                let cur_try = 1 << i;
                if cur_try & last_status == 0 && cur_try & left == 0 && cur_try & right == 0 {
                    cache.push(cur_try);
                    Self::queens(
                        n,
                        cur_try | last_status,
                        (left << 1) | (cur_try << 1),
                        (right >> 1) | (cur_try >> 1),
                        cur + 1,
                        cache,
                        res,
                    );
                    cache.pop();
                }
            }
        }
    }

    fn status_to_str(n: i32, status: i32) -> String {
        let mut res = String::new();
        for i in (0..n).rev() {
            if status & (1 << i) != 0 {
                res.push('Q');
            } else {
                res.push('.');
            }
        }
        res
    }
}

#[cfg(test)]
mod test_solution {
    #[test]
    fn test_status_to_str() {
        assert_eq!(super::Solution::status_to_str(4, 10), "Q.Q.");
        assert_eq!(super::Solution::status_to_str(4, 1), "...Q");
        println!("{:?}", super::Solution::solve_n_queens(4))
    }
}
