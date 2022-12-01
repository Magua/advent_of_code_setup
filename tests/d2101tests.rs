
use advent_of_code_setup::d2101::solution::*;
use std::error::Error;

#[tokio::test]
async fn verify_example_input() -> Result<(), Box<dyn Error>> {
    let result = count_rows_increased("./src/d2101/example_input").await?;

    assert_eq!(result, 7);

    Ok(())
}

#[tokio::test]
async fn verify_input() -> Result<(), Box<dyn Error>> {
    let result = count_rows_increased("./src/d2101/input").await?;
    
    assert_eq!(result, 1387);

    Ok(())
}

#[tokio::test]
async fn verify_sliding_window_example() -> Result<(), Box<dyn Error>> {
    let result = count_windows_increased("./src/d2101/example_input").await?;
    
    assert_eq!(result, 5);
    
    Ok(())
}

#[tokio::test]
async fn verify_sliding_window() -> Result<(), Box<dyn Error>> {
    let result = count_windows_increased("./src/d2101/input").await?;
    
    assert_eq!(result, 1362);

    Ok(())
}

