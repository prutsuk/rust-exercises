/// Returns the nth Fibonacci number (0-indexed).
/// `fibonacci(0) == 0`, `fibonacci(1) == 1`, etc.
pub fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 1..n {
        let tmp = b;
        b = a.saturating_add(b);
        a = tmp;
    }
    b
}

/// Returns `n!` (factorial). Returns `None` on overflow.
pub fn factorial(n: u64) -> Option<u64> {
    let mut result: u64 = 1;
    for i in 2..=n {
        result = result.checked_mul(i)?;
    }
    Some(result)
}

/// Returns the greatest common divisor of `a` and `b` using Euclid's algorithm.
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

/// Returns the least common multiple of `a` and `b`.
/// Returns 0 if either argument is 0.
pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    a / gcd(a, b) * b
}

/// Returns `true` if `n` is a prime number.
pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n < 4 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

/// Returns all prime numbers up to and including `limit` using the Sieve of
/// Eratosthenes.
pub fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return vec![];
    }
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    is_prime
        .into_iter()
        .enumerate()
        .filter_map(|(idx, prime)| if prime { Some(idx) } else { None })
        .collect()
}

/// Computes `base^exp mod modulus`.
/// Returns 0 when `modulus` is 0.
pub fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 0 {
        return 0;
    }
    if modulus == 1 {
        return 0;
    }
    let mut result: u64 = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result.wrapping_mul(base) % modulus;
        }
        exp /= 2;
        base = base.wrapping_mul(base) % modulus;
    }
    result
}
