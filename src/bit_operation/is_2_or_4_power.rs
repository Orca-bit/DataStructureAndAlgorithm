fn is_2_power(num: i32) -> bool {
    num & (num - 1) == 0
}

fn is_4_power(num: i32) -> bool {
    is_2_power(num) && num & 0x55555555 != 0        // 0x55555555 = 01010101...0101
}

#[test]
fn test() {
    assert_eq!(is_2_power(2), true);
    assert_eq!(is_2_power(0), true);
    assert_eq!(is_2_power(3), false);
    assert_eq!(is_2_power(200), false);
    assert_eq!(is_4_power(64), true);
    assert_eq!(is_4_power(10), false);
}