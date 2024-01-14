mod reader;
mod algorithms;

use reader::read_fastq;


use algorithms::nussinov::*;

use std::io::{self, Error, ErrorKind};
use std::env;



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
        let folding_score = nussinov_rna_folding(&seq);
        println!("Folding score for sequence {}: {}", seq, folding_score);
    }

    Ok(())
}
