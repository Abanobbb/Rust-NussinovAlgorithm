

extern crate bio;
use bio::io::fastq;
use std::fs::File;
use std::io::{self, Error, ErrorKind};
use std::path::Path;


pub fn read_fastq<P: AsRef<Path>>(path: P) -> Result<Vec<String>, io::Error> {
    let reader = File::open(path)?;
    let records = fastq::Reader::new(reader);
    let mut sequences = Vec::new();

    for result in records.records() {
        let record = result.map_err(|e| Error::new(ErrorKind::Other, e))?;
        sequences.push(std::str::from_utf8(record.seq()).unwrap().to_string());
    }

    Ok(sequences)
}


