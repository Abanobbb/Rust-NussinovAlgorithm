pub fn visualize_ascii_art(seq: &str, pairings: &Vec<(usize, usize)>) {
    // Determine the height of the ASCII art
    let height = pairings.iter().map(|&(i, j)| j - i).max().unwrap_or(0) + 1;
    let width = seq.len();
    let mut art = vec![vec![' '; width]; height]; // 2D canvas for ASCII art

    // Populate the 2D canvas with stems and loops
    for &(i, j) in pairings {
        // Vertical lines for stems
        for k in 0..(j - i) {
            art[k][i] = '|';
            art[k][j] = '|';
        }
        // Horizontal line at the bottom of a stem
        art[j - i][i + 1..j].fill('_');
    }

    // Print each line of the ASCII art
    println!("Sequence:   {}", seq);
    for row in art.iter().rev() { // Reverse to have the bottom line printed last
        let line: String = row.iter().collect();
        println!("{}", line);
    }
}


pub fn visualize_verbose_ascii_art(seq: &str, pairings: &Vec<(usize, usize)>) {
    let height = pairings.iter().map(|&(i, j)| j - i).max().unwrap_or(0) + 1;
    let width = seq.len();
    let mut art = vec![vec![' '; width]; height];

    // Function to determine the loop symbol based on the loop size
    let loop_symbol = |size: usize| -> char {
        match size {
            0..=1 => '.',
            2..=4 => 'o',
            5..=8 => 'O',
            9..=12 => '0',
            13..=20 => '@',
            _ => '#',
        }
    };

    // Function to determine the base pair symbol based on the nucleotides
    let base_pair_symbol = |left: char, right: char| -> char {
        match (left, right) {
            ('G', 'C') | ('C', 'G') => '=',
            ('A', 'U') | ('U', 'A') => '-',
            ('G', 'U') | ('U', 'G') => '┄',
            _ => '.',
        }
    };

    for &(i, j) in pairings {
        let loop_size = j - i - 1;
        let symbol = loop_symbol(loop_size);

        for k in 0..height {
            if k < loop_size {
                art[k][i] = '|';
                art[k][j] = '|';
            } else if k == loop_size {
                art[k][i] = base_pair_symbol(seq.as_bytes()[i] as char, seq.as_bytes()[j] as char);
                art[k][j] = base_pair_symbol(seq.as_bytes()[i] as char, seq.as_bytes()[j] as char);
                if loop_size > 0 {
                    for l in i+1..j {
                        art[k][l] = symbol;
                    }
                }
            }
        }
    }

    println!("Sequence: {}", seq);
    for row in art.iter().rev() {
        let line: String = row.iter().collect();
        println!("{}", line);
    }
}



pub fn visualize_compact_rna(seq: &str, pairings: &Vec<(usize, usize)>) {
    let width = seq.len();
    let mut art = vec![vec![' '; width]; 3]; // Reduce the vertical size

    // Function to determine the base pair symbol based on the nucleotides
    let base_pair_symbol = |left: char, right: char| -> char {
        match (left, right) {
            ('G', 'C') | ('C', 'G') => '═',
            ('A', 'U') | ('U', 'A') => '─',
            ('G', 'U') | ('U', 'G') => '┄',
            _ => '─',
        }
    };

    // Draw horizontal lines for base pairs
    for &(i, j) in pairings {
        let symbol = base_pair_symbol(seq.as_bytes()[i] as char, seq.as_bytes()[j] as char);
        art[1][i] = '╭';
        art[1][j] = '╮';
        for k in i+1..j {
            art[1][k] = symbol;
        }
    }

    // Draw vertical lines for base stacking
    for &(i, j) in pairings {
        if i > 0 && art[1][i-1] != '╭' {
            art[0][i] = '│';
        }
        if j < width - 1 && art[1][j+1] != '╮' {
            art[2][j] = '│';
        }
    }

    for row in art.iter().rev() {
        let line: String = row.iter().collect();
        println!("{}", line);
    }
}

