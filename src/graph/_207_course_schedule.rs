use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

#[derive(Clone)]
struct Node {
    course: i32,
    in_degree: i32,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(course: i32) -> Self {
        Self {
            course,
            in_degree: 0,
            children: vec![],
        }
    }
}

struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        if prerequisites.is_empty() {
            return true;
        }
        let mut map = HashMap::new();
        for arr in prerequisites.iter() {
            let to = arr[0];
            let from = arr[1];
            map.entry(to)
                .or_insert(Rc::new(RefCell::new(Node::new(to))))
                .borrow_mut()
                .in_degree += 1;
            let to = map.get(&to).unwrap().clone();
            map.entry(from)
                .or_insert(Rc::new(RefCell::new(Node::new(from))))
                .borrow_mut()
                .children
                .push(to);
        }
        let size = map.len();
        let mut que = map
            .values()
            .filter(|a| a.borrow().in_degree == 0)
            .map(|a| a.clone())
            .collect::<VecDeque<_>>();
        let mut count = 0;
        while let Some(cur) = que.pop_front() {
            count += 1;
            for child in cur.borrow().children.iter() {
                child.borrow_mut().in_degree -= 1;
                if child.borrow().in_degree == 0 {
                    que.push_back(child.clone());
                }
            }
        }
        count == size
    }
}

#[test]
fn test() {
    let num_courses = 2;
    let prerequisites: Vec<Vec<i32>> = vec![vec![1, 0]];
    let res = true;
    assert_eq!(Solution::can_finish(num_courses, prerequisites), res);
    let num_courses = 2;
    let prerequisites: Vec<Vec<i32>> = vec![vec![1, 0], vec![0, 1]];
    let res = false;
    assert_eq!(Solution::can_finish(num_courses, prerequisites), res);
}
