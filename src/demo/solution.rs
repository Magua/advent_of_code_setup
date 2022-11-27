use crate::dec00::solution::count_rows_containing;

pub async fn reused_count_rows_containing(filepath: &str, s: &str) -> Result<i32, Box<dyn std::error::Error>> {
    count_rows_containing(filepath, s).await
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn verify_reused_count_rows_containing() {
        match aw!(reused_count_rows_containing("./src/dec00/dec00.txt", "l")) {
            Ok(result) => assert_eq!(result, 2),
            Err(r) => panic!("Expected 2, got error: {}", r),
        }
    }
}
