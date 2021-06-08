fn get_larger_num(a: i32, b: i32) -> i32 {
    let c = a.overflowing_sub(b).0;         // 允许溢出，测试算法
    let sa = sign(a);
    let sb = sign(b);
    let sc = sign(c);
    let dif_sab = sa ^ sb;
    let same_sab = flip(dif_sab);
    let return_a = dif_sab * sa + same_sab * sc;
    let return_b = flip(return_a);
    return_a * a + return_b * b
}

fn flip(num: i32) -> i32 {          // 1 -> 0, 0 ->１
    num ^ 1
}

fn sign(num: i32) -> i32 {          // 非负返回1,负数返回0
    flip((num >> 31) & 1)
}

#[test]
fn test() {
    assert_eq!(-1, get_larger_num(i32::MIN, -1));
    assert_eq!(i32::MAX, get_larger_num(i32::MIN, i32::MAX));
}