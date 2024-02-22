use std::vec;

const PRIMES: [u64; 168] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397, 401, 409, 419, 421,
    431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541, 547,
    557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659,
    661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
    809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929,
    937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
];

pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u64;
    for &prime in PRIMES.iter().take_while(|&&prime| prime <= sqrt_n) {
        if n % prime == 0 {
            return false;
        }
    }

    let mut i = PRIMES[PRIMES.len() - 1] + 2;
    while i <= sqrt_n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

pub fn is_palindrome(n: i32) -> bool {
    if n % 10 == 0 {
        return false;
    }

    let mut n = n;
    let original = n;
    let mut reversed = 0;

    while n > 0 {
        let digit = n % 10;
        reversed = reversed * 10 + digit;
        n /= 10;
    }

    original == reversed
}

pub fn generate_combinations_of_three(max_value: i32) -> impl Iterator<Item = (u32, u32, u32)> {
    (0..max_value)
        .flat_map(move |i| (0..max_value)
        .flat_map(move |j| (0..max_value)
        .map(move |k| (i as u32, j as u32, k as u32))))
}

pub fn sieve_of_eratosthenes(limit: u64) -> Vec<u64> {
    let sieve = sieve(limit as usize);
    let mut primes = Vec::new();

    for i in 2..=limit {
        if sieve[i as usize] {
            primes.push(i);
        }
    }

    primes
}

// Optimized Sieve of Eratosthenes
fn sieve(n: usize) -> Vec<bool>{

    // Initialize Sieve Array with all elements initially set to True
    let mut sieve_array = vec![true; n+1];

    // Set arr[0] and arr[1] to false, because 0 and 1 are not prime
    sieve_array[0] =  false;
    sieve_array[1] = false;

    // Mark all even numbers as false, except 2
    for i in (4..n+1).step_by(2) {
        sieve_array[i] = false;
    }

    // Traverse from 3 to square root of n
    // If a number is prime, mark all its multiples except number itself as false
    // Optimization : Check numbers only upto square root of n
    let mut i = 3;
    while i*i<= n+1 {
        if sieve_array[i] {

            // Mark all the multiples except number itself as false
            // Optimization : start from i*i, because smaller multiples are already marked
            // Optimization : use 2*i as step, because  we need to check only odd multiples
            for j in (i*i..n+1).step_by(2*i) {
                sieve_array[j] = false;
            }
        }
        // We do not have to check even numbers.
        // So, we increment i by 2
        i+=2;
    }

    return sieve_array;
}