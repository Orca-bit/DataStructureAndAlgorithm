use rand::distributions::WeightedIndex;
use rand::prelude::*;

struct Solution {
    rng: ThreadRng,
    rects: Vec<Vec<i32>>,
    size: usize,
    dist: WeightedIndex<i32>,
}

impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let rng = thread_rng();
        let size = rects.len();
        let weights = rects
            .iter()
            .map(|v| (v[2] - v[0] + 1) * (v[3] - v[1] + 1))
            .collect::<Vec<_>>();
        let dist = WeightedIndex::new(weights).unwrap();
        Self {
            rng,
            rects,
            size,
            dist,
        }
    }

    fn pick(&mut self) -> Vec<i32> {
        let rect = &self.rects[self.rng.sample(&self.dist)];
        let x = self.rng.gen_range(rect[0]..=rect[2]);
        let y = self.rng.gen_range(rect[1]..=rect[3]);
        vec![x, y]
    }
}
