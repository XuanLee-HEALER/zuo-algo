use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    println!(
        "dj {}",
        Solution::network_delay_time_1(
            vec![
                vec![2, 4, 10],
                vec![5, 2, 38],
                vec![3, 4, 33],
                vec![4, 2, 76],
                vec![3, 2, 64],
                vec![1, 5, 54],
                vec![1, 4, 98],
                vec![2, 3, 61],
                vec![2, 1, 0],
                vec![3, 5, 77],
                vec![5, 1, 34],
                vec![3, 1, 79],
                vec![5, 3, 2],
                vec![1, 2, 59],
                vec![4, 3, 46],
                vec![5, 4, 44],
                vec![2, 5, 89],
                vec![4, 5, 21],
                vec![1, 3, 86],
                vec![4, 1, 95]
            ],
            5,
            1
        )
    );

    println!(
        "key {}",
        Solution::shortest_path_all_keys(vec![
            ".#.b.".into(),
            "A.#aB".into(),
            "#d...".into(),
            "@.cC.".into(),
            "D...#".into()
        ])
    );

    println!(
        "car {}",
        Solution::electric_car_plan(
            vec![
                vec![1, 3, 3],
                vec![3, 2, 1],
                vec![2, 1, 3],
                vec![0, 1, 4],
                vec![3, 0, 5]
            ],
            6,
            1,
            0,
            vec![2, 10, 4, 1]
        )
    )
}

struct Solution;

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let mut graph = vec![Vec::new(); n + 1];
        for time in &times {
            let (p1, p2, w) = (time[0] as usize, time[1] as usize, time[2]);
            graph[p1].push((p2, w));
        }
        let mut distances = vec![i32::MAX; n + 1];
        let mut visited = vec![false; n + 1];
        distances[k] = 0;
        let mut min_heap = BinaryHeap::new();
        min_heap.push(Reverse(Target(k, 0)));

        while !min_heap.is_empty() {
            let cur_p = min_heap.pop().unwrap().0;
            if !visited[cur_p.0] {
                visited[cur_p.0] = true;
                for &(p2, w) in &graph[cur_p.0] {
                    if !visited[p2] && distances[cur_p.0] + w < distances[p2] {
                        distances[p2] = distances[cur_p.0] + w;
                        min_heap.push(Reverse(Target(p2, distances[p2])));
                    }
                }
            }
        }

        let max_dis = distances[1..n + 1].iter().max().unwrap();
        if *max_dis == i32::MAX {
            -1
        } else {
            *max_dis
        }
    }

    pub fn network_delay_time_1(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let n = n as usize;
        let k = k as usize;
        let m = times.len();
        let mut head = vec![0; n + 1];
        let mut next = vec![0; m + 1];
        let mut to = vec![0; m + 1];
        let mut weight = vec![0; m + 1];
        let mut cnt = 1;
        for time in &times {
            let (p1, p2, w) = (time[0] as usize, time[1] as usize, time[2]);
            add_edge(
                &mut cnt,
                &mut head,
                &mut next,
                &mut to,
                &mut weight,
                p1,
                p2,
                w,
            );
        }

        let mut min_heap = vec![(0, 0); n];
        let mut distances = vec![i32::MAX; n + 1];
        let mut point_idx = vec![-1; n + 1];
        let mut heap_size = 0;
        add_or_ignore_or_update(&mut min_heap, &mut point_idx, &mut heap_size, k, 0);

        while heap_size > 0 {
            // println!("heap: {:?} {}", min_heap, heap_size);
            let (p1, w) = pop(&mut min_heap, &mut point_idx, &mut heap_size);
            distances[p1] = w;
            let mut next_edge = head[p1];
            while next_edge > 0 {
                add_or_ignore_or_update(
                    &mut min_heap,
                    &mut point_idx,
                    &mut heap_size,
                    to[next_edge],
                    w + weight[next_edge],
                );
                next_edge = next[next_edge];
            }
        }

        let max_dis = distances[1..=n].iter().max().unwrap();
        if *max_dis == i32::MAX {
            -1
        } else {
            *max_dis
        }
    }

    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let n = heights.len();
        let m = heights[0].len();
        let mut distances = vec![vec![0; m]; n];
        let mut visited = vec![vec![false; m]; n];
        let mut min_heap = BinaryHeap::new();

        min_heap.push(Reverse(Target1(0, 0, 0)));
        while !min_heap.is_empty() {
            let Target1(pi, pj, w) = min_heap.pop().unwrap().0;
            if visited[pi][pj] {
                continue;
            }
            visited[pi][pj] = true;
            distances[pi][pj] = w;
            if pi == n - 1 && pj == m - 1 {
                return w;
            }

            if pi > 0 && !visited[pi - 1][pj] {
                let cost = (heights[pi - 1][pj] - heights[pi][pj])
                    .abs()
                    .max(distances[pi][pj]);
                min_heap.push(Reverse(Target1(pi - 1, pj, cost)));
            }
            if pi + 1 < n && !visited[pi + 1][pj] {
                let cost = (heights[pi + 1][pj] - heights[pi][pj])
                    .abs()
                    .max(distances[pi][pj]);
                min_heap.push(Reverse(Target1(pi + 1, pj, cost)));
            }
            if pj > 0 && !visited[pi][pj - 1] {
                let cost = (heights[pi][pj - 1] - heights[pi][pj])
                    .abs()
                    .max(distances[pi][pj]);
                min_heap.push(Reverse(Target1(pi, pj - 1, cost)));
            }
            if pj + 1 < m && !visited[pi][pj + 1] {
                let cost = (heights[pi][pj + 1] - heights[pi][pj])
                    .abs()
                    .max(distances[pi][pj]);
                min_heap.push(Reverse(Target1(pi, pj + 1, cost)));
            }
        }

        -1
    }

    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut distances = vec![vec![i32::MAX; m]; n];
        let mut visited = vec![vec![false; m]; n];
        let mut min_heap = BinaryHeap::new();

        min_heap.push(Reverse(Target1(0, 0, grid[0][0])));
        while !min_heap.is_empty() {
            let Target1(pi, pj, w) = min_heap.pop().unwrap().0;
            if visited[pi][pj] {
                continue;
            }
            visited[pi][pj] = true;
            if pi == n - 1 && pj == m - 1 {
                return w;
            }

            if pi > 0 && !visited[pi - 1][pj] {
                let cost = grid[pi - 1][pj].max(w);
                if cost < distances[pi - 1][pj] {
                    distances[pi - 1][pj] = cost;
                    min_heap.push(Reverse(Target1(pi - 1, pj, cost)));
                }
            }
            if pi + 1 < n && !visited[pi + 1][pj] {
                let cost = grid[pi + 1][pj].max(w);
                if cost < distances[pi + 1][pj] {
                    distances[pi + 1][pj] = cost;
                    min_heap.push(Reverse(Target1(pi + 1, pj, cost)));
                }
            }
            if pj > 0 && !visited[pi][pj - 1] {
                let cost = grid[pi][pj - 1].max(w);
                if cost < distances[pi][pj - 1] {
                    distances[pi][pj - 1] = cost;
                    min_heap.push(Reverse(Target1(pi, pj - 1, cost)));
                }
            }
            if pj + 1 < m && !visited[pi][pj + 1] {
                let cost = grid[pi][pj + 1].max(w);
                if cost < distances[pi][pj + 1] {
                    distances[pi][pj + 1] = cost;
                    min_heap.push(Reverse(Target1(pi, pj + 1, cost)));
                }
            }
        }

        -1
    }

    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let grid = grid
            .into_iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        const MAX_KEY: u8 = 6;
        let n = grid.len();
        let m = grid[0].len();
        let total_p = n * m * (1 << MAX_KEY);
        let mut visited = vec![vec![vec![false; 1 << MAX_KEY]; m]; n];
        let mut q = vec![(0, 0, 0); total_p];
        let mut l = 0;
        let mut r = 0;
        let mut all_key = 0;
        for (i, sub) in grid.iter().enumerate() {
            for (j, &e) in sub.iter().enumerate() {
                if e == '@' {
                    visited[i][j][0] = true;
                    q[r] = (i, j, 0);
                    r += 1;
                }
                if e.is_ascii_lowercase() {
                    all_key |= 1 << (e as u8 - b'a');
                }
            }
        }

        // up right down left
        let udlr = [-1, 0, 1, 0, -1];
        let mut level = 1;
        while r > l {
            for _ in 0..(r - l) {
                let (pi, pj, key) = q[l];
                l += 1;
                let (pi, pj) = (pi as i32, pj as i32);
                for win in udlr.windows(2) {
                    // ！！每个新点的初始key不能影响待遍历的下一个点
                    let mut key = key;
                    let ni = pi + win[0];
                    let nj = pj + win[1];
                    if ni < 0
                        || ni >= n as i32
                        || nj < 0
                        || nj >= m as i32
                        || grid[ni as usize][nj as usize] == '#'
                    {
                        continue;
                    }
                    let ni = ni as usize;
                    let nj = nj as usize;
                    match grid[ni][nj] {
                        c if c.is_ascii_lowercase() => {
                            key |= 1 << (c as u8 - b'a');
                            if key == all_key {
                                return level;
                            }
                        }
                        c if c.is_ascii_uppercase() => {
                            if key & (1 << (c as u8 - b'A')) == 0 {
                                continue;
                            };
                        }
                        _ => (),
                    };
                    if !visited[ni][nj][key] {
                        visited[ni][nj][key] = true;
                        q[r] = (ni, nj, key);
                        r += 1;
                    }
                }
            }
            level += 1;
        }

        -1
    }

    pub fn electric_car_plan(
        paths: Vec<Vec<i32>>,
        cnt: i32,
        start: i32,
        end: i32,
        charge: Vec<i32>,
    ) -> i32 {
        let n = charge.len();
        let cnt = cnt as usize;
        let start = start as usize;
        let end = end as usize;
        let mut graph = vec![vec![]; n];
        for path in &paths {
            let p1 = path[0] as usize;
            let p2 = path[1] as usize;
            let w = path[2];
            graph[p1].push((p2, w));
            graph[p2].push((p1, w));
        }

        let mut visited = vec![vec![false; cnt + 1]; n];
        let mut distances = vec![vec![i32::MAX; cnt + 1]; n];
        let mut min_heap = BinaryHeap::new();
        min_heap.push(Reverse(Target1(start, 0, 0)));

        while !min_heap.is_empty() {
            let Target1(city, battery, cost) = min_heap.pop().unwrap().0;
            if visited[city][battery] {
                continue;
            }
            // 最短花费路径，碰到直接返回
            if city == end {
                return cost;
            }
            visited[city][battery] = true;
            if battery < cnt
                && !visited[city][battery + 1]
                && cost + charge[city] < distances[city][battery + 1]
            {
                distances[city][battery + 1] = cost + charge[city];
                min_heap.push(Reverse(Target1(
                    city,
                    battery + 1,
                    distances[city][battery + 1],
                )));
            }
            for &(nx_city, w) in &graph[city] {
                if battery >= w as usize
                    && !visited[nx_city][battery - w as usize]
                    && cost + w < distances[nx_city][battery - w as usize]
                {
                    distances[nx_city][battery - w as usize] = cost + w;
                    min_heap.push(Reverse(Target1(
                        nx_city,
                        battery - w as usize,
                        distances[nx_city][battery - w as usize],
                    )));
                }
            }
        }

        -1
    }
}

fn add_or_ignore_or_update(
    heap: &mut [(usize, i32)],
    point_idx: &mut [i32],
    heap_size: &mut usize,
    new_point: usize,
    new_weight: i32,
) {
    match point_idx[new_point] {
        -1 => {
            point_idx[new_point] = *heap_size as i32;
            *heap_size += 1;
            heap_insert(
                heap,
                point_idx,
                new_point,
                new_weight,
                point_idx[new_point] as usize,
            );
        }
        o if o >= 0 => {
            if new_weight < heap[o as usize].1 {
                heap_insert(heap, point_idx, new_point, new_weight, o as usize);
            }
        }
        _ => (),
    }
}

fn heap_insert(
    heap: &mut [(usize, i32)],
    point_idx: &mut [i32],
    point: usize,
    weight: i32,
    mut loc: usize,
) {
    heap[loc] = (point, weight);
    while loc > 0 {
        let p_idx = (loc - 1) >> 1;
        if heap[loc].1 < heap[p_idx].1 {
            heap.swap(loc, p_idx);
            point_idx.swap(heap[loc].0, heap[p_idx].0);
            loc = p_idx;
        } else {
            break;
        }
    }
}

fn heapify(heap: &mut [(usize, i32)], point_idx: &mut [i32], heap_size: usize, mut loc: usize) {
    while loc < heap_size {
        let l = (loc << 1) + 1;
        if l < heap_size && l + 1 < heap_size {
            let a = if heap[l].1 < heap[l + 1].1 { l } else { l + 1 };
            if heap[loc].1 > heap[a].1 {
                heap.swap(loc, a);
                point_idx.swap(heap[loc].0, heap[a].0);
                loc = a;
            } else {
                break;
            }
        } else if l < heap_size && heap[loc].1 > heap[l].1 {
            heap.swap(loc, l);
            point_idx.swap(heap[loc].0, heap[l].0);
            loc = l;
        } else {
            break;
        }
    }
}

fn pop(heap: &mut [(usize, i32)], point_idx: &mut [i32], heap_size: &mut usize) -> (usize, i32) {
    let res = heap[0];
    heap.swap(0, *heap_size - 1);
    point_idx.swap(heap[0].0, heap[*heap_size - 1].0);
    // 最后将弹出的元素设置为-2，如果先设置，上一行交换后，留下的元素就成为了-2
    point_idx[res.0] = -2;
    *heap_size -= 1;
    heapify(heap, point_idx, *heap_size, 0);
    res
}

#[allow(clippy::too_many_arguments)]
fn add_edge(
    cnt: &mut usize,
    head: &mut [usize],
    next: &mut [usize],
    to: &mut [usize],
    weight: &mut [i32],
    p1: usize,
    p2: usize,
    w: i32,
) {
    next[*cnt] = head[p1];
    head[p1] = *cnt;
    to[*cnt] = p2;
    weight[*cnt] = w;
    *cnt += 1;
}

struct Target1(usize, usize, i32);

impl Ord for Target1 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.2.cmp(&other.2)
    }
}

impl PartialOrd for Target1 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Target1 {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
    }
}

impl Eq for Target1 {}

struct Target(usize, i32);

impl Ord for Target {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

impl PartialOrd for Target {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Target {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Eq for Target {}
