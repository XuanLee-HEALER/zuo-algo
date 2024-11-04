fn main() {
    println!(
        "{:?}",
        Solution::distance_limited_paths_exist(
            3,
            vec![vec![0, 1, 2], vec![1, 2, 4], vec![2, 0, 8], vec![1, 0, 16]],
            vec![vec![0, 1, 2], vec![0, 2, 5]]
        )
    )
}

struct Solution;

impl Solution {
    fn min_cost_to_supply_water(n: i32, wells: Vec<i32>, pipes: Vec<Vec<i32>>) -> i32 {
        let mut union_find = (0..=n).collect::<Vec<i32>>();
        let mut ans = 0;
        let mut edge_num = 0;
        let mut edges = Vec::with_capacity(40_001);
        for (i, &e) in wells.iter().enumerate() {
            edges.push((0, i + 1, e));
            edge_num += 1;
        }

        for pipe in &pipes {
            let p1 = pipe[0];
            let p2 = pipe[1];
            let w = pipe[2];
            edges.push((p1 as usize, p2 as usize, w));
            edge_num += 1;
        }

        edges[..edge_num].sort_unstable_by_key(|e| e.2);

        for &(p1, p2, w) in &edges[..edge_num] {
            if union(&mut union_find, p1 as i32, p2 as i32) {
                ans += w;
            }
        }

        ans
    }

    pub fn distance_limited_paths_exist(
        n: i32,
        edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut union_find = (0..n).collect::<Vec<i32>>();
        let mut ans = vec![false; queries.len()];
        let mut queries = queries
            .iter()
            .enumerate()
            .map(|(i, v)| (v[0], v[1], v[2], i))
            .collect::<Vec<(i32, i32, i32, usize)>>();
        queries.sort_unstable_by_key(|e| e.2);
        let mut edges = edge_list;
        edges.sort_unstable_by_key(|e| e[2]);
        let mut j = 0;

        for query in &queries {
            while j < edges.len() {
                let p1 = edges[j][0];
                let p2 = edges[j][1];
                let w = edges[j][2];
                if w < query.2 {
                    let _ = union(&mut union_find, p1, p2);
                    j += 1;
                } else {
                    break;
                }
            }
            ans[query.3] = same_set(&mut union_find, query.0, query.1);
        }

        ans
    }
}

fn find(arr: &mut [i32], v: i32) -> i32 {
    if arr[v as usize] != v {
        arr[v as usize] = find(arr, arr[v as usize]);
    }
    arr[v as usize]
}

fn same_set(arr: &mut [i32], v1: i32, v2: i32) -> bool {
    find(arr, v1) == find(arr, v2)
}

fn union(arr: &mut [i32], v1: i32, v2: i32) -> bool {
    let b_v1 = find(arr, v1);
    let b_v2 = find(arr, v2);
    if b_v1 == b_v2 {
        false
    } else {
        arr[b_v1 as usize] = b_v2;
        true
    }
}
