use std::{cmp::Reverse, collections::BinaryHeap, time};

use rand::{thread_rng, Rng};

fn main() {
    assert_eq!(4, manhattan(1, 1, 3, 3));
    assert_eq!(2, diagonal(1, 1, 3, 3));
    assert_eq!(5_f64, euclidean(1, 1, 4, 5));
    for _ in 0..50 {
        let grid = build_grid(100);
        let n = grid.len();
        let (bi, bj, ei, ej) = (0, 0, n - 1, n - 1);
        let dj = djikstra(&grid, bi, bj, ei, ej);
        let astar = astar(&grid, bi, bj, ei, ej, diagonal);
        assert_eq!(dj, astar);
    }

    loop {
        let grid = build_grid(2009);
        let n = grid.len();
        let (bi, bj, ei, ej) = (0, 0, n - 1, n - 1);
        let tick = time::Instant::now();
        let dj = djikstra(&grid, bi, bj, ei, ej);
        println!("dijkstra: {} ms", tick.elapsed().as_millis());
        let tick = time::Instant::now();
        let astar = astar(&grid, bi, bj, ei, ej, manhattan);
        println!("a*: {} ms", tick.elapsed().as_millis());
        assert_eq!(dj, astar);
        println!("dj {} a* {}", dj, astar);
        if dj != -1 {
            break;
        }
    }

    // for row in &grid {
    //     for col in row {
    //         print!("{} ", col);
    //     }
    //     println!()
    // }
}

fn djikstra(grid: &[Vec<i32>], i: usize, j: usize, ti: usize, tj: usize) -> i32 {
    let n = grid.len();
    let mut visited = vec![vec![false; n]; n];
    let mut distances = vec![vec![i32::MAX; n]; n];
    let mut min_heap = BinaryHeap::new();

    min_heap.push(Reverse(Point(i, j, 1)));
    distances[i][j] = 1;
    let rotate = [-1, 0, 1, 0, -1];
    while !min_heap.is_empty() {
        let Point(ci, cj, w) = min_heap.pop().unwrap().0;
        if visited[ci][cj] {
            continue;
        }
        if ci == ti && cj == tj {
            return w;
        }
        visited[ci][cj] = true;
        for udlr in rotate.windows(2) {
            let ni = ci as i32 + udlr[0];
            let nj = cj as i32 + udlr[1];
            if ni < 0
                || ni as usize >= n
                || nj < 0
                || nj as usize >= n
                || grid[ni as usize][nj as usize] == 0
                || visited[ni as usize][nj as usize]
            {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if w + grid[ni][nj] < distances[ni][nj] {
                distances[ni][nj] = w + grid[ni][nj];
                min_heap.push(Reverse(Point(ni, nj, distances[ni][nj])));
            }
        }
    }

    -1
}

fn astar<F>(grid: &[Vec<i32>], i: usize, j: usize, ti: usize, tj: usize, predict: F) -> i32
where
    F: Fn(i32, i32, i32, i32) -> i32,
{
    let n = grid.len();
    let mut visited = vec![vec![false; n]; n];
    let mut distances = vec![vec![i32::MAX; n]; n];
    let mut min_heap = BinaryHeap::new();

    min_heap.push(Reverse(Point(i, j, 1)));
    distances[i][j] = 1;
    let rotate = [-1, 0, 1, 0, -1];
    while !min_heap.is_empty() {
        // a*使用距离数组中的结果，而不是堆中的结果
        let Point(ci, cj, _) = min_heap.pop().unwrap().0;
        if visited[ci][cj] {
            continue;
        }
        if ci == ti && cj == tj {
            return distances[ci][cj];
        }
        visited[ci][cj] = true;
        for udlr in rotate.windows(2) {
            let ni = ci as i32 + udlr[0];
            let nj = cj as i32 + udlr[1];
            if ni < 0
                || ni as usize >= n
                || nj < 0
                || nj as usize >= n
                || grid[ni as usize][nj as usize] == 0
                || visited[ni as usize][nj as usize]
            {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if distances[ci][cj] + 1 < distances[ni][nj] {
                distances[ni][nj] = distances[ci][cj] + 1;
                min_heap.push(Reverse(Point(
                    ni,
                    nj,
                    distances[ni][nj] + predict(ni as i32, nj as i32, ti as i32, tj as i32),
                )));
            }
        }
    }

    -1
}

fn manhattan(bi: i32, bj: i32, ei: i32, ej: i32) -> i32 {
    (ei - bi).abs() + (ej - bj).abs()
}

fn diagonal(bi: i32, bj: i32, ei: i32, ej: i32) -> i32 {
    (ei - bi).abs().max((ej - bj).abs())
}

fn euclidean(bi: i32, bj: i32, ei: i32, ej: i32) -> f64 {
    ((ei - bi).abs().pow(2) as f64 + (ej - bj).abs().pow(2) as f64).sqrt()
}

struct Point(usize, usize, i32);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
    }
}

impl Eq for Point {}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.2.cmp(&other.2)
    }
}

fn build_grid(n: usize) -> Vec<Vec<i32>> {
    assert!(n > 0);
    let mut rng = thread_rng();
    let mut ori_grid = vec![vec![0; n]; n];
    for row in ori_grid.iter_mut() {
        for col in row.iter_mut() {
            if rng.gen_ratio(2, 3) {
                *col = 1;
            }
        }
    }
    ori_grid[0][0] = 1;
    ori_grid[n - 1][n - 1] = 1;
    ori_grid
}
