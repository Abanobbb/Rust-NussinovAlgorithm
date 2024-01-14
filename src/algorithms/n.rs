// Define the scoring for matches
pub fn match_score(a: char, b: char) -> i32 {
    match (a, b) {
        ('A', 'U') | ('U', 'A') | ('C', 'G') | ('G', 'C') => 1,
        _ => 0,
    }
}

pub fn nussinov_rna_folding(seq: &str) -> i32 {
    let length = seq.len();
    let mut dp = vec![vec![0; length]; length];
    let seq_chars: Vec<char> = seq.chars().collect();

    // Loop over the size of the gap (k) between i and j
    for k in 1..length {
        // Loop over the starting index i
        for i in 0..length - k {
            // Calculate the ending index j
            let j = i + k;
            // Initialize max_val with the value of not pairing i with j
            let mut max_val = dp[i][j - 1];

            // Loop over all possible splits (t) of the subsequence (i, j)
            for t in i..j {
                // Check if we can pair i with t, and if so, calculate the value
                if match_score(seq_chars[i], seq_chars[t]) > 0 {
                    // Calculate the value of the best valid split of the subsequence
                    let possible_val = dp[i + 1][t - 1] + dp[t + 1][j] + 1;
                    max_val = max_val.max(possible_val);
                }
            }

            // Calculate the value of pairing i with j (if they are a valid pair)
            let paired_val = if match_score(seq_chars[i], seq_chars[j]) > 0 {
                dp[i + 1][j - 1] + 1
            } else {
                0
            };

            // The value for dp[i][j] is the maximum of max_val and paired_val
            dp[i][j] = max_val.max(paired_val);
        }
    }

    // The maximum number of pairs is in the top right corner of the matrix
    dp[0][length - 1]
}

