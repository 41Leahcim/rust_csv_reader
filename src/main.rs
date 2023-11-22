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
    let reader = BufReader::new(File::open(path).expect("File not found"));
    let mut lines = reader.lines();
    let mut result = lines
        .next()
        .expect("No labels")
        .expect("Failed to read first line")
        .split(',')
        .map(|label| (label.trim().to_owned(), vec![]))
        .collect::<Vec<_>>();
    for line in lines {
        let line = line.expect("Failed to read line");
        assert_eq!(
            line.split(',').count(),
            result.len(),
            "Not enough values on a line"
        );
        for (column, value) in result
            .iter_mut()
            .map(|(_, values)| values)
            .zip(line.split(','))
        {
            column.push(value.parse().expect("Value is not a number!"));
        }
    }
    result
}

fn main() {
    let args = Args::parse();
    let content = read_csv(args.file);
    for (label, values) in content {
        println!("{label}: {:?}", &values[..5]);
    }
}
