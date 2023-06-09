// https://leetcode.com/problems/maximum-performance-of-a-team/solutions/1252582/rust-binaryheap/
use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let (res, _, _) = efficiency
            .iter()
            .zip(speed.iter())
            .map(|(&e, &s)| (Reverse(e as u64), s as u64))
            .collect::<BinaryHeap<_>>()
            .into_sorted_vec()
            .iter()
            .fold((0, 0, BinaryHeap::new()), |(acc, mut sum, mut heap), &(Reverse(e), s)| {
                heap.push(Reverse(s));
                if heap.len() > k as usize {
                    sum -= heap.pop().unwrap().0;
                }
                (acc.max(e * (sum + s)), sum + s, heap)
            });
        (res % 1_000_000_007) as _
    }
}