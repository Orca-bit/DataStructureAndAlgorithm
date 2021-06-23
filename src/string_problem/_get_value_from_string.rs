use std::collections::VecDeque;

fn get_value(exp: String) -> i32 {
    process(&exp.chars().collect(), 0).0
}

fn process(chars: &Vec<char>, mut index: usize) -> (i32, usize) {
    let mut deque = VecDeque::new();
    let mut num = 0;
    while index < chars.len() && chars[index] != ')' {
        if ('0'..='9').contains(&chars[index]) {
            num = num * 10 + (chars[index] as u8 - b'0') as i32;
            index += 1;
        } else if chars[index] == '(' {
            let next = process(chars, index + 1);
            num = next.0;
            index = next.1 + 1;
        } else {
            add_num(&mut deque, num);
            num = 0;
            deque.push_back(chars[index].to_string());
            index += 1;
        }
    }
    add_num(&mut deque, num);
    (get_num(deque), index)
}

fn add_num(deque: &mut VecDeque<String>, mut num: i32) {
    if !deque.is_empty() {
        let top = deque.pop_back().unwrap();
        if top.as_str() == "+" || top.as_str() == "-" {
            deque.push_back(top);
        } else {
            let cur = deque.pop_back().unwrap().parse::<i32>().unwrap();
            num = match top.as_str() {
                "*" => cur * num,
                "/" => cur / num,
                _ => panic!("check the '*' or '/'"),
            }
        }
    }
    deque.push_back(num.to_string());
}

fn get_num(mut deque: VecDeque<String>) -> i32 {
    let mut res = 0;
    let mut add= true;
    while !deque.is_empty() {
        let cur = deque.pop_front().unwrap();
        if cur.as_str() == "+" {
            add = true;
        } else if cur.as_str() == "-" {
            add = false;
        } else {
            let num = cur.parse::<i32>().unwrap();
            res += if add { num } else { -num };
        }
    }
    res
}

#[test]
fn test() {
    let exp = "48*((70-65)-43)+8*1".to_string();
    assert_eq!(get_value(exp), -1816);
}