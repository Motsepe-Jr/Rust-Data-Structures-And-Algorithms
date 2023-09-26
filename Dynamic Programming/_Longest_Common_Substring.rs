fn longest_common_substring(m: &str, n: &str) -> usize {
    let str1 = m.as_bytes();
    let str2 = n.as_bytes();

    let mut dp = vec![vec![0; str2.len() + 1]; str1.len() + 1];
    let mut max = 0;


    for i in 1..=str1.len() {
        for j in 1..str2.len() {
            
            if str1[i-1] == str2[j-1] {

                dp[i][j] = dp[i-1][j-1] + 1;
                max = max.max(dp[i][j]);

        }
    }

    max

}

}

fn main() {
    let m = "ABABC";
    let n = "BABCAB";

    let result = longest_common_substring(m, n);
    println!("Longest Common Substring Length: {}", result);
}