fn get_nearest_greater_no_repeat<T: Copy + PartialOrd>(nums: &[T]) -> Vec<(i32, i32)> {
    let mut res = vec![(-1, -1); nums.len()];
    let mut mono_stack = vec![];
    for (i, &item) in nums.iter().enumerate() {
        while !mono_stack.is_empty() && nums[*mono_stack.last().unwrap()] < item {
            let pop_index = mono_stack.pop().unwrap();
            let left_greater_index = match mono_stack.last() {
                Some(&last) => last as i32,
                None => -1,
            };
            res[pop_index] = (left_greater_index, i as i32);
        }
        mono_stack.push(i);
    }
    while let Some(pop_index) = mono_stack.pop() {
        let left_greater_index = match mono_stack.last() {
            Some(&last) => last as i32,
            None => -1,
        };
        res[pop_index] = (left_greater_index, -1);
    }
    res
}

fn get_nearest_greater<T: Copy + PartialOrd>(nums: &[T]) -> Vec<(i32, i32)> {
    let mut res = vec![(-1, -1); nums.len()];
    let mut mono_stack: Vec<Vec<usize>> = Vec::new();
    for (i, &item) in nums.iter().enumerate() {
        while !mono_stack.is_empty() && nums[*mono_stack.last().unwrap().last().unwrap()] < item {
            let pop_index_v = mono_stack.pop().unwrap();
            let left_greater_index = if mono_stack.is_empty() {
                -1
            } else {
                *mono_stack.last().unwrap().last().unwrap() as i32
            };
            for pop_index in pop_index_v {
                res[pop_index] = (left_greater_index, i as i32);
            }
        }
        if !mono_stack.is_empty() && nums[*mono_stack.last().unwrap().last().unwrap()] == item {
            mono_stack.last_mut().unwrap().push(i);
        } else {
            mono_stack.push(vec![i]);
        }
    }
    while !mono_stack.is_empty() {
        let pop_index_v = mono_stack.pop().unwrap();
        let left_greater_index = if mono_stack.is_empty() {
            -1
        } else {
            *mono_stack.last().unwrap().last().unwrap() as i32
        };
        for pop_index in pop_index_v {
            res[pop_index] = (left_greater_index, -1);
        }
    }
    res
}

#[test]
fn test() {
    let nums = vec![3, 4, 1, 5, 6, 2, 7];
    println!("{:?}", get_nearest_greater_no_repeat(&nums));
    let nums = vec![3, 4, 1, 5, 6, 5, 2, 7];
    println!("{:?}", get_nearest_greater(&nums));
}
