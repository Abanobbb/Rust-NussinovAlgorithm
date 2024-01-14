pub fn match_score(a: char, b: char) -> i32 {
    match (a, b) {
        ('A', 'U') | ('U', 'A') | ('C', 'G') | ('G', 'C') => 1,
        _ => 0,
    }
}



pub fn nussinov_rna_folding(seq: &str) -> (i32, Vec<Vec<Option<usize>>>) {
    let length = seq.len();
    let mut dp = vec![vec![0; length]; length];
    let mut backtrack = vec![vec![None; length]; length];
    let seq_chars: Vec<char> = seq.chars().collect();

    // Fill the DP table and store backtracking information
    for k in 1..length {
        for i in (0..=length - k - 1).rev() {
            let j = i + k;
            let mut max_val = dp[i + 1][j]; // Consider i unpaired
            let mut max_k: Option<usize> = None;

            // Check pairing between 'i' and 'j'
            if match_score(seq_chars[i], seq_chars[j]) > 0 {
                let val = dp[i + 1][j - 1] + 1;
                if val > max_val {
                    max_val = val;
                    max_k = Some(j);
                }
            }

            // Pair 'i' with 'k' where k is in [i+1..j-1]
            for t in i+1..j {
                if match_score(seq_chars[i], seq_chars[t]) > 0 {
                    let possible_val = dp[i + 1][t - 1] + dp[t + 1][j] + 1;
                    if possible_val > max_val {
                        max_val = possible_val;
                        max_k = Some(t);
                    }
                }
            }

            dp[i][j] = max_val;
            backtrack[i][j] = max_k;
        }
    }

    (dp[0][length - 1], backtrack)
}

pub fn reconstruct(backtrack: &Vec<Vec<Option<usize>>>, i: usize, j: usize, pairings: &mut Vec<(usize, usize)>) {
    if i < j {
        if let Some(k) = backtrack[i][j] {
            if k == j {
                // i is paired with j
                pairings.push((i, j));
                reconstruct(backtrack, i + 1, j - 1, pairings);
            } else {
                // i is paired with k and sequence is split
                pairings.push((i, k));
                reconstruct(backtrack, i + 1, k - 1, pairings);
                reconstruct(backtrack, k + 1, j, pairings);
            }
        } else {
            // No pairing for i, move to next
            reconstruct(backtrack, i + 1, j, pairings);
        }
    }
}