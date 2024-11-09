use std::collections::HashMap;

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
    println!(
        "expert {:?}",
        Solution::find_all_people(5, vec![vec![1, 4, 3], vec![0, 4, 3]], 3)
    );
    println!(
        "good path {}",
        Solution::number_of_good_paths(
            vec![2, 4, 1, 2, 2, 5, 3, 4, 4],
            vec![
                vec![0, 1],
                vec![2, 1],
                vec![0, 3],
                vec![4, 1],
                vec![4, 5],
                vec![3, 6],
                vec![7, 5],
                vec![2, 8]
            ]
        )
    );
    println!(
        "malware {}",
        Solution::min_malware_spread(
            vec![
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 1, 0, 0, 0, 0, 0, 0, 1],
                vec![0, 0, 1, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 1, 0, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 1, 0, 1, 1, 1],
                vec![0, 0, 0, 0, 0, 1, 0, 0, 1],
                vec![0, 0, 0, 0, 1, 0, 1, 1, 0],
                vec![0, 0, 0, 0, 1, 0, 1, 1, 0],
                vec![0, 1, 0, 1, 1, 1, 0, 0, 1]
            ],
            vec![8, 4, 2, 0]
        )
    );
    println!(
        "swim {}",
        Solution::swim_in_water(vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6]
        ])
    )
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

    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut x_ = HashMap::new();
        let mut y_ = HashMap::new();
        let mut union_find = UnionFind::new(stones.len());
        for (idx, stone) in stones.iter().enumerate() {
            let x = stone[0];
            let y = stone[1];
            if let std::collections::hash_map::Entry::Vacant(e) = x_.entry(x) {
                e.insert(idx);
            } else {
                union_find.union(idx, *x_.get(&x).unwrap());
            }
            if let std::collections::hash_map::Entry::Vacant(e) = y_.entry(y) {
                e.insert(idx);
            } else {
                union_find.union(idx, *y_.get(&y).unwrap());
            }
        }

        (stones.len() - union_find.sets) as i32
    }

    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        let mut meetings = meetings;
        let mut union_find = UnionFind::new(n as usize);
        let mut secrets = vec![false; n as usize];
        secrets[0] = true;
        union_find.union_with_secrets(0, first_person as usize, &mut secrets);
        meetings.sort_unstable_by(|a, b| a[2].cmp(&b[2]));

        let m_l = meetings.len();
        let mut l = 0;
        let mut r = 1;
        while l < m_l {
            while r < m_l && meetings[r][2] == meetings[r - 1][2] {
                r += 1;
            }

            for meeting in &meetings[l..r] {
                union_find.union_with_secrets(
                    meeting[0] as usize,
                    meeting[1] as usize,
                    &mut secrets,
                );
            }

            for meeting in &meetings[l..r] {
                if !secrets[union_find.find(meeting[0] as usize)] {
                    union_find.father[meeting[0] as usize] = meeting[0] as usize;
                }
                if !secrets[union_find.find(meeting[1] as usize)] {
                    union_find.father[meeting[1] as usize] = meeting[1] as usize;
                }
            }

            l = r;
            r += 1;
        }

        (0..n)
            .filter(|&i| secrets[union_find.find(i as usize)])
            .collect::<Vec<i32>>()
    }

    pub fn number_of_good_paths(vals: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let mut edges = edges;
        let mut union_find = UnionFind::new(vals.len());
        let mut max_cnt = vec![1; vals.len()];
        edges.sort_unstable_by(|a, b| {
            vals[a[0] as usize]
                .max(vals[a[1] as usize])
                .cmp(&vals[b[0] as usize].max(vals[b[1] as usize]))
        });
        let mut ans = 0;
        for edge in &edges {
            ans += union_find.union_with_max_count(
                edge[0] as usize,
                edge[1] as usize,
                &mut max_cnt,
                &vals,
            );
        }
        ans + vals.len() as i32
    }

    pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
        let pn = graph.len();
        let mut initial = initial;
        let mut virus = vec![false; pn];
        initial.iter().for_each(|&i| virus[i as usize] = true);
        let mut size = vec![1; pn];
        let mut infect = vec![-1; pn];
        let mut cnts = vec![0; pn];
        let mut union_find = UnionFind::new(pn);
        for node in 0..pn {
            if !virus[node] {
                for (i, &link) in graph[node].iter().enumerate() {
                    if !virus[i] && node != i && link == 1 {
                        union_find.union_with_size(node, i, &mut size);
                    }
                }
            }
        }

        for &vir in &initial {
            let vir = vir as usize;
            for (node, &link) in graph[vir].iter().enumerate() {
                if vir != node && link == 1 {
                    let behalf = union_find.find(node);
                    match infect[behalf] {
                        -1 => infect[behalf] = vir as i32,
                        -2 => continue,
                        o if o == vir as i32 => continue,
                        _ => infect[behalf] = -2,
                    }
                }
            }
        }

        for i in 0..pn {
            if !virus[i] && i == union_find.find(i) && infect[i] != -1 && infect[i] != -2 {
                // 每个感染病毒应该累积拯救的普通节点的数量
                cnts[infect[i] as usize] += size[i]
            }
        }

        initial.sort_unstable();
        let mut ans = initial[0];
        let mut max = cnts[ans as usize];
        for &vir in initial.iter().skip(1) {
            if cnts[vir as usize] > max {
                ans = vir;
                max = cnts[vir as usize];
            }
        }

        ans
    }

    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut union_find = (0..n * n).collect::<Vec<usize>>();
        let mut l = grid[0][0] as usize;
        let mut r = n * n - 1;
        let mut ans = -1;
        while l <= r {
            let m = l + ((r - l) >> 1);
            Self::union_all(&grid, n, &mut union_find, m as i32);
            if Self::same_set(&mut union_find, 0, n * n - 1) {
                ans = m as i32;
                if m == 0 {
                    break;
                }
                r = m - 1;
            } else {
                l = m + 1;
            }
            union_find = (0..n * n).collect::<Vec<usize>>();
        }

        ans
    }

    fn union_all(grid: &[Vec<i32>], n: usize, union_find: &mut [usize], s: i32) {
        Self::try_union(grid, n, union_find, 0, 0, s);
    }

    fn try_union(
        grid: &[Vec<i32>],
        n: usize,
        union_find: &mut [usize],
        i: usize,
        j: usize,
        s: i32,
    ) {
        let uf_idx = i * n + j;
        if uf_idx == 0 || (grid[i][j] <= s && !Self::same_set(union_find, 0, uf_idx)) {
            Self::union(union_find, 0, uf_idx);
            if i > 0 {
                Self::try_union(grid, n, union_find, i - 1, j, s);
            }
            if i + 1 < n {
                Self::try_union(grid, n, union_find, i + 1, j, s);
            }
            if j > 0 {
                Self::try_union(grid, n, union_find, i, j - 1, s);
            }
            if j + 1 < n {
                Self::try_union(grid, n, union_find, i, j + 1, s);
            }
        }
    }

    fn same_set(union_find: &mut [usize], v1: usize, v2: usize) -> bool {
        Self::find(union_find, v1) == Self::find(union_find, v2)
    }

    fn find(union_find: &mut [usize], v1: usize) -> usize {
        if union_find[v1] != v1 {
            union_find[v1] = Self::find(union_find, union_find[v1]);
        }
        union_find[v1]
    }

    fn union(union_find: &mut [usize], v1: usize, v2: usize) {
        let b_v1 = Self::find(union_find, v1);
        let b_v2 = Self::find(union_find, v2);
        if b_v1 != b_v2 {
            union_find[b_v1] = b_v2;
        }
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

    fn union_with_secrets(&mut self, v1: usize, v2: usize, secrets: &mut [bool]) {
        let b_v1 = self.find(v1);
        let b_v2 = self.find(v2);
        if b_v1 != b_v2 {
            self.father[b_v1] = b_v2;
            secrets[b_v2] |= secrets[b_v1];
        }
    }

    fn union_with_max_count(
        &mut self,
        v1: usize,
        v2: usize,
        max_cnt: &mut [i32],
        vals: &[i32],
    ) -> i32 {
        let b_v1 = self.find(v1);
        let b_v2 = self.find(v2);
        let mut ans = 0;
        if b_v1 != b_v2 {
            match vals[b_v1].cmp(&vals[b_v2]) {
                std::cmp::Ordering::Greater => self.father[b_v2] = b_v1,
                std::cmp::Ordering::Less => self.father[b_v1] = b_v2,
                std::cmp::Ordering::Equal => {
                    ans = max_cnt[b_v1] * max_cnt[b_v2];
                    self.father[b_v1] = b_v2;
                    max_cnt[b_v2] += max_cnt[b_v1]
                }
            }
        }
        ans
    }

    fn union_with_size(&mut self, v1: usize, v2: usize, size: &mut [i32]) {
        let b_v1 = self.find(v1);
        let b_v2 = self.find(v2);
        if b_v1 != b_v2 {
            self.father[b_v1] = b_v2;
            size[b_v2] += size[b_v1];
        }
    }
}
