use std::{
    fs::File,
    io::{BufRead as _, BufReader},
    path::{Path, PathBuf},
};

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    file: PathBuf,
}

fn read_csv(path: impl AsRef<Path>) -> Vec<(String, Vec<f64>)> {
    // Open the csv file for reading
    let reader = BufReader::new(File::open(path).expect("File not found"));

    // Create an iterator over the lines
    let mut lines = reader.lines();

    // Take the first line and split it into a vector of labels
    let mut result = lines
        .next()
        .expect("No labels")
        .expect("Failed to read first line")
        .split(',')
        .map(|label| (label.trim().to_owned(), vec![]))
        .collect::<Vec<_>>();

    // Read every line and add each value to the correct label
    for line in lines {
        // Panic if the line couldn't be read
        let line = line.expect("Failed to read line");

        // Make sure the line contains the right amount of columns
        assert_eq!(
            line.split(',').count(),
            result.len(),
            "Not enough values on a line"
        );

        // Add each value to the right column
        for (column, value) in result
            .iter_mut()
            .map(|(_, values)| values)
            .zip(line.split(','))
        {
            column.push(value.parse().expect("Value is not a number!"));
        }
    }

    // Return the result
    result
}

fn main() {
    // Parse the arguments
    let args = Args::parse();

    // Read and parse the csv file
    let content = read_csv(args.file);

    // Print the first 5 values for every column
    for (label, values) in content {
        println!("{label}: {:?}", &values[..5]);
    }
}
