use std::fs::File;
use std::io::{BufRead, BufReader};

pub async fn count_rows_increased(filepath: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut increased = 0;
    let mut previous = i64::MAX;

    for line in reader.lines() {
        let l = line?;
        let depth = l.parse::<i64>()?;
        if depth > previous {
            increased += 1;
        }
        previous = depth;
        println!("s: {}, d: {}", depth, previous);
    }

    Ok(increased)
}

use sliding_windows::IterExt;
use sliding_windows::Storage;

pub async fn count_windows_increased(filepath: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut storage: Storage<i32> = Storage::new(3);

    let result = reader
        .lines()
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .sliding_windows(&mut storage)
        .fold((0, i32::MAX), |state, w| {
            let (increased, previous) = state;
            let depth: i32 = w.iter().sum();
            if depth > previous {
                (increased + 1, depth)
            } else {
                (increased, depth)
            }
        });

    Ok(result.0)
}
