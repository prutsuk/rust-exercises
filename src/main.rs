use rust_exercises::{collections, math, strings, validation};

fn main() {
    println!("=== Rust Exercises ===\n");

    println!("-- Math --");
    println!("fibonacci(10) = {}", math::fibonacci(10));
    println!("factorial(10) = {:?}", math::factorial(10));
    println!("gcd(54, 24) = {}", math::gcd(54, 24));
    println!("is_prime(97) = {}", math::is_prime(97));
    println!("primes up to 30: {:?}", math::sieve_of_eratosthenes(30));

    println!("\n-- Strings --");
    println!("reverse(\"hello\") = {}", strings::reverse("hello"));
    println!(
        "is_palindrome(\"racecar\") = {}",
        strings::is_palindrome("racecar")
    );
    println!(
        "caesar_cipher(\"Hello\", 13) = {}",
        strings::caesar_cipher("Hello", 13)
    );
    println!(
        "count_vowels(\"hello\") = {}",
        strings::count_vowels("hello")
    );

    println!("\n-- Collections --");
    let data = vec![5, 3, 8, 1, 2];
    let mut sortable = data.clone();
    collections::merge_sort(&mut sortable);
    println!("merge_sort({data:?}) = {sortable:?}");
    println!(
        "binary_search([1,3,5,7,9], 5) = {:?}",
        collections::binary_search(&[1, 3, 5, 7, 9], &5)
    );

    println!("\n-- Validation --");
    println!(
        "is_valid_email(\"user@example.com\") = {}",
        validation::is_valid_email("user@example.com")
    );
    println!(
        "password_strength(\"Abcdef1!\") = {}",
        validation::password_strength("Abcdef1!")
    );
    println!(
        "is_balanced(\"{{[()]}}\") = {}",
        validation::is_balanced("{[()]}")
    );
    println!(
        "is_valid_ipv4(\"192.168.1.1\") = {}",
        validation::is_valid_ipv4("192.168.1.1")
    );
}
