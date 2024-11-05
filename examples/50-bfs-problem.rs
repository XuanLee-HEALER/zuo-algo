use std::collections::HashSet;

fn main() {
    println!(
        "sticker {}",
        Solution::min_stickers(
            vec![
                "these".into(),
                "guess".into(),
                "about".into(),
                "garden".into(),
                "him".into()
            ],
            "atomher".into()
        )
    );
    println!(
        "01bfs {}",
        Solution::minimum_obstacles(vec![vec![0, 0], vec![0, 1], vec![1, 1], vec![1, 0]])
    );
    println!(
        "01bfs-1 {}",
        Solution::min_cost(vec![
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2]
        ])
    )
}

struct Solution;

impl Solution {
    pub fn max_distance(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid.len();

        let mut mark = vec![vec![false; m]; n];

        let mut q = vec![(0, 0); n * m];
        let mut l = 0;
        let mut r = 0;

        let mut seas = 0;
        for (i, sub) in grid.iter().enumerate() {
            for (j, &v) in sub.iter().enumerate() {
                if v == 1 {
                    q[r] = (i, j);
                    mark[i][j] = true;
                    r += 1;
                } else {
                    seas += 1;
                }
            }
        }

        if seas == 0 || seas == n * m {
            return -1;
        }

        let mut level = 0;
        while r > l {
            level += 1;
            let cur = r - l;
            for _ in 0..cur {
                let p = q[l];
                l += 1;
                if p.0 > 0 && grid[p.0 - 1][p.1] == 0 && !mark[p.0 - 1][p.1] {
                    q[r] = (p.0 - 1, p.1);
                    mark[p.0 - 1][p.1] = true;
                    r += 1;
                }
                if p.0 + 1 < n && grid[p.0 + 1][p.1] == 0 && !mark[p.0 + 1][p.1] {
                    q[r] = (p.0 + 1, p.1);
                    mark[p.0 + 1][p.1] = true;
                    r += 1;
                }
                if p.1 > 0 && grid[p.0][p.1 - 1] == 0 && !mark[p.0][p.1 - 1] {
                    q[r] = (p.0, p.1 - 1);
                    mark[p.0][p.1 - 1] = true;
                    r += 1;
                }
                if p.1 + 1 < m && grid[p.0][p.1 + 1] == 0 && !mark[p.0][p.1 + 1] {
                    q[r] = (p.0, p.1 + 1);
                    mark[p.0][p.1 + 1] = true;
                    r += 1;
                }
            }
        }

        level - 1
    }

    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let mut target = target.chars().collect::<Vec<char>>();
        target.sort();
        let stickers = stickers
            .iter()
            .map(|s| {
                let mut cs = s.chars().collect::<Vec<char>>();
                cs.sort();
                cs
            })
            .collect::<Vec<Vec<char>>>();
        let mut graph = vec![vec![]; 26];
        let c_idx = |c: &char| -> usize { (*c as u8 - b'a') as usize };
        for sticker in &stickers {
            let mut last = '\0';
            for c in sticker {
                if last == '\0' || *c != last {
                    graph[c_idx(c)].push(sticker.clone());
                    last = *c;
                }
            }
        }

        let mut visited = HashSet::new();
        let mut q = vec![vec![]; 401];
        let mut l = 0;
        let mut r = 0;
        let merge_chars = |t: &[char], s: &[char]| -> Vec<char> {
            let mut res = vec![];
            let mut i = 0;
            let mut j = 0;
            while i < t.len() && j < s.len() {
                match t[i].cmp(&s[j]) {
                    std::cmp::Ordering::Greater => {
                        j += 1;
                    }
                    std::cmp::Ordering::Less => {
                        res.push(t[i]);
                        i += 1;
                    }
                    std::cmp::Ordering::Equal => {
                        i += 1;
                        j += 1;
                    }
                }
            }
            while i < t.len() {
                res.push(t[i]);
                i += 1;
            }
            res
        };

        q[r] = target.clone();
        r += 1;
        visited.insert(target.clone());
        let mut level = 0;
        while r > l {
            let size = r - l;
            for _ in 0..size {
                let t = q[l].clone();
                l += 1;
                let c = t[0];
                for sticker in &graph[c_idx(&c)] {
                    let merge_str = merge_chars(&t, sticker);
                    if merge_str.is_empty() {
                        return level + 1;
                    } else if !visited.contains(&merge_str) {
                        visited.insert(merge_str.clone());
                        q[r] = merge_str;
                        r += 1;
                    }
                }
            }
            level += 1;
        }

        -1
    }

    pub fn minimum_obstacles(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let q_cap = 2 * n * m + 1;
        let mut deque = vec![(0, 0); q_cap];
        let mut head = 0;
        let mut tail = q_cap - 1;
        let mut q_size = 0;
        let mut distance = vec![vec![i32::MAX; m]; n];
        distance[0][0] = 0;
        head = (head + q_cap - 1) % q_cap;
        deque[head] = (0, 0);
        q_size += 1;
        let mut ans = 0;
        while q_size > 0 {
            let cur_idx = deque[head];
            head = (head + 1) % q_cap;
            q_size -= 1;
            let x = cur_idx.0;
            let y = cur_idx.1;
            if cur_idx == (n - 1, m - 1) {
                ans = distance[x][y];
                break;
            } else {
                let d = distance[x][y];
                if x > 0 && d + grid[x - 1][y] < distance[x - 1][y] {
                    distance[x - 1][y] = d + grid[x - 1][y];
                    if grid[x - 1][y] == 0 {
                        head = (head + q_cap - 1) % q_cap;
                        deque[head] = (x - 1, y);
                    } else {
                        tail = (tail + 1) % q_cap;
                        deque[tail] = (x - 1, y);
                    }
                    q_size += 1;
                }
                if x + 1 < n && d + grid[x + 1][y] < distance[x + 1][y] {
                    distance[x + 1][y] = d + grid[x + 1][y];
                    if grid[x + 1][y] == 0 {
                        head = (head + q_cap - 1) % q_cap;
                        deque[head] = (x + 1, y);
                    } else {
                        tail = (tail + 1) % q_cap;
                        deque[tail] = (x + 1, y);
                    }
                    q_size += 1;
                }
                if y > 0 && d + grid[x][y - 1] < distance[x][y - 1] {
                    distance[x][y - 1] = d + grid[x][y - 1];
                    if grid[x][y - 1] == 0 {
                        head = (head + q_cap - 1) % q_cap;
                        deque[head] = (x, y - 1);
                    } else {
                        tail = (tail + 1) % q_cap;
                        deque[tail] = (x, y - 1);
                    }
                    q_size += 1;
                }
                if y + 1 < m && d + grid[x][y + 1] < distance[x][y + 1] {
                    distance[x][y + 1] = d + grid[x][y + 1];
                    if grid[x][y + 1] == 0 {
                        head = (head + q_cap - 1) % q_cap;
                        deque[head] = (x, y + 1);
                    } else {
                        tail = (tail + 1) % q_cap;
                        deque[tail] = (x, y + 1);
                    }
                    q_size += 1;
                }
            }
        }

        ans
    }

    // grid 1 -> 2 <- 3 down 4 up
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let q_cap = n * m * 2 + 1;
        let mut deque = vec![(0, 0); q_cap];
        let mut head = 0;
        let mut tail = q_cap - 1;
        let mut q_size = 0;
        let mut distance = vec![vec![i32::MAX; m]; n];

        distance[0][0] = 0;
        head = (head + q_cap - 1) % q_cap;
        deque[head] = (0, 0);
        q_size += 1;
        let mut ans = 0;
        while q_size > 0 {
            let cur_idx = deque[head];
            head = (head + 1) % q_cap;
            q_size -= 1;
            let x = cur_idx.0;
            let y = cur_idx.1;
            let oriental = grid[x][y];
            let d = distance[x][y];
            if x == n - 1 && y == m - 1 {
                ans = distance[x][y];
                break;
            }

            if x > 0 {
                let w = if oriental == 4 { 0 } else { 1 };
                if d + w < distance[x - 1][y] {
                    distance[x - 1][y] = d + w;
                    if w == 0 {
                        head = (head + q_cap - 1) % q_cap;
                        deque[head] = (x - 1, y);
                        q_size += 1;
                    } else {
                        tail = (tail + 1) % q_cap;
                        deque[tail] = (x - 1, y);
                        q_size += 1;
                    }
                }
            }

            if x + 1 < n {
                let w = if oriental == 3 { 0 } else { 1 };
                if d + w < distance[x + 1][y] {
                    distance[x + 1][y] = d + w;
                    if w == 0 {
                        head = (head + q_cap - 1) % q_cap;
                        deque[head] = (x + 1, y);
                        q_size += 1;
                    } else {
                        tail = (tail + 1) % q_cap;
                        deque[tail] = (x + 1, y);
                        q_size += 1;
                    }
                }
            }

            if y > 0 {
                let w = if oriental == 2 { 0 } else { 1 };
                if d + w < distance[x][y - 1] {
                    distance[x][y - 1] = d + w;
                    if w == 0 {
                        head = (head + q_cap - 1) % q_cap;
                        deque[head] = (x, y - 1);
                        q_size += 1;
                    } else {
                        tail = (tail + 1) % q_cap;
                        deque[tail] = (x, y - 1);
                        q_size += 1;
                    }
                }
            }

            if y + 1 < m {
                let w = if oriental == 1 { 0 } else { 1 };
                if d + w < distance[x][y + 1] {
                    distance[x][y + 1] = d + w;
                    if w == 0 {
                        head = (head + q_cap - 1) % q_cap;
                        deque[head] = (x, y + 1);
                        q_size += 1;
                    } else {
                        tail = (tail + 1) % q_cap;
                        deque[tail] = (x, y + 1);
                        q_size += 1;
                    }
                }
            }
        }

        ans
    }
}
