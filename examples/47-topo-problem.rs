fn main() {
    println!(
        "result {:?}",
        Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]) // Solution::find_order(2, vec![vec![1, 0]])
    );
    println!(
        "alien ({})",
        Solution::alien_order(vec!["z".into(), "z".into(),])
    );
    println!(
        "stamp {:?}",
        Solution::moves_to_stamp("uskh".into(), "uskhkhhskh".into())
    );
}

struct Solution;

impl Solution {
    /// 要读a 必须先读b，拓扑排序结果 b要在a前面，所以图应该是 b->a
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut adjascent_table = vec![vec![0; 0]; num_courses as usize];
        let mut indegree = vec![0; num_courses as usize];
        for edge in &prerequisites {
            adjascent_table[edge[1] as usize].push(edge[0] as usize);
            indegree[edge[0] as usize] += 1;
        }

        let mut arr = vec![0; num_courses as usize];
        let mut l = 0;
        let mut r = 0;
        let mut q = vec![0; num_courses as usize];
        let mut cnt = 0;
        for (idx, &e) in indegree.iter().enumerate() {
            if e == 0 {
                q[r] = idx;
                r += 1;
            }
        }

        while r > l {
            arr[cnt] = q[l] as i32;
            for &other in &adjascent_table[q[l]] {
                if indegree[other] == 1 {
                    q[r] = other;
                    r += 1;
                }
                indegree[other] -= 1;
            }
            l += 1;
            cnt += 1;
        }

        if cnt == num_courses as usize {
            arr
        } else {
            vec![]
        }
    }

    pub fn alien_order(words: Vec<String>) -> String {
        let c_to_idx = |c: char| -> usize { (c as u8 - b'a') as usize };
        let idx_to_c = |i: usize| -> char { (i as u8 + b'a') as char };
        let mut indegree = [-1; 26];
        for s in &words {
            s.chars().for_each(|c| {
                if indegree[c_to_idx(c)] == -1 {
                    indegree[c_to_idx(c)] += 1
                }
            });
        }
        let mut graph = vec![vec![0; 0]; 26];
        let mut i = 0;
        let mut j = 1;
        while j < words.len() {
            let c1: Vec<char> = words[i].chars().collect();
            let c2: Vec<char> = words[j].chars().collect();
            let mut idx = 0;
            while idx < c1.len().min(c2.len()) {
                if c1[idx] != c2[idx] {
                    let c1_idx = c_to_idx(c1[idx]);
                    let c2_idx = c_to_idx(c2[idx]);
                    graph[c1_idx].push(c2_idx);
                    indegree[c2_idx] += 1;
                    break;
                }
                idx += 1;
            }
            if idx == c2.len() && c1.len() > c2.len() {
                return "".into();
            }
            i += 1;
            j += 1;
        }

        let mut q = vec![0; 26];
        let mut l = 0;
        let mut r = 0;
        let mut total_alpha = 0;
        let mut ans = String::new();
        for (i, &v) in indegree.iter().enumerate() {
            if v != -1 {
                total_alpha += 1;
                if v == 0 {
                    q[r] = i;
                    r += 1;
                }
            }
        }

        while r > l {
            ans.push(idx_to_c(q[l]));
            for &other in &graph[q[l]] {
                if indegree[other] == 1 {
                    q[r] = other;
                    r += 1;
                }
                indegree[other] -= 1;
            }
            l += 1;
        }

        if ans.len() == total_alpha {
            ans
        } else {
            "".into()
        }
    }

    pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
        let stamp: Vec<char> = stamp.chars().collect();
        let target: Vec<char> = target.chars().collect();
        let n = stamp.len();
        let m = target.len();
        let pn = m - n + 1;
        let mut graph = vec![vec![0; 0]; m + 1];
        let mut indegree = vec![0; m + 1];
        for (i, _) in target.iter().take(pn).enumerate() {
            let mut ti = i;
            for &c in &stamp {
                if c != target[ti] {
                    graph[ti].push(i);
                    indegree[i] += 1;
                }
                ti += 1;
            }
        }

        let mut q = vec![0; m + 1];
        let mut l = 0;
        let mut r = 0;
        let mut ans = vec![0; m + 1];
        let mut cnt = 0;

        for (i, &e) in indegree.iter().take(pn).enumerate() {
            if e == 0 {
                q[r] = i;
                r += 1;
            }
        }

        let mut visited = vec![false; m + 1];
        while r > l {
            let cur_loc = q[l];
            ans[cnt] = cur_loc as i32;
            for (j, e) in graph.iter().skip(cur_loc).take(n).enumerate() {
                if !visited[j + cur_loc] {
                    for &x in e {
                        indegree[x] -= 1;
                        if indegree[x] == 0 {
                            q[r] = x;
                            r += 1;
                        }
                    }
                    visited[j + cur_loc] = true;
                }
            }

            cnt += 1;
            l += 1;
        }

        if cnt == pn {
            let ans = &mut ans[..cnt];
            ans.reverse();
            ans.into()
        } else {
            vec![]
        }
    }
}
