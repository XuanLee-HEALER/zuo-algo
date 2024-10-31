fn main() {
    let n = 6;
    let edges = vec![
        vec![1, 2, -1],
        vec![1, 4, 2],
        vec![3, 2, 4],
        vec![2, 3, 3],
        vec![4, 2, 5],
        vec![4, 5, 0],
        vec![2, 5, -3],
        vec![2, 6, 12],
        vec![6, 3, 16],
    ];
    let d_w = Graph::new_direct_weight(n, &edges);
    d_w.iterate();
    let id_w = Graph::new_indirect_weight(n, &edges);
    id_w.iterate();
    let d_uw = Graph::new_direct_unweight(n, &edges);
    d_uw.iterate();
    let id_uw = Graph::new_indirect_unweight(n, &edges);
    id_uw.iterate();
}

const MAX_N: usize = 21;
const MAX_EDGE: usize = 81;

struct Graph {
    n: usize,

    // 邻接矩阵
    adjacent_matrix: [[i32; MAX_N]; MAX_N],

    // 邻接表
    adjacent_table: Vec<Vec<(i32, i32)>>,

    // 链式前向星
    head: [usize; MAX_N],
    next: [usize; MAX_EDGE],
    to: [usize; MAX_EDGE],
    weight: Option<[i32; MAX_EDGE]>,
    cnt: usize,
}

impl Graph {
    fn new_direct_weight(n: usize, edges: &[Vec<i32>]) -> Self {
        let mut a_m = [[i32::MAX; MAX_N]; MAX_N];
        let mut a_t = vec![vec![]; n + 1];

        let mut h = [0; MAX_N];
        let mut nx = [0; MAX_EDGE];
        let mut t = [0; MAX_EDGE];
        let mut w = [0; MAX_EDGE];
        let mut cnt = 1;

        for edge in edges {
            a_m[edge[0] as usize][edge[1] as usize] = edge[2];
            a_t[edge[0] as usize].push((edge[1], edge[2]));
            nx[cnt] = h[edge[0] as usize];
            h[edge[0] as usize] = cnt;
            t[cnt] = edge[1] as usize;
            w[cnt] = edge[2];
            cnt += 1;
        }

        Self {
            n,
            adjacent_matrix: a_m,
            adjacent_table: a_t,
            head: h,
            next: nx,
            to: t,
            weight: Some(w),
            cnt,
        }
    }

    fn new_indirect_weight(n: usize, edges: &[Vec<i32>]) -> Self {
        let mut a_m = [[i32::MAX; MAX_N]; MAX_N];
        let mut a_t = vec![vec![]; n + 1];

        let mut h = [0; MAX_N];
        let mut nx = [0; MAX_EDGE];
        let mut t = [0; MAX_EDGE];
        let mut w = [0; MAX_EDGE];
        let mut cnt = 1;

        for edge in edges {
            a_m[edge[0] as usize][edge[1] as usize] = edge[2];
            a_m[edge[1] as usize][edge[0] as usize] = edge[2];
            a_t[edge[0] as usize].push((edge[1], edge[2]));
            a_t[edge[1] as usize].push((edge[0], edge[2]));

            nx[cnt] = h[edge[0] as usize];
            h[edge[0] as usize] = cnt;
            t[cnt] = edge[1] as usize;
            w[cnt] = edge[2];
            cnt += 1;
            nx[cnt] = h[edge[1] as usize];
            h[edge[1] as usize] = cnt;
            t[cnt] = edge[0] as usize;
            w[cnt] = edge[2];
            cnt += 1;
        }

        Self {
            n,
            adjacent_matrix: a_m,
            adjacent_table: a_t,
            head: h,
            next: nx,
            to: t,
            weight: Some(w),
            cnt,
        }
    }

    fn new_direct_unweight(n: usize, edges: &[Vec<i32>]) -> Self {
        let mut a_m = [[i32::MAX; MAX_N]; MAX_N];
        let mut a_t = vec![vec![]; n + 1];

        let mut h = [0; MAX_N];
        let mut nx = [0; MAX_EDGE];
        let mut t = [0; MAX_EDGE];
        let mut cnt = 1;

        for edge in edges {
            a_m[edge[0] as usize][edge[1] as usize] = 1;
            a_t[edge[0] as usize].push((edge[1], 0));
            nx[cnt] = h[edge[0] as usize];
            h[edge[0] as usize] = cnt;
            t[cnt] = edge[1] as usize;
            cnt += 1;
        }

        Self {
            n,
            adjacent_matrix: a_m,
            adjacent_table: a_t,
            head: h,
            next: nx,
            to: t,
            weight: None,
            cnt,
        }
    }

    fn new_indirect_unweight(n: usize, edges: &[Vec<i32>]) -> Self {
        let mut a_m = [[i32::MAX; MAX_N]; MAX_N];
        let mut a_t = vec![vec![]; n + 1];

        let mut h = [0; MAX_N];
        let mut nx = [0; MAX_EDGE];
        let mut t = [0; MAX_EDGE];
        let mut cnt = 1;

        for edge in edges {
            a_m[edge[0] as usize][edge[1] as usize] = 1;
            a_m[edge[1] as usize][edge[0] as usize] = 1;
            a_t[edge[0] as usize].push((edge[1], 0));
            a_t[edge[1] as usize].push((edge[0], 0));

            nx[cnt] = h[edge[0] as usize];
            h[edge[0] as usize] = cnt;
            t[cnt] = edge[1] as usize;
            cnt += 1;
            nx[cnt] = h[edge[1] as usize];
            h[edge[1] as usize] = cnt;
            t[cnt] = edge[0] as usize;
            cnt += 1;
        }

        Self {
            n,
            adjacent_matrix: a_m,
            adjacent_table: a_t,
            head: h,
            next: nx,
            to: t,
            weight: None,
            cnt,
        }
    }

    fn iterate(&self) {
        println!("===== 邻接矩阵遍历 =====");
        for i in 1..=self.n {
            for j in 1..=self.n {
                if self.adjacent_matrix[i][j] == i32::MAX {
                    print!("- ")
                } else {
                    print!("{} ", self.adjacent_matrix[i][j]);
                }
            }
            println!()
        }

        let mut buf = String::new();
        println!("===== 邻接表遍历 =====");
        for i in 1..=self.n {
            buf.push_str(&format!("{i}: "));
            for &(to, weight) in &self.adjacent_table[i] {
                buf.push_str(&format!("({}, {}) ", to, weight));
            }
            println!("{}", buf);
            buf.clear();
        }

        println!("===== 链式前向星遍历 =====");
        for i in 1..=self.n {
            let mut j = self.head[i];
            buf.push_str(&format!("{i}: "));
            while self.head[i] != 0 && j != 0 {
                buf.push_str(&format!(
                    "({}, {}) ",
                    self.to[j],
                    if let Some(w) = self.weight { w[j] } else { 0 }
                ));
                j = self.next[j];
            }
            println!("{}", buf);
            buf.clear();
        }
    }
}
