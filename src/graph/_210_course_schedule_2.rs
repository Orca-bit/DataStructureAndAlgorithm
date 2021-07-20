use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = (0..num_courses).collect::<Vec<_>>();
        let map = Node::construct(&prerequisites);
        let size = map.len();
        let mut queue = VecDeque::new();
        let mut index = 0;
        for i in 0..res.len() {
            if !map.contains_key(&(i as i32)) {
                res[index] = i as i32;
                index += 1;
            } else {
                if map.get(&(i as i32)).unwrap().borrow().in_degree == 0 {
                    queue.push_back(map.get(&(i as i32)).unwrap().clone());
                }
            }
        }
        let mut count = 0;
        while !queue.is_empty() {
            let cur = queue.pop_front().unwrap();
            count += 1;
            res[index] = cur.borrow().course;
            index += 1;
            for node in cur.borrow().next.iter() {
                node.borrow_mut().in_degree -= 1;
                if node.borrow().in_degree == 0 {
                    queue.push_back(node.clone());
                }
            }
        }
        if count == size {
            res
        } else {
            vec![]
        }
    }
}

struct Node {
    in_degree: i32,
    course: i32,
    next: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    pub fn new(course: i32) -> Self {
        Self {
            in_degree: 0,
            course,
            next: vec![],
        }
    }

    pub fn construct(prerequisites: &Vec<Vec<i32>>) -> HashMap<i32, Rc<RefCell<Node>>> {
        let mut map = HashMap::new();
        for courses in prerequisites.iter() {
            let to = courses[0];
            let from = courses[1];
            map.entry(to)
                .or_insert(Rc::new(RefCell::new(Node::new(to))))
                .borrow_mut()
                .in_degree += 1;
            let to = map.get(&to).unwrap().clone();
            map.entry(from)
                .or_insert(Rc::new(RefCell::new(Node::new(from))))
                .borrow_mut()
                .next
                .push(to);
        }
        map
    }
}

#[test]
fn test() {
    // let num_courses = 2;
    // let prerequisites: Vec<Vec<i32>> = vec![vec![1, 0]];
    // let res = vec![0, 1];
    // assert_eq!(Solution::find_order(num_courses, prerequisites), res);
    // let num_courses = 4;
    // let prerequisites: Vec<Vec<i32>> = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
    // let res = vec![0, 1, 2, 3];
    // assert_eq!(Solution::find_order(num_courses, prerequisites), res);
    assert_eq!(Solution::find_order(3, vec![vec![1, 0]]), vec![2, 0, 1]);
}
