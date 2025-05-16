use std::cmp::Ordering;

use zuo_algo::Heap;

fn main() {
    println!(
        "{:?}",
        Solution::min_interval(
            vec![vec![2, 3], vec![2, 5], vec![1, 8], vec![20, 25]],
            vec![2, 19, 5, 22]
        )
    )
}

struct Solution;

impl Solution {
    pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        #[derive(Clone, Copy)]
        struct Unit(i32, i32);
        impl Eq for Unit {}
        impl PartialEq for Unit {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl Ord for Unit {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0)
            }
        }
        impl PartialOrd for Unit {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                match self.0.partial_cmp(&other.0) {
                    Some(core::cmp::Ordering::Equal) => {}
                    ord => return ord,
                }
                self.1.partial_cmp(&other.1)
            }
        }
        let mut intervals = intervals;
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut qs: Vec<(usize, &i32)> = queries.iter().enumerate().collect();
        qs.sort_unstable_by(|a, b| a.1.cmp(&b.1));
        let mut res = vec![0; qs.len()];
        let mut heap = Heap::new(intervals.len());
        let mut j = 0;
        for (i, l) in qs {
            while j < intervals.len() {
                let tl = intervals[j][0];
                let tr = intervals[j][1];
                if tl <= *l {
                    heap.insert(Unit(tr - tl + 1, tr));
                    j += 1;
                } else {
                    break;
                }
            }
            while !heap.is_empty() {
                // if i == 2 {
                //     println!("=> {}", heap.peek().unwrap().1)
                // }
                if heap.peek().unwrap().1 < *l {
                    heap.remove();
                } else {
                    break;
                }
            }
            res[i] = if let Some(h) = heap.peek() { h.0 } else { -1 }
        }
        res
    }

    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        #[derive(Clone, Copy)]
        struct Task(i32, i32);

        impl Eq for Task {}

        impl PartialEq for Task {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl Ord for Task {
            fn cmp(&self, other: &Self) -> Ordering {
                other.0.cmp(&self.0)
            }
        }

        impl PartialOrd for Task {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut buildings = buildings;
        let mut dots = Vec::with_capacity(buildings.len() << 1);
        for build in &buildings {
            let (l, r) = (build[0], build[1]);
            dots.push(l);
            dots.push(r - 1);
            dots.push(r);
        }
        dots.sort_unstable();
        dots.dedup();
        buildings.sort_unstable_by(|a, b| a[0].cmp(&b[0]));
        let mut dot_height = vec![0; dots.len()];
        let mut heap = zuo_algo::Heap::new(buildings.len());
        let mut j = 0;
        for (i, &dot) in dots.iter().enumerate() {
            while j < buildings.len() {
                if buildings[j][0] <= dot {
                    heap.insert(Task(
                        buildings[j][2],
                        dots.binary_search(&(buildings[j][1] - 1)).unwrap() as i32,
                    ));
                    j += 1
                } else {
                    break;
                }
            }

            while !heap.is_empty() {
                if let Some(h) = heap.peek() {
                    if h.1 < i as i32 {
                        heap.remove();
                        continue;
                    }
                }
                break;
            }

            dot_height[i] = if let Some(head) = heap.peek() {
                head.0
            } else {
                0
            }
        }
        let mut res = vec![];
        let mut pre = 0;
        for (i, dh) in dot_height.iter().enumerate() {
            if *dh != pre {
                res.push(vec![dots[i], *dh]);
                pre = *dh
            }
        }
        res
    }
}
