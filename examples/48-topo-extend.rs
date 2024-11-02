fn main() {
    println!(
        "richer {:?}",
        Solution::loud_and_rich(
            vec![
                vec![1, 0],
                vec![2, 1],
                vec![3, 1],
                vec![3, 7],
                vec![4, 3],
                vec![5, 3],
                vec![6, 3]
            ],
            vec![3, 2, 5, 4, 6, 1, 7, 0]
        )
    )
}

struct Solution;

impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let n = quiet.len();
        let mut graph = vec![Vec::new(); n];
        let mut indegree = vec![0; n];
        let mut ans: Vec<i32> = (0..n as i32).collect();

        for edge in &richer {
            graph[edge[0] as usize].push(edge[1] as usize);
            indegree[edge[1] as usize] += 1;
        }

        let mut q = vec![0; n];
        let mut l = 0;
        let mut r = 0;

        for (i, &e) in indegree.iter().enumerate() {
            if e == 0 {
                q[r] = i;
                r += 1;
            }
        }

        while r > l {
            let cur_idx = q[l];
            for &other in &graph[cur_idx] {
                if quiet[ans[cur_idx] as usize] < quiet[ans[other] as usize] {
                    ans[other] = ans[cur_idx];
                }
                if indegree[other] == 1 {
                    q[r] = other;
                    r += 1;
                }
                indegree[other] -= 1;
            }
            l += 1;
        }

        ans
    }

    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut graph = vec![Vec::new(); n + 1];
        let mut indegree = vec![0; n + 1];
        for edge in relations {
            graph[edge[0] as usize].push(edge[1] as usize);
            indegree[edge[1] as usize] += 1;
        }

        let mut q = vec![0; n];
        let mut l = 0;
        let mut r = 0;
        let mut ans = 0;
        let mut aux = vec![0; n + 1];

        indegree
            .iter()
            .skip(1)
            .take(n)
            .enumerate()
            .for_each(|(i, &e)| {
                if e == 0 {
                    q[r] = i + 1;
                    r += 1;
                }
            });

        while r > l {
            let cur_idx = q[l];
            aux[cur_idx] += time[cur_idx - 1];
            for &other in &graph[cur_idx] {
                aux[other] = aux[cur_idx].max(aux[other]);
                if indegree[other] == 1 {
                    q[r] = other;
                    r += 1;
                }
                indegree[other] -= 1;
            }
            ans = ans.max(aux[cur_idx]);
            l += 1;
        }

        ans
    }

    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let n = favorite.len();
        let mut indegree = vec![0; n];
        for &e in &favorite {
            indegree[e as usize] += 1;
        }

        let mut q = vec![0; n];
        let mut l = 0;
        let mut r = 0;
        indegree.iter().enumerate().for_each(|(i, &e)| {
            if e == 0 {
                q[r] = i;
                r += 1;
            }
        });

        let mut deep = vec![0; n];
        while r > l {
            let cur_idx = q[l];
            let next = favorite[cur_idx] as usize;
            deep[next] = deep[next].max(deep[cur_idx] + 1);
            if indegree[next] == 1 {
                q[r] = next;
                r += 1
            }
            indegree[next] -= 1;
            l += 1;
        }

        let mut small = 0;
        let mut big = 0;
        (0..n).for_each(|i| {
            let mut i = i;
            let mut ring_len = 0;
            while indegree[i] > 0 {
                ring_len += 1;
                indegree[i] = 0;
                i = favorite[i] as usize;
            }

            if ring_len == 2 {
                small += 2 + deep[i] + deep[favorite[i] as usize]
            } else if ring_len > 2 {
                big = big.max(ring_len)
            } else {
                return;
            }
        });

        small.max(big)
    }
}
