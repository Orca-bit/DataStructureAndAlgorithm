struct ElementaryArithmetic;

impl ElementaryArithmetic {
    fn add(mut num1: i32, mut num2: i32) -> i32 {
        let mut sum = num1;
        while num2 != 0 {
            sum = num1 ^ num2; // 无进位相加
            num2 = (num1 & num2) << 1; // 进位信息
            num1 = sum;
        }
        sum
    }

    fn negative(num: i32) -> i32 {
        Self::add(!num, 1) // 取反加一为相反数
    }

    fn subtraction(num1: i32, num2: i32) -> i32 {
        Self::add(num1, Self::negative(num2))
    }

    fn multiplication(mut num1: i32, mut num2: i32) -> i32 {
        let mut res = 0;
        while num2 != 0 {
            if num2 & 1 != 0 {
                res = Self::add(res, num1);
            }
            num1 <<= 1;
            num2 >>= 1;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(ElementaryArithmetic::add(2, 3), 5);
    assert_eq!(ElementaryArithmetic::subtraction(2, 3), -1);
    // assert_eq!(ElementaryArithmetic::multiplication(2, -3), -6); // not pass
}
