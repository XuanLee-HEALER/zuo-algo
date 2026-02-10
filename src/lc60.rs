struct Solution;

impl Solution {
    // n个数的全排列共 n! 种，按字典序排列后，可以逐位确定第 k 个排列：
    // 第1位：固定第1位后，剩余 n-1 个数有 (n-1)! 种排列，
    //   所以第1位是第 k/(n-1)! 个可选数字（0-index）
    // 取余：k %= (n-1)!，含义是去掉已经跳过的整组排列，
    //   得到在「第1位确定后的子排列」中的新排名
    //   例如 n=3, k=4(0-index)：k/2!=2 → 第1位选第3个数，
    //   k%2!=0 → 在剩余2个数的排列中排第0个
    // 之后问题变成：从剩余数字中求第 k 个排列，完全相同的子问题
    pub fn get_permutation(n: i32, k: i32) -> String {
        let n = n as usize;
        let mut factorial = vec![1usize; n + 1];
        for i in 1..=n {
            factorial[i] = factorial[i - 1] * i;
        }
        let mut nums: Vec<u8> = (1..=n as u8).collect();
        let mut res = String::new();
        let mut k = k as usize - 1; // 转为 0-index，方便直接整除定位
        for i in (1..=n).rev() {
            let f = factorial[i - 1]; // 固定当前位后，剩余数字的排列数
            let idx = k / f; // 当前位选第几个可选数字
            res.push((b'0' + nums.remove(idx)) as char);
            k %= f; // 去掉已跳过的整组，得到子问题中的新排名
        }
        res
    }
}

#[cfg(test)]
mod test_lc60 {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::get_permutation(3, 3), "213");
        assert_eq!(Solution::get_permutation(4, 9), "2314");
        assert_eq!(Solution::get_permutation(3, 1), "123");
    }
}
