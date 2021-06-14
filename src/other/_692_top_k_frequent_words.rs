use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq)]
struct Pair<'a> {
    word: &'a str,
    times: usize,
}

impl PartialEq for Pair<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word && self.times == other.times
    }
}

impl PartialOrd for Pair<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.times.cmp(&self.times) {        // 这里相反是为了利用系统大根堆组织小根堆
            Ordering::Equal => self.word.cmp(other.word),   // 在词频一样的情况下比较字典序
            other => other,
        }
    }
}

struct Solution;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut hm: HashMap<&str, usize> = HashMap::new();
        let mut heap = BinaryHeap::with_capacity(k as usize);
        for word in words.iter() {
            *hm.entry(&word).or_default() += 1;
        }
        for (word, times) in hm {
            let pair = Pair { word, times };
            if heap.len() == k as usize {
                if pair < *heap.peek().unwrap() {
                    // 通过自定义比较器直接利用大根堆组织小根堆，这里的小于号代表对应小根堆的大于号
                    heap.pop();
                    heap.push(pair);
                }
            } else {
                heap.push(pair);
            }
        }
        let mut res = vec![];
        while let Some(pair) = heap.pop() {
            res.push(pair.word.to_string());
        }
        res.reverse();
        res
    }
}

#[test]
fn test() {
    let words: Vec<String> = vec_string!["i", "love", "leetcode", "i", "love", "coding"];
    let res: Vec<String> = vec_string!["i", "love"];
    let k = 2;
    assert_eq!(Solution::top_k_frequent(words, k), res);
    let words: Vec<String> =
        vec_string!["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"];
    let res: Vec<String> = vec_string!["the", "is", "sunny", "day"];
    let k = 4;
    assert_eq!(Solution::top_k_frequent(words, k), res);
}
