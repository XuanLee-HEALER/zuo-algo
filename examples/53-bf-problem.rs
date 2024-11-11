fn main() {
    println!(
        "cheapest {}",
        Solution::find_cheapest_price(
            7,
            vec![
                vec![0, 3, 7],
                vec![4, 5, 3],
                vec![6, 4, 8],
                vec![2, 0, 10],
                vec![6, 5, 6],
                vec![1, 2, 2],
                vec![2, 5, 9],
                vec![2, 6, 8],
                vec![3, 6, 3],
                vec![4, 0, 10],
                vec![4, 6, 8],
                vec![5, 2, 6],
                vec![1, 4, 3],
                vec![4, 1, 6],
                vec![0, 5, 10],
                vec![3, 1, 5],
                vec![4, 3, 1],
                vec![5, 4, 10],
                vec![0, 1, 6]
            ],
            2,
            4,
            1
        )
    )
}

struct Solution;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
        let n = n as usize;
        let src = src as usize;
        let dst = dst as usize;
        let mut k = k;
        let mut distances = vec![i32::MAX; n + 1];
        distances[src] = 0;
        let mut cur_dis = Vec::with_capacity(distances.len());
        cur_dis.clone_from(&distances);
        while k >= 0 {
            for flight in &flights {
                let (b, e, w) = (flight[0] as usize, flight[1] as usize, flight[2]);
                if distances[b] != i32::MAX && distances[b] + w < cur_dis[e] {
                    cur_dis[e] = distances[b] + w;
                }
            }
            distances.copy_from_slice(&cur_dis);
            k -= 1;
        }

        if distances[dst] == i32::MAX {
            -1
        } else {
            distances[dst]
        }
    }
}
