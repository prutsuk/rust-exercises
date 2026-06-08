use crate::errors::AppError;

/// A minimal user record.
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub age: u8,
}

/// Validate and construct a `User`.
///
/// Rules:
/// - `name` must be non-empty and no longer than 64 characters.
/// - `age` must be in 1..=150.
pub fn create_user(name: &str, age: u8) -> Result<User, AppError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(AppError::Validation("name must not be empty".into()));
    }
    if name.len() > 64 {
        return Err(AppError::Validation(
            "name must be 64 characters or fewer".into(),
        ));
    }
    if age == 0 || age > 150 {
        return Err(AppError::Validation(format!(
            "age {age} is outside the valid range 1..=150"
        )));
    }
    Ok(User {
        name: name.to_string(),
        age,
    })
}

/// Find a user by name (case-insensitive).
pub fn find_user<'a>(users: &'a [User], name: &str) -> Result<&'a User, AppError> {
    let lower = name.to_lowercase();
    users
        .iter()
        .find(|u| u.name.to_lowercase() == lower)
        .ok_or_else(|| AppError::NotFound(format!("user '{name}'")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_user() {
        let u = create_user("Alice", 30).unwrap();
        assert_eq!(u.name, "Alice");
        assert_eq!(u.age, 30);
    }

    #[test]
    fn empty_name() {
        let err = create_user("", 25).unwrap_err();
        assert!(err.to_string().contains("name must not be empty"));
    }

    #[test]
    fn whitespace_only_name() {
        let err = create_user("   ", 25).unwrap_err();
        assert!(err.to_string().contains("name must not be empty"));
    }

    #[test]
    fn name_too_long() {
        let long = "a".repeat(65);
        let err = create_user(&long, 25).unwrap_err();
        assert!(err.to_string().contains("64 characters"));
    }

    #[test]
    fn age_zero() {
        let err = create_user("Bob", 0).unwrap_err();
        assert!(err.to_string().contains("outside the valid range"));
    }

    #[test]
    fn find_existing_user() {
        let users = vec![create_user("Alice", 30).unwrap()];
        let found = find_user(&users, "alice").unwrap();
        assert_eq!(found.name, "Alice");
    }

    #[test]
    fn find_missing_user() {
        let users = vec![create_user("Alice", 30).unwrap()];
        let err = find_user(&users, "Bob").unwrap_err();
        assert!(err.to_string().contains("not found: user 'Bob'"));
    }
}
