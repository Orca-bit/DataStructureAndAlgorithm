struct Xor;

impl Xor {
    pub fn print_odd_num_1(arr: &[i32]) -> i32 {
        let mut xor = 0;
        for &item in arr {
            xor ^= item;
        }
        xor
    }

    pub fn print_odd_nums_2(arr: &[i32]) -> (i32, i32) {
        let mut xor = 0;
        for &item in arr {
            xor ^= item;
        }
        let right_one = xor & (!xor + 1);
        let mut xor1 = 0;
        for &item in arr {
            if item & right_one == 0 {
                xor1 ^= item;
            }
        }
        (xor1, xor ^ xor1)
    }
}

#[test]
fn test() {
    let arr = vec![1 ,1, 2, 3, 2, 3, 4];
    assert_eq!(Xor::print_odd_num_1(&arr), 4);
    let arr2 = vec![1 ,1, 2, 3, 2, 3, 4, 5];
    println!("{:?}",Xor::print_odd_nums_2(&arr2));
}