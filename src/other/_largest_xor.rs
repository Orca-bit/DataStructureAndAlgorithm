#[derive(Default)]
struct TrieNode {
    next: [Option<Box<TrieNode>>; 2],
}

impl TrieNode {
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Default)]
struct NumTrie {
    head: TrieNode,
}

impl NumTrie {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, num: i32) {
        let mut cur = &mut self.head;
        for i in (0..32).rev() {
            let path = (num >> i) & 1;
            if cur.next[path as usize].is_none() {
                cur.next[path as usize] = Some(Box::new(TrieNode::new()));
            }
            cur = cur.next[path as usize].as_mut().unwrap();
        }
    }

    fn max_xor(&self, xor: i32) -> i32 {
        let mut cur = &self.head;
        let mut res = 0;
        for i in (0..32).rev() {
            let path = (xor >> i) & 1;
            let mut best = if i == 31 { path } else { path ^ 1 };
            best = if cur.next[best as usize].is_some() {
                best
            } else {
                best ^ 1
            };
            res |= (path ^ best) << i;
            cur = cur.next[best as usize].as_ref().unwrap();
        }
        res
    }
}

fn max_xor_sub_array(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut max = i32::MIN;
    let mut xor = 0;
    let mut num_trie = NumTrie::new();
    num_trie.add(0);
    for &num in nums.iter() {
        xor ^= num;
        max = max.max(num_trie.max_xor(xor));
        num_trie.add(xor);
    }
    max
}

fn max_xor_sub_array_1(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut pre_xor = vec![0; nums.len()];
    pre_xor[0] = nums[0];
    for i in 1..pre_xor.len() {
        pre_xor[i] = pre_xor[i - 1] ^ nums[i];
    }
    let mut res = i32::MIN;
    for j in 0..nums.len() {
        for i in 0..=j {
            res = res.max(if i == 0 {pre_xor[j]} else {pre_xor[j] ^ pre_xor[i - 1]});
        }
    }
    res
}

#[test]
fn test() {
    assert_eq!(max_xor_sub_array(vec![3, -28, -29, 2]), 7);
    assert_eq!(max_xor_sub_array_1(vec![3, -28, -29, 2]), 7);
}
