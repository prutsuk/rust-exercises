/// Reverses a string, preserving grapheme boundaries for ASCII input.
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/// Returns `true` if the string is a palindrome (case-insensitive,
/// ignoring non-alphanumeric characters).
pub fn is_palindrome(s: &str) -> bool {
    let cleaned: Vec<char> = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    let len = cleaned.len();
    if len == 0 {
        return true;
    }
    for i in 0..len / 2 {
        if cleaned[i] != cleaned[len - 1 - i] {
            return false;
        }
    }
    true
}

/// Applies a Caesar cipher shift to alphabetic characters.
/// Non-alphabetic characters are left unchanged.
pub fn caesar_cipher(s: &str, shift: i32) -> String {
    let shift = shift.rem_euclid(26);
    s.chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                let base = b'a';
                (base + ((c as u8 - base) as i32 + shift) as u8 % 26) as char
            } else if c.is_ascii_uppercase() {
                let base = b'A';
                (base + ((c as u8 - base) as i32 + shift) as u8 % 26) as char
            } else {
                c
            }
        })
        .collect()
}

/// Counts the number of vowels (a, e, i, o, u) in a string (case-insensitive).
pub fn count_vowels(s: &str) -> usize {
    s.chars()
        .filter(|c| matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u'))
        .count()
}

/// Counts the occurrences of each word in the input, returning a sorted vector
/// of `(word, count)` pairs. Words are lowercased and split on whitespace.
pub fn word_frequency(s: &str) -> Vec<(String, usize)> {
    let mut map = std::collections::HashMap::new();
    for word in s.split_whitespace() {
        let w = word
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();
        if !w.is_empty() {
            *map.entry(w).or_insert(0) += 1;
        }
    }
    let mut result: Vec<(String, usize)> = map.into_iter().collect();
    result.sort();
    result
}

/// Converts a string to title case: the first letter of every word is
/// uppercased and the rest are lowercased.
pub fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let upper: String = first.to_uppercase().collect();
                    let rest: String = chars.map(|c| c.to_ascii_lowercase()).collect();
                    format!("{upper}{rest}")
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Returns `true` if two strings are anagrams of each other (case-insensitive,
/// ignoring non-alphabetic characters).
pub fn is_anagram(a: &str, b: &str) -> bool {
    let mut count = [0i32; 26];
    for c in a.chars().filter(|c| c.is_ascii_alphabetic()) {
        count[(c.to_ascii_lowercase() as u8 - b'a') as usize] += 1;
    }
    for c in b.chars().filter(|c| c.is_ascii_alphabetic()) {
        count[(c.to_ascii_lowercase() as u8 - b'a') as usize] -= 1;
    }
    count.iter().all(|&x| x == 0)
}
