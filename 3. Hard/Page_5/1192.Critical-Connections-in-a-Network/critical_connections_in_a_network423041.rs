// https://leetcode.com/problems/critical-connections-in-a-network/solutions/423041/rust-solution/
use std::cmp::min;

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n as usize];
        for edge in connections {
            let (i, j) = (edge[0], edge[1]);
            adj[i as usize].push(j as usize);
            adj[j as usize].push(i as usize);
        }
        let mut pre: Vec<i32> = vec![-1; n as usize];
        let mut low: Vec<i32> = vec![-1; n as usize];
        let mut bridge: Vec<Vec<i32>> = Vec::new();
        Self::dfs(/*count=*/0, 0, 0, &adj, &mut pre, &mut low, &mut bridge);
        bridge
    }
    
    fn dfs(mut count: i32, u: usize, v: usize, adj: &Vec<Vec<usize>>,
           pre: &mut Vec<i32>, low: &mut Vec<i32>, bridge: &mut Vec<Vec<i32>>) {
        count += 1;
        pre[v] = count;
        low[v] = count;
        for &w in adj[v].iter() {
            if pre[w] == -1 {
                Self::dfs(count, v, w, adj, pre, low, bridge);
                if pre[w] == low[w] {
                    bridge.push(vec![v as i32, w as i32]);
                }
            }
            if w != u {
                low[v] = min(low[v], low[w]);
            }
        }
    }
}