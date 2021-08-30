use super::util::*;

struct Solution;

impl Solution {
    pub fn minimal_exec_time(root: TreeLink) -> f64 {
        if let Some(data) = Self::process(&root) {
            data.sum - data.par
        } else {
            0.
        }
    }

    fn process(root: &TreeLink) -> Option<ReturnData> {
        if let Some(node) = root {
            let node = node.borrow();
            let left = Self::process(&node.left);
            let right = Self::process(&node.right);
            let (mut sum, mut par) = (node.val as f64, 0.);
            match (left, right) {
                (Some(left_data), Some(right_data)) => {
                    sum += left_data.sum;
                    sum += right_data.sum;
                    let (longer, shorter) = {
                        if left_data.sum >= right_data.sum {
                            (left_data, right_data)
                        } else {
                            (right_data, left_data)
                        }
                    };
                    if longer.sum - longer.par * 2. <= shorter.sum {
                        par = (longer.sum + shorter.sum) / 2.;
                    } else {
                        par = longer.par + shorter.sum;
                    }
                }
                (Some(left_data), None) => {
                    sum += left_data.sum;
                    par = left_data.par;
                }
                (None, Some(right_data)) => {
                    sum += right_data.sum;
                    par = right_data.par;
                }
                (None, None) => {}
            }
            Some(ReturnData::new(sum, par))
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
struct ReturnData {
    sum: f64,
    par: f64,
}

impl ReturnData {
    fn new(sum: f64, par: f64) -> Self {
        Self { sum, par }
    }
}
