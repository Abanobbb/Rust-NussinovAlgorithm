
# RNA Secondary Structure Prediction and Visualization CLI

## Overview

This command-line tool is designed to predict RNA secondary structures using the Nussinov algorithm. It outputs the structure in a dot-bracket notation and provides a visual representation using ASCII art.


## Current work

- Accepts a FASTQ file containing RNA sequences.
- Applies the Nussinov algorithm to predict the most stable secondary structure.
- Outputs the secondary structure in dot-bracket notation and ASCII


## Features

- **RNA Sequence Input**: The tool can read RNA sequences from FASTQ files.
- **Structure Prediction**: Applies the Nussinov algorithm to predict RNA secondary structures.
- **Folding Score**: Calculates a score that indicates the stability of the predicted structure.
- **Dot-Bracket Notation**: Provides the secondary structure in a readable dot-bracket format.
- **ASCII Visualization**: Transforms the predicted structure into a visual ASCII art form.





## Installation and usage 

 make sure you have Rust and Cargo installed. then:

```bash
# Clone the repository from GitHub
git clone [https://github.com/username/rna-structure-cli.git](https://github.com/Abanobbb/Rust-NussinovAlgorithm.git)
cd Rust-NussinovAlgorithm

# Build the project with Cargo
cargo build --release



## Usage

To run the application, use the following command:

```bash
cargo run  -- path_to_fastq.fastq
```



## Example output

```plaintext

Input RNA Sequence:
AUGGCUACGAUUAGCUACG

Predicted Secondary Structure in Dot-Bracket Notation:
.(((..(((...)))..)))

Number of Base Pairs:
6

ASCII Visualization of the Secondary Structure:
AUGGCUACGAUUAGCUACG
 .===..=---=..===.
  |   |||   |||   
  |   ||└-o-┘||   
  |   ||     ||   
  |   └──O───┘   
  |              
  └─────@────────

```
  


## Visualization Details

The visualization component of the tool translates the RNA sequence and its base pairings into an ASCII representation.

### Symbols in Visualization

Different elements of the structure are depicted using these symbols:

#### Loops

- `.` - Tiny loops with 1-2 nucleotides
- `o` - Small loops of 3-4 nucleotides
- `O` - Medium loops with 5-8 nucleotides
- `0` - Big loops having 9-12 nucleotides
- `@` - Very big loops with 13-20 nucleotides
- `#` - Extremely big loops exceeding 20 nucleotides

#### Base Pairs

- `=` - G-C base pair, indicating a strong bond
- `-` - A-U base pair
- `┄` - G-U base pair, also known as a wobble bond
- `.` - Other non-standard base pairings

### Visualization Example

Given the sequence "GGGAAAUCCCUUU" and base pairings like `[(0, 12), (1, 11), (2, 10), (3, 9), (4, 8)]`, the ASCII art would be:

```plaintext
0         1
0123456789012
GGGAAAUCCCUUU
=┄┄---===┄┄=
| |   | | | |
| └-o-┘ | | |
|       | | |
└──o───┘ | |
          | |
          └─o
