use crate::errors::AppError;

/// Parse a whitespace-separated list of integers, propagating parse failures.
pub fn parse_number_list(input: &str) -> Result<Vec<i64>, AppError> {
    input
        .split_whitespace()
        .map(|tok| tok.parse::<i64>().map_err(AppError::from))
        .collect()
}

/// Return the average of a non-empty slice, or an error if the slice is empty.
pub fn safe_average(nums: &[i64]) -> Result<f64, AppError> {
    if nums.is_empty() {
        return Err(AppError::Validation("cannot average an empty list".into()));
    }
    let sum: i64 = nums.iter().sum();
    Ok(sum as f64 / nums.len() as f64)
}

/// Look up a value by index, returning a descriptive error on out-of-bounds.
pub fn checked_index(nums: &[i64], idx: usize) -> Result<i64, AppError> {
    nums.get(idx).copied().ok_or_else(|| {
        AppError::NotFound(format!("index {idx} out of range (len={})", nums.len()))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_numbers() {
        let nums = parse_number_list("1 2 3").unwrap();
        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn parse_invalid_token() {
        let err = parse_number_list("1 two 3").unwrap_err();
        assert!(err.to_string().contains("parse error"));
    }

    #[test]
    fn average_nonempty() {
        let avg = safe_average(&[10, 20, 30]).unwrap();
        assert!((avg - 20.0).abs() < f64::EPSILON);
    }

    #[test]
    fn average_empty() {
        let err = safe_average(&[]).unwrap_err();
        assert!(err.to_string().contains("empty list"));
    }

    #[test]
    fn index_in_bounds() {
        assert_eq!(checked_index(&[10, 20, 30], 1).unwrap(), 20);
    }

    #[test]
    fn index_out_of_bounds() {
        let err = checked_index(&[10, 20], 5).unwrap_err();
        assert!(err.to_string().contains("index 5 out of range"));
    }
}
