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


## Example Output

Upon successful execution, the tool provides the folding score, and the predicted secondary structure in dot-bracket notation.

Folding score: 39
Structure: (.......(..((..)....((.())..)))..........()((..(().).()...(()..)..).(..(.().....(.)..)).)(.)......().....(..)..().....(.(...)).........((.((..()))))..........()(.).(()..(..).(.)(.)..)...()(...).)
