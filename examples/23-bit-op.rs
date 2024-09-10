fn main() {
    // for i in -5..5 {
    //     println!("number {} is power of 2: {}", i, is_power_two(i));
    //     println!("number {} is power of 3: {}", i, is_power_three(i));
    // }

    assert_eq!(1, bit_1_num(2));
    assert_eq!(1, bit_1_num(4));
    assert_eq!(31, bit_1_num(-2));
    assert!(is_power_of_four_1(4_i32.pow(10)));
    assert!(is_power_of_four_1(1));
    assert!(is_power_of_four_1(4));

    assert_eq!(4, nearest_power_of_2(3));
    assert_eq!(4, x_y_bit_and(5, 7));
    assert_eq!(0x80000000_u32, reverse_bit(1_u32));
}

fn is_power_two(n: i32) -> bool {
    n > 0 && n == (n & (-n))
}

fn is_power_three(n: i32) -> bool {
    n > 0 && (3_i32.pow(19) % n) == 0
}

fn is_power_of_four(n: i32) -> bool {
    match n {
        n if n <= 0 => false,
        n => {
            for i in 0..=15 {
                if 4_i32.pow(i) == n {
                    return true;
                }
            }
            false
        }
    }
}

fn is_power_of_four_1(n: i32) -> bool {
    n > 0 && (n == 1 || (n == n & -n && bit_1_num(n - 1) % 2 == 0))
}

fn nearest_power_of_2(n: i32) -> i32 {
    if n <= 0 {
        0
    } else {
        let mut n = n - 1;
        n |= n >> 1;
        n |= n >> 2;
        n |= n >> 4;
        n |= n >> 8;
        n |= n >> 16;
        n + 1
    }
}

fn x_y_bit_and(x: i32, y: i32) -> i32 {
    let (x, mut y) = (x, y);
    while y > x {
        y &= y - 1;
    }
    x & y
}

fn reverse_bit(n: u32) -> u32 {
    let mut n = n;
    n = (n & 0xaaaaaaaa) >> 1 | (n & 0x55555555) << 1;
    n = (n & 0xcccccccc) >> 2 | (n & 0x33333333) << 2;
    n = (n & 0xf0f0f0f0) >> 4 | (n & 0x0f0f0f0f) << 4;
    n = (n & 0xff00ff00) >> 8 | (n & 0x00ff00ff) << 8;
    n.rotate_left(16)
}

fn bit_1_num(n: i32) -> i32 {
    let mut n = n;
    n = (n & (0x55555555)).wrapping_add((n >> 1) & (0x7fffffff) & (0x55555555));
    n = (n & (0x33333333)).wrapping_add((n >> 2) & (0x3fffffff) & (0x33333333));
    n = (n & (0x0f0f0f0f)).wrapping_add((n >> 4) & (0x0fffffff) & (0x0f0f0f0f));
    n = (n & (0x00ff00ff)).wrapping_add((n >> 8) & (0x00ffffff) & (0x00ff00ff));
    n = (n & (0x0000ffff)).wrapping_add((n >> 16) & (0x0000ffff) & (0x0000ffff));
    n
}
