fn main() {
    println!("{}", Solution::largest_island(vec![vec![1, 0], vec![0, 1]]))
}

struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '1' {
                    ans += 1;
                    Self::check_point(&mut grid, i, j);
                }
            }
        }

        ans
    }

    fn check_point(grid: &mut [Vec<char>], i: usize, j: usize) {
        if i >= grid.len() || j >= grid[i].len() || grid[i][j] != '1' {
            return;
        }

        grid[i][j] = '\0';
        if i > 0 {
            Self::check_point(grid, i - 1, j);
        }
        Self::check_point(grid, i + 1, j);
        if j > 0 {
            Self::check_point(grid, i, j - 1);
        }
        Self::check_point(grid, i, j + 1);
    }

    pub fn solve(board: &mut Vec<Vec<char>>) {
        let n = board.len();
        let m = board[0].len();
        for j in 0..m {
            if board[0][j] == 'O' {
                Self::update_rim(board, 0, j);
            }
            if board[n - 1][j] == 'O' {
                Self::update_rim(board, n - 1, j);
            }
        }
        for i in 1..n - 1 {
            if board[i][0] == 'O' {
                Self::update_rim(board, i, 0);
            }
            if board[i][board[i].len() - 1] == 'O' {
                Self::update_rim(board, i, m - 1);
            }
        }
        for row in board.iter_mut() {
            for e in row.iter_mut() {
                if *e == 'O' {
                    *e = 'X';
                }
                if *e == 'F' {
                    *e = 'O'
                }
            }
        }
    }

    fn update_rim(board: &mut [Vec<char>], i: usize, j: usize) {
        if i >= board.len() || j >= board[i].len() || board[i][j] != 'O' {
            return;
        }

        board[i][j] = 'F';

        if i > 0 {
            Self::update_rim(board, i - 1, j);
        }
        Self::update_rim(board, i + 1, j);
        if j > 0 {
            Self::update_rim(board, i, j - 1);
        }
        Self::update_rim(board, i, j + 1);
    }

    pub fn largest_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let n = grid.len();
        let m = grid[0].len();
        let mut id = 2;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    Self::partition(&mut grid, i, j, id);
                    id += 1;
                }
            }
        }

        let mut statis = vec![0; id as usize];
        for row in &grid {
            for &e in row {
                if e != 0 {
                    statis[e as usize] += 1;
                }
            }
        }

        let mut ans = *statis.iter().max().unwrap();
        let mut dup = vec![false; id as usize];
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 0 {
                    let mut cur_max = 1;
                    if i > 0 && !dup[grid[i - 1][j] as usize] {
                        cur_max += statis[grid[i - 1][j] as usize];
                        dup[grid[i - 1][j] as usize] = true;
                    }
                    if i < n - 1 && !dup[grid[i + 1][j] as usize] {
                        cur_max += statis[grid[i + 1][j] as usize];
                        dup[grid[i + 1][j] as usize] = true;
                    }
                    if j > 0 && !dup[grid[i][j - 1] as usize] {
                        cur_max += statis[grid[i][j - 1] as usize];
                        dup[grid[i][j - 1] as usize] = true;
                    }
                    if j < m - 1 && !dup[grid[i][j + 1] as usize] {
                        cur_max += statis[grid[i][j + 1] as usize];
                        dup[grid[i][j + 1] as usize] = true;
                    }
                    if i > 0 {
                        dup[grid[i - 1][j] as usize] = false;
                    }
                    if i < n - 1 {
                        dup[grid[i + 1][j] as usize] = false;
                    }
                    if j > 0 {
                        dup[grid[i][j - 1] as usize] = false;
                    }
                    if j < m - 1 {
                        dup[grid[i][j + 1] as usize] = false;
                    }
                    ans = ans.max(cur_max);
                }
            }
        }

        ans
    }

    fn partition(grid: &mut [Vec<i32>], i: usize, j: usize, id: i32) {
        if i >= grid.len() || j >= grid[i].len() || grid[i][j] != 1 {
            return;
        }

        grid[i][j] = id;

        if i > 0 {
            Self::partition(grid, i - 1, j, id);
        }
        Self::partition(grid, i + 1, j, id);
        if j > 0 {
            Self::partition(grid, i, j - 1, id);
        }
        Self::partition(grid, i, j + 1, id);
    }

    pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = vec![0; hits.len()];
        let n = grid.len();
        let m = grid[0].len();
        if n == 1 {
            return ans;
        }

        let mut grid = grid;
        for hit in &hits {
            let i = hit[0] as usize;
            let j = hit[1] as usize;
            // 打击砖块消失
            grid[i][j] -= 1;
        }

        for j in 0..m {
            let _ = Self::stable(&mut grid, 0, j);
        }

        let mut cur_idx = hits.len() - 1;
        for hit in hits.iter().rev() {
            let i = hit[0] as usize;
            let j = hit[1] as usize;
            grid[i][j] += 1;
            if grid[i][j] == 1
                && (i == 0
                    || (i > 0 && grid[i - 1][j] == 2)
                    || (i < n - 1 && grid[i + 1][j] == 2)
                    || (j > 0 && grid[i][j - 1] == 2)
                    || (j < m - 1 && grid[i][j + 1] == 2))
            {
                ans[cur_idx] = Self::stable(&mut grid, i, j) - 1;
            }
            cur_idx -= 1;
        }

        ans
    }

    fn stable(grid: &mut [Vec<i32>], i: usize, j: usize) -> i32 {
        if i >= grid.len() || j >= grid[i].len() || grid[i][j] != 1 {
            return 0;
        }

        grid[i][j] = 2;

        let mut ans = 1;
        if i > 0 {
            ans += Self::stable(grid, i - 1, j);
        }
        ans += Self::stable(grid, i + 1, j);

        if j > 0 {
            ans += Self::stable(grid, i, j - 1);
        }
        ans += Self::stable(grid, i, j + 1);

        ans
    }
}
