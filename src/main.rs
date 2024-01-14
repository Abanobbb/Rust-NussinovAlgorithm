mod reader;
mod algorithms;
mod visualization;

use reader::read_fastq;
use algorithms::nussinov::*;
use visualization::dot_bracket::*;

use std::io::{self, Error, ErrorKind};
use std::env;



use std::thread;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "No path to FASTQ file provided",
        ));
    }

    let fastq_path = &args[1];
    let sequences = read_fastq(fastq_path)?;

    for seq in sequences {
        // Run the Nussinov RNA folding algorithm and get backtracking information
        let (folding_score, backtrack) = nussinov_rna_folding(&seq);
        
        // Reconstruct the base pairings using backtracking
        let mut pairings = Vec::new();
        reconstruct(&backtrack, 0, seq.len() - 1, &mut pairings);
        
        // Output the results
        println!("Folding score: {}", folding_score);
        //println!("Number of pairings: {}", pairings.len());
        visualize_structure(&seq, &pairings);
        
        // Check if the folding score equals the number of pairings
        assert_eq!(folding_score as usize, pairings.len(), "The folding score does not match the number of pairings.");
    }

    Ok(())
}
