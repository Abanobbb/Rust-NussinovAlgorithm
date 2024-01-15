


pub fn visualize_structure_dot_bracket(seq: &str, pairings: &Vec<(usize, usize)>) {
    let mut structure = vec!['.'; seq.len()];
    for &(i, j) in pairings {
        structure[i] = '(';
        structure[j] = ')';
    }
    println!("Sequence:   {}", seq);
    println!("Structure: {}", structure.into_iter().collect::<String>());
}