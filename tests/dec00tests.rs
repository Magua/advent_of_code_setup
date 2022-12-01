use std::error::Error;

use advent_of_code_setup::dec00::solution::*;

#[test]
fn verify_echo_simple() {
    let result = echo_simple("a string");

    assert_eq!(result, "a string");
}

#[test]
fn verify_error_handling_ok() -> Result<(), Box<dyn Error>> {
    let result = echo_error_handling("a string")?;

    assert_eq!(result, "a string");

    Ok(())
}

#[test]
fn verify_error_handling_not_ok() -> Result<(), Box<dyn Error>> {
    let result = echo_error_handling("profanity").unwrap_err();

    assert_eq!(result, "oh dear");

    Ok(())
}

#[tokio::test]
async fn verify_async_ok() -> Result<(), Box<dyn Error>> {
    let result = asynch_echo("a string").await?;

    assert_eq!(result, "a string");

    Ok(())
}

#[tokio::test]
async fn verify_count_rows_containing() -> Result<(), Box<dyn Error>> {
    let result = count_rows_containing("./src/dec00/dec00.txt", "l").await?;

    assert_eq!(result, 2);

    Ok(())
}
