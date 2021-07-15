struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let process = || -> Option<i32> {
            let mut stk = vec![];
            for k in tokens {
                match k.as_str() {
                    "+" => {
                        let right = stk.pop()?;
                        let left = stk.pop()?;
                        stk.push(left + right);
                    }
                    "-" => {
                        let right = stk.pop()?;
                        let left = stk.pop()?;
                        stk.push(left - right);
                    }
                    "*" => {
                        let right = stk.pop()?;
                        let left = stk.pop()?;
                        stk.push(left * right);
                    }
                    "/" => {
                        let right = stk.pop()?;
                        let left = stk.pop()?;
                        stk.push(left / right);
                    }
                    _ => {
                        stk.push(k.parse().unwrap());
                    }
                }
            }
            stk.pop()
        };
        process().unwrap()
    }
}

#[test]
fn test() {
    let tokens = vec![
        "2".to_string(),
        "1".to_string(),
        "+".to_string(),
        "3".to_string(),
        "*".to_string(),
    ];
    let res = 9;
    assert_eq!(Solution::eval_rpn(tokens), res);
    let tokens = vec![
        "4".to_string(),
        "13".to_string(),
        "5".to_string(),
        "/".to_string(),
        "+".to_string(),
    ];
    let res = 6;
    assert_eq!(Solution::eval_rpn(tokens), res);
    let tokens = vec![
        "10".to_string(),
        "6".to_string(),
        "9".to_string(),
        "3".to_string(),
        "+".to_string(),
        "-11".to_string(),
        "*".to_string(),
        "/".to_string(),
        "*".to_string(),
        "17".to_string(),
        "+".to_string(),
        "5".to_string(),
        "+".to_string(),
    ];
    let res = 22;
    assert_eq!(Solution::eval_rpn(tokens), res);
}
