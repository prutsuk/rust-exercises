use rust_exercises::strings::*;

#[test]
fn reverse_basic() {
    assert_eq!(reverse("hello"), "olleh");
    assert_eq!(reverse(""), "");
    assert_eq!(reverse("a"), "a");
}

#[test]
fn reverse_with_spaces() {
    assert_eq!(reverse("hello world"), "dlrow olleh");
}

#[test]
fn is_palindrome_true() {
    assert!(is_palindrome("racecar"));
    assert!(is_palindrome("RaceCar"));
    assert!(is_palindrome("A man, a plan, a canal: Panama"));
    assert!(is_palindrome(""));
}

#[test]
fn is_palindrome_false() {
    assert!(!is_palindrome("hello"));
    assert!(!is_palindrome("abc"));
}

#[test]
fn is_palindrome_with_non_alpha() {
    assert!(is_palindrome("Was it a car or a cat I saw?"));
    assert!(is_palindrome("No 'x' in Nixon"));
}

#[test]
fn caesar_cipher_shift_positive() {
    assert_eq!(caesar_cipher("abc", 1), "bcd");
    assert_eq!(caesar_cipher("xyz", 3), "abc");
    assert_eq!(caesar_cipher("Hello, World!", 13), "Uryyb, Jbeyq!");
}

#[test]
fn caesar_cipher_shift_negative() {
    assert_eq!(caesar_cipher("bcd", -1), "abc");
    assert_eq!(caesar_cipher("abc", -3), "xyz");
}

#[test]
fn caesar_cipher_shift_zero() {
    assert_eq!(caesar_cipher("Hello", 0), "Hello");
    assert_eq!(caesar_cipher("Hello", 26), "Hello");
}

#[test]
fn count_vowels_basic() {
    assert_eq!(count_vowels("hello"), 2);
    assert_eq!(count_vowels("AEIOU"), 5);
    assert_eq!(count_vowels("bcdfg"), 0);
    assert_eq!(count_vowels(""), 0);
}

#[test]
fn word_frequency_basic() {
    let freq = word_frequency("the cat sat on the mat");
    assert!(freq.contains(&("the".to_string(), 2)));
    assert!(freq.contains(&("cat".to_string(), 1)));
    assert!(freq.contains(&("mat".to_string(), 1)));
}

#[test]
fn word_frequency_empty() {
    assert_eq!(word_frequency(""), Vec::<(String, usize)>::new());
}

#[test]
fn word_frequency_with_punctuation() {
    let freq = word_frequency("hello, hello! world.");
    assert!(freq.contains(&("hello".to_string(), 2)));
    assert!(freq.contains(&("world".to_string(), 1)));
}

#[test]
fn title_case_basic() {
    assert_eq!(title_case("hello world"), "Hello World");
    assert_eq!(title_case("HELLO WORLD"), "Hello World");
    assert_eq!(title_case(""), "");
}

#[test]
fn title_case_single_word() {
    assert_eq!(title_case("rust"), "Rust");
}

#[test]
fn is_anagram_true() {
    assert!(is_anagram("listen", "silent"));
    assert!(is_anagram("Triangle", "Integral"));
    assert!(is_anagram("", ""));
}

#[test]
fn is_anagram_false() {
    assert!(!is_anagram("hello", "world"));
    assert!(!is_anagram("abc", "abcd"));
}

#[test]
fn is_anagram_ignores_non_alpha() {
    assert!(is_anagram("a b c", "c b a"));
}
