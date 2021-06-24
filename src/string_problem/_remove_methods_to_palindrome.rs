fn methods(s: String) -> i32 {
    if s.is_empty() {
        return 0;
    }
    let s = s.chars().collect::<Vec<_>>();
    let n = s.len();
    let mut dp = vec![vec![1; n]; n];
    for i in 0..n - 1 {
        if s[i] == s[i + 1] {
            dp[i][i + 1] = 3;
        } else {
            dp[i][i + 1] = 2;
        }
    }
    if n > 2 {
        for i in (0..n - 2).rev() {
            for j in i + 2..n {
                if s[i] != s[j] {
                    dp[i][j] = (dp[i + 1][j] + dp[i][j - 1] - dp[i + 1][j - 1]) % 998244353;
                } else {
                    dp[i][j] = (dp[i + 1][j] + dp[i][j - 1] + 1) % 998244353;
                }
                while dp[i][j] < 0 {
                    dp[i][j] += 998244353;
                }
            }
        }
    }
    dp[0][n - 1]
}

#[test]
fn test() {
    assert_eq!(methods("aab".to_string()), 4);
    assert_eq!(methods("abcde".to_string()), 5);
}

fn main1() {
    let stdin = std::io::stdin();
    let mut s = String::new();
    let _ = stdin.read_line(&mut s);
    println!("{}", methods(s.trim().to_string()));
}
