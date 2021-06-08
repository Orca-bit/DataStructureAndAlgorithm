use std::rc::Rc;
use std::cell::RefCell;
use std::io;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: TreeLink,
    right: TreeLink,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

type TreeLink = Option<Rc<RefCell<TreeNode>>>;

fn build_tree_from_reader() -> TreeLink {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    let v: Vec<_>  = input.split_ascii_whitespace().collect();
    let mut root = TreeNode::new(v[0].parse().unwrap());
    let left = v[1].parse::<i32>().unwrap();
    let right= v[2].parse::<i32>().unwrap();
    if left != 0 {
        root.left = build_tree_from_reader();
    }
    if right!= 0 {
        root.right = build_tree_from_reader();
    }
    Some(Rc::new(RefCell::new(root)))
}

#[test]
fn test() {
    let root = build_tree_from_reader();
    println!("{:?}", root);
}