use rust_exercises::validation::*;

#[test]
fn is_valid_email_true() {
    assert!(is_valid_email("user@example.com"));
    assert!(is_valid_email("user.name@example.co.uk"));
    assert!(is_valid_email("a@b.cc"));
}

#[test]
fn is_valid_email_false_no_at() {
    assert!(!is_valid_email("userexample.com"));
}

#[test]
fn is_valid_email_false_no_domain() {
    assert!(!is_valid_email("user@"));
    assert!(!is_valid_email("@example.com"));
}

#[test]
fn is_valid_email_false_short_tld() {
    assert!(!is_valid_email("user@example.c"));
}

#[test]
fn is_valid_email_false_no_dot_in_domain() {
    assert!(!is_valid_email("user@localhost"));
}

#[test]
fn is_valid_email_false_leading_dot_in_local() {
    assert!(!is_valid_email(".user@example.com"));
}

#[test]
fn is_valid_email_false_consecutive_dots() {
    assert!(!is_valid_email("user..name@example.com"));
}

#[test]
fn password_strength_empty() {
    assert_eq!(password_strength(""), 0);
}

#[test]
fn password_strength_short_lower() {
    assert_eq!(password_strength("abc"), 0);
}

#[test]
fn password_strength_long_lower() {
    assert_eq!(password_strength("abcdefgh"), 1);
}

#[test]
fn password_strength_long_mixed() {
    assert_eq!(password_strength("Abcdefgh"), 2);
}

#[test]
fn password_strength_full() {
    assert_eq!(password_strength("Abcdef1!"), 4);
}

#[test]
fn password_strength_digit_only_short() {
    assert_eq!(password_strength("12"), 1);
}

#[test]
fn is_balanced_true() {
    assert!(is_balanced("()"));
    assert!(is_balanced("()[]{}"));
    assert!(is_balanced("{[()]}"));
    assert!(is_balanced(""));
    assert!(is_balanced("hello world"));
}

#[test]
fn is_balanced_false() {
    assert!(!is_balanced("(]"));
    assert!(!is_balanced("([)]"));
    assert!(!is_balanced("{"));
    assert!(!is_balanced(")"));
}

#[test]
fn is_balanced_nested() {
    assert!(is_balanced("({[a + b] * c})"));
}

#[test]
fn is_valid_ipv4_true() {
    assert!(is_valid_ipv4("192.168.1.1"));
    assert!(is_valid_ipv4("0.0.0.0"));
    assert!(is_valid_ipv4("255.255.255.255"));
}

#[test]
fn is_valid_ipv4_false() {
    assert!(!is_valid_ipv4("256.0.0.0"));
    assert!(!is_valid_ipv4("1.2.3"));
    assert!(!is_valid_ipv4("1.2.3.4.5"));
    assert!(!is_valid_ipv4(""));
}

#[test]
fn is_valid_ipv4_no_leading_zeros() {
    assert!(!is_valid_ipv4("01.02.03.04"));
    assert!(!is_valid_ipv4("192.168.01.1"));
}

#[test]
fn luhn_check_valid() {
    assert!(luhn_check("4539 1488 0343 6467"));
    assert!(luhn_check("79927398713"));
}

#[test]
fn luhn_check_invalid() {
    assert!(!luhn_check("1234567890"));
    assert!(!luhn_check("79927398710"));
}

#[test]
fn luhn_check_too_short() {
    assert!(!luhn_check("0"));
    assert!(!luhn_check(""));
}

#[test]
fn luhn_check_with_dashes() {
    assert!(luhn_check("4539-1488-0343-6467"));
}
