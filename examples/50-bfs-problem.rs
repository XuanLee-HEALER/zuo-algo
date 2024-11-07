use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
};

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
    );
    for ans in Solution::find_ladders(
        "hit".into(),
        "cog".into(),
        vec![
            "hot".into(),
            "dot".into(),
            "dog".into(),
            "lot".into(),
            "log".into(),
            "cog".into(),
        ],
    ) {
        println!("=> {:?}", ans);
    }
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

    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let n = height_map.len();
        let m = height_map[0].len();
        let mut min_heap = MinHeap::new(n * m);
        let mut visited = vec![vec![false; m]; n];
        for (i, sub) in height_map.iter().enumerate() {
            for (j, &e) in sub.iter().enumerate() {
                if i == 0 || i == n - 1 || j == 0 || j == m - 1 {
                    min_heap.push(WaterLine(i, j, e));
                    visited[i][j] = true;
                }
            }
        }

        let mut ans = 0;

        while !min_heap.is_empty() {
            let WaterLine(i, j, w) = min_heap.pop();
            ans += w - height_map[i][j];
            if i > 0 && !visited[i - 1][j] {
                visited[i - 1][j] = true;
                min_heap.push(WaterLine(i - 1, j, w.max(height_map[i - 1][j])));
            }
            if i + 1 < n && !visited[i + 1][j] {
                visited[i + 1][j] = true;
                min_heap.push(WaterLine(i + 1, j, w.max(height_map[i + 1][j])));
            }
            if j > 0 && !visited[i][j - 1] {
                visited[i][j - 1] = true;
                min_heap.push(WaterLine(i, j - 1, w.max(height_map[i][j - 1])));
            }
            if j + 1 < m && !visited[i][j + 1] {
                visited[i][j + 1] = true;
                min_heap.push(WaterLine(i, j + 1, w.max(height_map[i][j + 1])));
            }
        }

        ans
    }

    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut base = word_list.iter().collect::<HashSet<&String>>();
        if !base.contains(&end_word) {
            return vec![];
        }
        let _ = base.remove(&begin_word);

        let p_words = |word: &str| -> Vec<String> {
            let mut ans = Vec::new();
            let mut cs = word.chars().collect::<Vec<char>>();
            for (i, c) in word.char_indices() {
                let old_c = c;
                for j in 0..=25_u8 {
                    let rc = (b'a' + j) as char;
                    if rc == old_c {
                        continue;
                    }
                    cs[i] = rc;
                    ans.push(cs.iter().collect::<String>());
                }
                cs[i] = old_c;
            }
            ans
        };

        let mut cur_level = HashSet::new();
        let mut next_level = HashSet::new();
        let mut graph = HashMap::new();

        cur_level.insert(begin_word.clone());
        let mut is_find = false;
        while !cur_level.is_empty() {
            // 每次遍历过的单词不再参与后续遍历
            base.retain(|&e| !cur_level.contains(e));
            for word in cur_level.drain() {
                let next_words = p_words(&word);
                for nw in next_words {
                    if base.contains(&nw) {
                        if nw == end_word {
                            is_find = true;
                        }
                        graph
                            .entry(nw.clone())
                            .and_modify(|e: &mut Vec<String>| {
                                e.push(word.clone());
                            })
                            .or_insert(vec![word.clone()]);
                        next_level.insert(nw);
                    }
                }
            }

            if is_find {
                break;
            } else {
                cur_level = next_level.clone();
                next_level.clear();
            }
        }

        let mut ans = Vec::new();
        let mut buf = VecDeque::new();
        if is_find {
            Self::collect_path(&graph, &begin_word, &end_word, &mut buf, &mut ans);
        }

        ans
    }

    fn collect_path(
        rel: &HashMap<String, Vec<String>>,
        begin_word: &str,
        end_word: &str,
        buf: &mut VecDeque<String>,
        ans: &mut Vec<Vec<String>>,
    ) {
        buf.push_front(end_word.to_string());
        if begin_word == end_word {
            ans.push(buf.iter().map(|s| s.to_string()).collect());
        } else {
            for nw in rel[end_word].iter() {
                Self::collect_path(rel, begin_word, nw, buf, ans);
            }
        }
        buf.pop_front();
    }
}

#[derive(Clone, Copy, Default)]
struct WaterLine(usize, usize, i32);
impl Ord for WaterLine {
    fn cmp(&self, other: &Self) -> Ordering {
        self.2.cmp(&other.2)
    }
}

impl PartialOrd for WaterLine {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for WaterLine {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
    }
}

impl Eq for WaterLine {}

struct MinHeap<T: Ord>
where
    T: Ord,
{
    heap: Vec<T>,
    size: usize,
}

impl<T> MinHeap<T>
where
    T: Copy + Clone + Ord + Default,
{
    fn new(n: usize) -> Self {
        Self {
            heap: vec![T::default(); n],
            size: 0,
        }
    }

    fn push(&mut self, v: T) {
        self.heap_insert(v);
    }

    fn pop(&mut self) -> T {
        let res = self.heap[0];
        self.heap.swap(0, self.size - 1);
        self.size -= 1;
        self.heapify(0);
        res
    }

    fn heapify(&mut self, idx: usize) {
        let mut idx = idx;
        while idx < self.size {
            let l_idx = (idx << 1) + 1;
            if l_idx < self.size && l_idx + 1 < self.size {
                let a_idx = if self.heap[l_idx].cmp(&self.heap[l_idx + 1]) == Ordering::Less {
                    l_idx
                } else {
                    l_idx + 1
                };
                if self.heap[idx].cmp(&self.heap[a_idx]) == Ordering::Greater {
                    self.heap.swap(idx, a_idx);
                    idx = a_idx;
                } else {
                    break;
                }
            } else if l_idx < self.size {
                if self.heap[idx].cmp(&self.heap[l_idx]) == Ordering::Greater {
                    self.heap.swap(idx, l_idx);
                    idx = l_idx
                } else {
                    break;
                };
            } else {
                break;
            }
        }
    }

    fn heap_insert(&mut self, v: T) {
        self.heap[self.size] = v;
        self.size += 1;
        let mut old_idx = self.size - 1;
        while old_idx > 0 {
            let p_idx = (old_idx - 1) >> 1;
            if self.heap[old_idx].cmp(&self.heap[p_idx]) == Ordering::Less {
                self.heap.swap(old_idx, p_idx);
                old_idx = p_idx;
            } else {
                break;
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }
}
