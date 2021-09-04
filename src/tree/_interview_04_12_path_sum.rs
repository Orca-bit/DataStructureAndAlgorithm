use super::util::*;

struct Solution;

impl Solution {
    pub fn path_sum(root: TreeLink, sum: i32) -> i32 {
        let height = Self::height(&root);
        let mut path = vec![0; height];
        let mut res = 0;
        Self::process(&root, &mut path, 0, sum, &mut res);
        res
    }

    fn process(root: &TreeLink, path: &mut Vec<i32>, level: usize, sum: i32, res: &mut i32) {
        if let Some(node) = root {
            let node = node.borrow();
            path[level] = node.val;
            let mut tmp = 0;
            for i in (0..level + 1).rev() {
                tmp += path[i];
                if tmp == sum {
                    *res += 1;
                }
            }
            Self::process(&node.left, path, level + 1, sum, res);
            Self::process(&node.right, path, level + 1, sum, res);
            // path[level] = 0;
        }
    }

    fn height(root: &TreeLink) -> usize {
        if let Some(node) = root {
            let node = node.borrow();
            1 + Self::height(&node.left).max(Self::height(&node.right))
        } else {
            0
        }
    }
}