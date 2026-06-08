use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::errors::AppError;

/// A parsed key=value configuration.
#[derive(Debug, PartialEq)]
pub struct Config {
    pub entries: HashMap<String, String>,
}

/// Read and parse a `KEY=VALUE` config file.
///
/// Blank lines and lines starting with `#` are skipped.
/// Returns an error if the file cannot be read or a line has no `=`.
pub fn load_config(path: &Path) -> Result<Config, AppError> {
    let contents = fs::read_to_string(path)?;
    parse_config(&contents)
}

fn parse_config(text: &str) -> Result<Config, AppError> {
    let mut entries = HashMap::new();
    for (i, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let (key, value) = trimmed.split_once('=').ok_or_else(|| {
            AppError::Validation(format!("line {}: missing '=' delimiter", i + 1))
        })?;
        entries.insert(key.trim().to_string(), value.trim().to_string());
    }
    Ok(Config { entries })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_config() {
        let text = "host = localhost\nport = 8080\n";
        let cfg = parse_config(text).unwrap();
        assert_eq!(cfg.entries.get("host").unwrap(), "localhost");
        assert_eq!(cfg.entries.get("port").unwrap(), "8080");
    }

    #[test]
    fn parse_skips_comments_and_blanks() {
        let text = "# comment\n\nkey = value\n";
        let cfg = parse_config(text).unwrap();
        assert_eq!(cfg.entries.len(), 1);
    }

    #[test]
    fn parse_missing_delimiter() {
        let text = "no_equals_here\n";
        let err = parse_config(text).unwrap_err();
        assert!(err.to_string().contains("missing '=' delimiter"));
    }

    #[test]
    fn load_missing_file() {
        let result = load_config(Path::new("/tmp/nonexistent_config_12345.txt"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("I/O error"));
    }

    #[test]
    fn load_valid_file() {
        let path = Path::new("/tmp/_rust_exercises_test_cfg.txt");
        fs::write(path, "db = postgres\n").unwrap();
        let cfg = load_config(path).unwrap();
        fs::remove_file(path).ok();
        assert_eq!(cfg.entries.get("db").unwrap(), "postgres");
    }
}
