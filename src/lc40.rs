struct Solution;

// Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sum to target.

// Each number in candidates may only be used once in the combination.

// Note: The solution set must not contain duplicate combinations.

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let arr = candidates
            .into_iter()
            .filter(|&v| v <= target)
            .collect::<Vec<i32>>();
        if arr.len() == 0 {
            vec![]
        } else {
            let mut cnts = vec![0; target as usize + 1];
            arr.iter().for_each(|&v| cnts[v as usize] += 1);
            Self::cs2(&cnts, target, target)
        }
    }

    // 到i这个数为止，能达成目标target的所有子序列
    fn cs2(cnts: &[i32], target: i32, i: i32) -> Vec<Vec<i32>> {
        if target > 0 {
            if i == 0 {
                vec![]
            } else if cnts[i as usize] == 0 {
                Self::cs2(cnts, target, i - 1)
            } else {
                let mut res = Self::cs2(cnts, target, i - 1);
                for k in 1..=cnts[i as usize] {
                    if i * k < target {
                        let mut t_res = Self::cs2(cnts, target - i * k, i - 1);
                        if !t_res.is_empty() {
                            for sub in t_res.iter_mut() {
                                for _ in 0..k {
                                    sub.push(i);
                                }
                            }
                            res.append(&mut t_res);
                        }
                    } else if i * k == target {
                        let mut t_res = Vec::with_capacity(k as usize);
                        for _ in 0..k {
                            t_res.push(i);
                        }
                        res.append(&mut vec![t_res]);
                    } else {
                        break;
                    }
                }
                res
            }
        } else {
            // target <= 0，不需要任何子序列
            vec![]
        }
    }
}

#[cfg(test)]
mod test_solution {
    #[test]
    fn test_combination_sum2() {
        let sample1_arr = vec![10, 1, 2, 7, 6, 1, 5];
        let sample1_target = 8;
        //         [
        // [1,1,6],
        // [1,2,5],
        // [1,7],
        // [2,6]
        // ]
        println!(
            "{:?}",
            super::Solution::combination_sum2(sample1_arr, sample1_target)
        );
        let sample1_arr = vec![2, 5, 2, 1, 2];
        let sample1_target = 5;
        // [
        // [1,2,2],
        // [5]
        // ]
        println!(
            "{:?}",
            super::Solution::combination_sum2(sample1_arr, sample1_target)
        );
    }
}
