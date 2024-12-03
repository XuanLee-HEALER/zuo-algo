/// Write a function that reverses a string. The input string is given as an array of characters s.
///
/// You must do this by modifying the input array in-place with O(1) extra memory.

struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        // let mut l = 0;
        // let mut r = s.len() - 1;
        // while l < r {
        //     s.swap(l, r);
        //     l += 1;
        //     r -= 1;
        // }
        s.reverse();
    }
}
