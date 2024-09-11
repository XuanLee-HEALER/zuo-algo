use std::i32;

fn main() {
    assert_eq!(5 + 6, bit_add(5, 6));
    assert_eq!(5 - 6, bit_sub(5, 6));
    assert_eq!(-5 * 6, bit_multi(-5, 6));
    assert_eq!(-1 * -5, bit_multi(-1, -5));
    assert_eq!(-1 / -5, bit_div(-1, -5));
    assert_eq!(12 / 5, bit_div(12, 5));
    assert_eq!(12 / -5, bit_div(12, -5));
    assert_eq!(i32::MAX, bit_gen_div(i32::MIN, -1));
    assert_eq!(i32::MIN / 66, bit_gen_div(i32::MIN, 66));
    assert_eq!(i32::MIN / -66, bit_gen_div(i32::MIN, -66));
}

fn bit_add(mut a: i32, mut b: i32) -> i32 {
    let mut res = a;
    while b != 0 {
        res = a ^ b;
        b = (a & b) << 1;
        a = res;
    }
    res
}

fn bit_neg(a: i32) -> i32 {
    bit_add(!a, 1)
}

fn bit_sub(a: i32, b: i32) -> i32 {
    bit_add(a, bit_neg(b))
}

fn bit_multi(mut a: i32, mut b: i32) -> i32 {
    let mut res = 0;
    while b != 0 {
        if b & 1 != 0 {
            res = bit_add(res, a);
        }
        a <<= 1;
        b = (b >> 1) & 0x7fffffff;
    }
    res
}

/// a,b不能是整数最小值
fn bit_div(a: i32, b: i32) -> i32 {
    let mut ca = if a >= 0 { a } else { bit_neg(a) };
    let cb = if b >= 0 { b } else { bit_neg(b) };
    let mut res = 0;
    for i in (0..31).rev() {
        if ca >> i >= cb {
            res |= 1 << i;
            ca = bit_sub(ca, cb << i);
        }
    }

    if (a > 0) ^ (b > 0) {
        bit_neg(res)
    } else {
        res
    }
}

/// 不限制a,b的范围
fn bit_gen_div(a: i32, b: i32) -> i32 {
    const I32_MIN: i32 = i32::MIN;
    const OFFSET: i32 = 1;
    match (a, b) {
        (I32_MIN, I32_MIN) => 1,
        (_, I32_MIN) => 0,
        (I32_MIN, -1) => i32::MAX,
        (I32_MIN, x) => {
            if x >= 0 {
                bit_div(bit_add(I32_MIN, x), x) - OFFSET
            } else {
                bit_div(bit_sub(I32_MIN, x), x) + OFFSET
            }
        }
        (a, b) => bit_div(a, b),
    }
}
