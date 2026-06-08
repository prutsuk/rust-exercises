use rust_exercises::math::*;

#[test]
fn fibonacci_base_cases() {
    assert_eq!(fibonacci(0), 0);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
}

#[test]
fn fibonacci_sequence() {
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(4), 3);
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(10), 55);
    assert_eq!(fibonacci(20), 6765);
}

#[test]
fn fibonacci_large() {
    assert_eq!(fibonacci(50), 12586269025);
}

#[test]
fn factorial_base_cases() {
    assert_eq!(factorial(0), Some(1));
    assert_eq!(factorial(1), Some(1));
}

#[test]
fn factorial_small() {
    assert_eq!(factorial(5), Some(120));
    assert_eq!(factorial(10), Some(3628800));
    assert_eq!(factorial(12), Some(479001600));
}

#[test]
fn factorial_overflow() {
    assert_eq!(factorial(21), None);
}

#[test]
fn gcd_basic() {
    assert_eq!(gcd(12, 8), 4);
    assert_eq!(gcd(54, 24), 6);
    assert_eq!(gcd(17, 13), 1);
}

#[test]
fn gcd_with_zero() {
    assert_eq!(gcd(0, 5), 5);
    assert_eq!(gcd(5, 0), 5);
    assert_eq!(gcd(0, 0), 0);
}

#[test]
fn gcd_same_number() {
    assert_eq!(gcd(7, 7), 7);
}

#[test]
fn lcm_basic() {
    assert_eq!(lcm(4, 6), 12);
    assert_eq!(lcm(3, 5), 15);
    assert_eq!(lcm(12, 18), 36);
}

#[test]
fn lcm_with_zero() {
    assert_eq!(lcm(0, 5), 0);
    assert_eq!(lcm(5, 0), 0);
}

#[test]
fn lcm_same_number() {
    assert_eq!(lcm(7, 7), 7);
}

#[test]
fn is_prime_small() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(!is_prime(4));
    assert!(is_prime(5));
}

#[test]
fn is_prime_larger() {
    assert!(is_prime(97));
    assert!(is_prime(101));
    assert!(!is_prime(100));
    assert!(!is_prime(99));
}

#[test]
fn is_prime_even_and_multiples_of_three() {
    assert!(!is_prime(6));
    assert!(!is_prime(9));
    assert!(!is_prime(25));
    assert!(!is_prime(49));
}

#[test]
fn sieve_small() {
    assert_eq!(sieve_of_eratosthenes(0), vec![]);
    assert_eq!(sieve_of_eratosthenes(1), vec![]);
    assert_eq!(sieve_of_eratosthenes(2), vec![2]);
    assert_eq!(sieve_of_eratosthenes(10), vec![2, 3, 5, 7]);
}

#[test]
fn sieve_medium() {
    let primes = sieve_of_eratosthenes(30);
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
}

#[test]
fn mod_pow_basic() {
    assert_eq!(mod_pow(2, 10, 1000), 24);
    assert_eq!(mod_pow(3, 4, 5), 1);
    assert_eq!(mod_pow(2, 0, 7), 1);
}

#[test]
fn mod_pow_edge_cases() {
    assert_eq!(mod_pow(5, 3, 0), 0);
    assert_eq!(mod_pow(5, 3, 1), 0);
}
