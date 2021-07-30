struct NumArray {
    pre_sum: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut arr = NumArray { pre_sum: vec![] };
        arr.calculate_pre_sum(&nums);
        arr
    }

    fn calculate_pre_sum(&mut self, nums: &[i32]) {
        self.pre_sum.resize(nums.len(), 0);
        self.pre_sum[0] = nums[0];
        for i in 1..nums.len() {
            self.pre_sum[i] = self.pre_sum[i - 1] + nums[i];
        }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left < 0 || right >= self.pre_sum.len() as i32 || right < left {
            return 0;
        }
        if left == 0 {
            self.pre_sum[right as usize]
        } else {
            self.pre_sum[right as usize] - self.pre_sum[left as usize - 1]
        }
    }
}

#[test]
fn test() {
    let obj = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    assert_eq!(obj.sum_range(0, 2), 1);
    assert_eq!(obj.sum_range(2, 5), -1);
    assert_eq!(obj.sum_range(0, 5), -3);
}
