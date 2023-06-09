// https://leetcode.com/problems/count-of-smaller-numbers-after-self/solutions/1402926/rust-divide-and-conquer-with-binary-index-tree/
use std::collections::{BTreeSet, HashMap};

impl Solution {
    pub fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
        let n: usize = nums.len();
        let mut res: Vec<i32> = vec![0; n];
        let rank: HashMap<i32, usize> = nums
            .clone()
            .drain(..)
            .collect::<BTreeSet<i32>>()
            .iter()
            .enumerate()
            .map(|(index, &num)| (num, index + 1))
            .collect();

        let mut tree = BinaryIndexTree::new(rank.keys().len());

        for i in (0..n).rev() {
            let num = nums[i];
            res[i] = tree.query(rank[&num] - 1);
            tree.update(rank[&num], 1);
        }

        res
    }
}

pub struct BinaryIndexTree {
    n: usize,
    nums: Vec<i32>,
}

impl BinaryIndexTree {
    pub fn new(length: usize) -> BinaryIndexTree {
        BinaryIndexTree {
            n: length + 1,
            nums: vec![0; length + 1],
        }
    }

    pub fn update(&mut self, mut index: usize, delta: i32) {
        while index < self.n {
            self.nums[index] += delta;
            index = (index | (index - 1)) + 1;
        }
    }

    pub fn query(&self, mut index: usize) -> i32 {
        let mut sum: i32 = 0;
        while index > 0 {
            sum += self.nums[index];
            index &= index - 1;
        }
        sum
    }
}