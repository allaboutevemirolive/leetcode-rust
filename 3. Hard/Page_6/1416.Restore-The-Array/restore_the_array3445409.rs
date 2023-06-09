// https://leetcode.com/problems/restore-the-array/solutions/3445409/rust-dp/
impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let k = k as u64;
        let m = 1_000_000_007;

        let mut dp = vec![0; n + 1];
        dp[n] = 1;

        for i in (0..n).rev() {
            if s[i] != b'0' {
                let mut cur = 0;
                for j in i..n {
                    // when k is large enough, cur * 10 may overflow as i32
                    cur = cur * 10 + (s[j] - b'0') as u64;
                    if cur > k {
                        break;
                    }

                    dp[i] = (dp[i] + dp[j + 1]) % m;
                }
            }
        }

        dp[0]
    }
}