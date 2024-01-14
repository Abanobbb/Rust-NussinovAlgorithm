

pub fn pairings_to_dot_bracket(length: usize, pairings: &[(usize, usize)]) -> String {
    let mut dot_bracket = vec!['.'; length];
    for &(i, j) in pairings {
        dot_bracket[i] = '(';
        dot_bracket[j] = ')';
    }
    dot_bracket.into_iter().collect()
}

pub fn visualize_structure(seq: &str, pairings: &Vec<(usize, usize)>) {
    let mut structure = vec!['.'; seq.len()];
    for &(i, j) in pairings {
        structure[i] = '(';
        structure[j] = ')';
    }
    println!("Sequence:   {}", seq);
    println!("Structure: {}", structure.into_iter().collect::<String>());
}