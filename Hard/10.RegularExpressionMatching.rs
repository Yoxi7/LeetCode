impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();
        let n = s.len();
        let m = p.len();

        let mut dp = vec![vec![false; m + 1]; n + 1];
        dp[0][0] = true;

        
        for j in 2..=m {
            if p_bytes[j - 1] == b'*' {
                dp[0][j] = dp[0][j - 2];
            }
        }

        for i in 1..=n {
            for j in 1..=m {
                if p_bytes[j - 1] == b'*' {
                    
                    dp[i][j] = dp[i][j - 2];
                    
                    let prev = p_bytes[j - 2];
                    if prev == b'.' || prev == s_bytes[i - 1] {
                        dp[i][j] |= dp[i - 1][j];
                    }
                } else if p_bytes[j - 1] == b'.' || p_bytes[j - 1] == s_bytes[i - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                }
            }
        }

        dp[n][m]
    }
}