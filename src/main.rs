mod algorithms;
mod reader;
mod visualization;

use algorithms::nussinov::*;
use reader::read_fastq;
use visualization::ascii::*;
use visualization::dot_bracket::*;

use std::env;
use std::io::{self, Error, ErrorKind};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Please provide an RNA sequence or a path to a FASTQ file.",
        ));
    }

    let input = &args[1];
    let sequences: Vec<String>;

    // Check if the input is a valid RNA sequence
    if input.chars().all(|c| matches!(c, 'A' | 'U' | 'G' | 'C')) 
    {
        // Treat input as RNA sequence
        sequences = vec![input.clone()];
    } else {
        // Treat input as FASTQ file path
        sequences = read_fastq(input)?;
    };

    for seq in sequences {
        let (folding_score, backtrack) = nussinov_rna_folding(&seq);
        let mut pairings = Vec::new();
        reconstruct(&backtrack, 0, seq.len() - 1, &mut pairings);

        println!("Folding score: {}", folding_score);
        visualize_structure_dot_bracket(&seq, &pairings);

        assert_eq!(
            folding_score as usize,
            pairings.len(),
            "The folding score does not match the number of pairings."
        );

        visualize_verbose_ascii_art(&seq, &pairings);
    }

    Ok(())
}
