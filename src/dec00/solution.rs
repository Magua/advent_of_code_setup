use std::fs::File;
use std::io::{BufReader, BufRead};

pub fn echo_simple(s: &str) -> &str {
    s
}

pub fn echo_error_handling(s: &str) -> Result<&str, &str> {
    if s.eq("profanity") {
        Err("oh dear")
    } else {
        Ok(echo_simple(s))
    }
}

pub async fn asynch_echo(s: &str) -> Result<&str, &str> {
    echo_error_handling(s)
}

pub async fn count_rows_containing(filepath: &str, s: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut count = 0;

    for line in reader.lines() {
        if line?.contains(s) {
            count += 1;
        }
    }

    Ok(count)
}