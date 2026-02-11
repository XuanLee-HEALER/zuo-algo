struct Solution;

impl Solution {
    // 逆向dp
    // 子问题 dp[i][j] 是骑士从 (i,j) 出发到终点，进入 (i, j)时的最低血量
    // 边界 dp[n-1][m-1] = max(1, 1-dungeon[n-1][m-1])
    // 转移方程 need = min(dp[i-1][j], dp[i][j-1]) - dungeon[i][j]; dp[i][j] = max(1, need)
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let n = dungeon.len();
        let m = dungeon[0].len();
        let mut dp = vec![vec![0; m]; n];
        // 终点
        dp[n - 1][m - 1] = (1 - dungeon[n - 1][m - 1]).max(1);
        // 最后一列：只能往下走
        for i in (0..n - 1).rev() {
            dp[i][m - 1] = (dp[i + 1][m - 1] - dungeon[i][m - 1]).max(1);
        }
        // 最后一行：只能往右走
        for j in (0..m - 1).rev() {
            dp[n - 1][j] = (dp[n - 1][j + 1] - dungeon[n - 1][j]).max(1);
        }
        // 一般位置：选下方和右方中需求更小的
        for i in (0..n - 1).rev() {
            for j in (0..m - 1).rev() {
                let need = dp[i + 1][j].min(dp[i][j + 1]) - dungeon[i][j];
                dp[i][j] = need.max(1);
            }
        }
        dp[0][0]
    }
}

#[cfg(test)]
mod test_lc174 {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::calculate_minimum_hp(vec![
                vec![-2, -3, 3],
                vec![-5, -10, 1],
                vec![10, 30, -5],
            ]),
            7
        );
        assert_eq!(Solution::calculate_minimum_hp(vec![vec![0]]), 1);
        assert_eq!(Solution::calculate_minimum_hp(vec![vec![100]]), 1);
        assert_eq!(Solution::calculate_minimum_hp(vec![vec![-5]]), 6);
    }
}
