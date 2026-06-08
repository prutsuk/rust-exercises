/// Validates that an email address has a basic correct structure:
/// `local@domain` where both parts are non-empty, domain contains a dot, and
/// the TLD is at least 2 characters.
pub fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.splitn(2, '@').collect();
    if parts.len() != 2 {
        return false;
    }
    let (local, domain) = (parts[0], parts[1]);
    if local.is_empty() || domain.is_empty() {
        return false;
    }
    if local.starts_with('.') || local.ends_with('.') || local.contains("..") {
        return false;
    }
    let domain_parts: Vec<&str> = domain.rsplitn(2, '.').collect();
    if domain_parts.len() != 2 {
        return false;
    }
    let tld = domain_parts[0];
    let domain_name = domain_parts[1];
    if tld.len() < 2 || domain_name.is_empty() {
        return false;
    }
    domain
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
}

/// Password strength: returns a score 0–4.
/// +1 for length >= 8, +1 for uppercase, +1 for digit, +1 for special char.
pub fn password_strength(password: &str) -> u8 {
    let mut score: u8 = 0;
    if password.len() >= 8 {
        score += 1;
    }
    if password.chars().any(|c| c.is_ascii_uppercase()) {
        score += 1;
    }
    if password.chars().any(|c| c.is_ascii_digit()) {
        score += 1;
    }
    if password
        .chars()
        .any(|c| !c.is_ascii_alphanumeric() && !c.is_whitespace())
    {
        score += 1;
    }
    score
}

/// Returns `true` if the string contains only balanced parentheses `()`, `[]`,
/// `{}`.
pub fn is_balanced(s: &str) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

/// Returns `true` if the string is a valid IPv4 address.
pub fn is_valid_ipv4(s: &str) -> bool {
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 4 {
        return false;
    }
    parts.iter().all(|part| {
        if part.is_empty() || (part.len() > 1 && part.starts_with('0')) {
            return false;
        }
        match part.parse::<u32>() {
            Ok(n) => n <= 255,
            Err(_) => false,
        }
    })
}

/// Validates that a credit-card number passes the Luhn checksum.
/// Non-digit characters (spaces, dashes) are stripped first.
pub fn luhn_check(card: &str) -> bool {
    let digits: Vec<u32> = card
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    if digits.len() < 2 {
        return false;
    }
    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| {
            if i % 2 == 1 {
                let doubled = d * 2;
                if doubled > 9 {
                    doubled - 9
                } else {
                    doubled
                }
            } else {
                d
            }
        })
        .sum();
    sum % 10 == 0
}
