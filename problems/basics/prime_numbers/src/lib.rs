#![forbid(unsafe_code)]

fn get_upper_bound(n: usize) -> usize {
    let n_float = n as f32;
    (n_float * (n_float.ln() + n_float.ln().ln()) + 10.0).ceil() as usize
}

pub fn get_n_prime_numbers(n: usize) -> Vec<usize> {
    if n == 0 {
        return vec![];
    }

    if n == 1 {
        return vec![2];
    }

    let upper_bound = get_upper_bound(n);
    let mut sieve = vec![true; upper_bound];
    let mut primes = Vec::with_capacity(size_of::<usize>() * n);
    primes.push(2);

    for i in (3..upper_bound).step_by(2) {
        if sieve[i] {
            primes.push(i);

            let mut j = i * i;
            while j < upper_bound {
                sieve[j] = false;
                j += i;
            }
        }

        if primes.len() == n {
            break;
        }
    }

    primes
}
