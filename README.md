# RNA Secondary Structure Prediction CLI

## Overview

This is a work-in-progress command-line application for RNA secondary structure prediction using the Nussinov folding algorithm. It takes a FASTQ file as input and outputs the secondary structure in dot-bracket notation along with a visualization.

## Current Capabilities

- Accepts a FASTQ file containing RNA sequences.
- Applies the Nussinov algorithm to predict the most stable secondary structure.
- Outputs the secondary structure in dot-bracket notation.

## Usage

To run the application, use the following command:

```bash
cargo run --release -- path-to-fastq.fastq
```



## Example Output

Upon successful execution, the tool provides the folding score, and the predicted secondary structure in dot-bracket notation.

Folding score: 39
Structure: (.......(..((..)....((.())..)))..........()((..(().).()...(()..)..).(..(.().....(.)..)).)(.)......().....(..)..().....(.(...)).........((.((..()))))..........()(.).(()..(..).(.)(.)..)...()(...).)


# RNA Secondary Structure Visualization

## Introduction

This tool provides a visualization of RNA secondary structures using ASCII art. It translates the RNA sequence and its associated base pairings into a visual representation that highlights the base pair bonds and unpaired nucleotide loops.

## Visualization Symbols

### Loop Symbols

Loops are represented by different ASCII characters based on their size:

- `.` (dot) - Very small loops (1-2 nucleotides).
- `o` (lowercase o) - Small loops (3-4 nucleotides).
- `O` (uppercase O) - Medium-sized loops (5-8 nucleotides).
- `0` (zero) - Large loops (9-12 nucleotides).
- `@` (at symbol) - Very large loops (13-20 nucleotides).
- `#` (hash) - Extremely large loops (>20 nucleotides).

### Base Pair Symbols

Base pairs are visualized with characters that represent the type of bond:

- `=` (equals) - G-C base pair (strong bond).
- `-` (dash) - A-U base pair.
- `┄` (dotted line) - G-U base pair (wobble bond).
- `.` (dot) - Other non-standard base pairings.

## Example Visualization

Given the RNA sequence "GGGAAAUCCCUUU" and base pairings `[(0, 12), (1, 11), (2, 10), (3, 9), (4, 8)]`, the ASCII art visualization would look like this:

```plaintext
0         1
0123456789012
GGGAAAUCCCUUU
=┄┄---===┄┄=
| |   | | | |
| └-O-┘ | | |
|       | | |
└──0───┘ | |
          | |
          └─@
