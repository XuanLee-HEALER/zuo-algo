fn main() {
    println!("couple {}", Solution::min_swaps_couples(vec![0, 2, 1, 3]));
    println!(
        "similar {}",
        Solution::num_similar_groups(vec![
            "tars".into(),
            "rats".into(),
            "arts".into(),
            "star".into()
        ])
    );
    println!(
        "island: {}",
        Solution::num_islands(vec![vec!['1'], vec!['1']])
    );
}

struct Solution;

impl Solution {
    pub fn min_swaps_couples(row: Vec<i32>) -> i32 {
        let group = row.len() / 2;
        let mut union_find = UnionFind::new(group);

        for v in row.windows(2).step_by(2) {
            let g1 = v[0] as usize / 2;
            let g2 = v[1] as usize / 2;
            if g1 != g2 {
                union_find.union(g1, g2);
            }
        }

        (group - union_find.sets) as i32
    }

    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let mut union_find = UnionFind::new(strs.len());
        let similar = |s1: &str, s2: &str| -> bool {
            let mut diff = 0;
            let s1 = s1.chars();
            let mut s2 = s2.chars();
            for c in s1 {
                if s2.next().unwrap() != c {
                    diff += 1;
                    if diff > 2 {
                        break;
                    }
                }
            }
            diff <= 2
        };
        for i in 0..strs.len() {
            for j in i + 1..strs.len() {
                if similar(&strs[i], &strs[j]) {
                    union_find.union(i, j);
                }
            }
        }

        union_find.sets as i32
    }

    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        // 全部集合的个数
        let grid_num = grid.len() * grid[0].len();
        let mut union_find = UnionFind::new(grid_num);
        let idx = |i: usize, j: usize| -> usize { i * grid[0].len() + j };
        // 全部1的个数
        let mut set_num = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '1' {
                    set_num += 1;
                    if i > 0 && grid[i - 1][j] == '1' {
                        union_find.union(idx(i, j), idx(i - 1, j));
                    }
                    if j > 0 && grid[i][j - 1] == '1' {
                        union_find.union(idx(i, j), idx(i, j - 1));
                    }
                }
            }
        }
        // 展开括号会溢出！！！
        (union_find.sets - (grid_num - set_num)) as i32
    }
}

struct UnionFind {
    father: Vec<usize>,
    sets: usize,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        let union_find = (0..size).collect();

        Self {
            father: union_find,
            sets: size,
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.father[v] != v {
            self.father[v] = self.find(self.father[v]);
        }
        self.father[v]
    }

    fn union(&mut self, v1: usize, v2: usize) {
        let b_v1 = self.find(v1);
        let b_v2 = self.find(v2);
        if b_v1 != b_v2 {
            self.father[b_v1] = b_v2;
            self.sets -= 1;
        }
    }
}
