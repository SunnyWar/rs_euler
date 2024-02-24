use itertools::Itertools;
use num_bigint::BigUint;
use num_traits::Zero;
use num_traits::One;

pub fn generate_combinations(n: usize, max_value: u32) -> Box<dyn Iterator<Item = Vec<u32>>> {
    let data: Vec<_> = (0..max_value).collect();
    Box::new(data.into_iter().combinations(n))
}

pub fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

pub fn factoria_biguint(n: &BigUint) -> BigUint {
    if *n == Zero::zero() {
        BigUint::one()
    } else {
        let sub = n - BigUint::one();
        n * factoria_biguint(&sub)
    }
}