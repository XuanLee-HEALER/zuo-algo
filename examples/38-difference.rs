fn main() {}

struct Solution;

impl Solution {
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut ans = vec![0; n as usize + 2];
        for booking in &bookings {
            let (l, r, v) = (booking[0], booking[1], booking[2]);
            ans[l as usize] += v;
            ans[r as usize + 1] -= v;
        }

        let mut sum = 0;
        for v in ans.iter_mut() {
            sum += *v;
            *v = sum;
        }

        ans[1..ans.len() - 1].into()
    }
}
