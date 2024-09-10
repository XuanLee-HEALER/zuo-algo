fn main() {
    let mut x = 5;
    let mut y = 8;
    println!("two number x={} y={}", x, y);
    exchange_2_nums(&mut x, &mut y);
    println!("after exchange x={} y={}", x, y);

    let a = 12;
    let b = 24;
    assert_eq!(24, max_val_1(a, b));
    // 溢出
    // assert_eq!(i32::MAX, max_val_1(i32::MIN, i32::MAX));
    assert_eq!(24, max_val_2(a, b));
    assert_eq!(i32::MAX, max_val_2(i32::MIN, i32::MAX));

    assert_eq!(3, find_three_num(&[1, 1, 1, 3], 3));
}

fn exchange_2_nums(a: &mut i32, b: &mut i32) {
    std::mem::swap(&mut (*a), &mut (*b));
    // *a ^= *b;
    // *b ^= *a;
    // *a ^= *b;
}

/// n只能是0或1，取反
fn flip(mut n: i32) -> i32 {
    n ^= 1;
    n
}

fn sign(n: i32) -> i32 {
    (n >> 31) & 1
}

/// 不考虑溢出
fn max_val_1(a: i32, b: i32) -> i32 {
    let c = a - b;
    // 如果c_sign是1，就是负数，则a更小，ret_a应该为0
    let c_sign = sign(c);
    let ret_a = flip(c_sign);
    ret_a * a + c_sign * b
}

/// 考虑溢出
fn max_val_2(a: i32, b: i32) -> i32 {
    let c = a.wrapping_sub(b);
    let c_sign = sign(c);

    let a_sigh = sign(a);
    let b_sign = sign(b);
    // a b符号相同，c不会溢出
    // a b符号不同，只要看a是不是正数
    // 如果不一样返回1，如果一样返回0
    let diff_s = a_sigh ^ b_sign;
    let same_s = flip(diff_s);
    let ret_a = diff_s * flip(a_sigh) + same_s * flip(c_sign);
    // let ret_a = ((a_sigh ^ b_sign) & flip(a_sigh)) | (flip(a_sigh ^ b_sign) & flip(c_sign));
    ret_a * a + flip(ret_a) * b
}

fn find_miss_num(arr: &[i32]) -> i32 {
    let mut xor1 = 0;
    let mut xor2 = 0;
    for (idx, e) in arr.iter().enumerate() {
        xor1 ^= idx as i32;
        xor2 ^= *e;
    }
    xor1 ^= arr.len() as i32;
    xor1 ^ xor2
}

fn find_single_num(arr: &[i32]) -> i32 {
    let mut xor = 0;
    for e in arr {
        xor ^= *e;
    }
    xor
}

fn find_double_num(arr: &[i32]) -> [i32; 2] {
    let mut xor = 0;
    for e in arr {
        xor ^= *e;
    }

    let r_d = xor & (-xor);
    let mut xor1 = 0;
    for e in arr {
        if (*e & r_d) != 0 {
            xor1 ^= *e;
        }
    }
    [xor1, xor1 ^ xor]
}

fn find_three_num(arr: &[i32], m: i32) -> i32 {
    let mut counter = [0; 32];
    for e in arr {
        for (i, xe) in (0..=31).enumerate() {
            counter[i] += (*e >> xe) & 1;
        }
    }
    let mut res = 0;
    for (idx, e) in counter.iter().enumerate() {
        res |= (*e % m) << idx
    }
    res
}
