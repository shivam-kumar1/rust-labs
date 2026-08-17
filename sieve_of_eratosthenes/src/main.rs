// https://www.geeksforgeeks.org/dsa/sieve-of-eratosthenes/

const LIMIT: usize = 50;

fn sieve_of_eratosthenes() -> usize {
    let mut num_primes: usize = 0;
    let mut is_prime: [bool; LIMIT + 1] = [true; LIMIT + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=LIMIT {
        if is_prime[i] {
            num_primes += 1;
            for j in (2 * i..=LIMIT).step_by(i) {
                is_prime[j] = false;
            }
        }
    }
    num_primes
}

fn main() {
    let num_primes = sieve_of_eratosthenes();

    println!("Number of primes found: {num_primes}");
}