


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

    // Filling the DP table
    for k in 1..length {
        for i in (0..length - k).rev() {
            let j = i + k;

            // Cases 2 and 3: Leaving 'i' or 'j' unpaired (respectively)
            let mut max_val = dp[i + 1][j].max(dp[i][j - 1]);

            // Case 4: Splitting the sequence at all possible 'k' positions
            for t in i + 1..j {
                if match_score(seq_chars[i], seq_chars[t]) > 0 {
                    let possible_val = if t > i + 1 {
                        dp[i + 1][t - 1] + dp[t + 1][j]
                    } else {
                        dp[t + 1][j]
                    } + 1;
                    max_val = max_val.max(possible_val);
                }
            }

            // Case 1: Pairing 'i' with 'j'
            if match_score(seq_chars[i], seq_chars[j]) > 0 {
                max_val = max_val.max(dp[i + 1][j - 1] + 1);
            }

            dp[i][j] = max_val;
        }
    }

    // The maximum number of pairs is in the top right corner of the DP matrix
    dp[0][length - 1]
}