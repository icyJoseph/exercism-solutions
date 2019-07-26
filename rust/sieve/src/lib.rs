pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    // start with a list of length upper_bound, filled with true
    // except for the first two numbers, 0 and 1, the rest
    // are all assumed to be primes
    let mut sieve: Vec<(bool, u64)> = (0..=upper_bound)
        .map(|num| if num < 2 { (false, num) } else { (true, num) })
        .collect();

    // this for loop copies the sieve and loops over it
    // so that we can safely mutate the original sieve
    // i is numerically equal to number, but of type usize!
    for (i, (is_prime, number)) in sieve.to_vec().iter().enumerate() {
        // optimization, run only until the sqrt of upper_bound (length)
        if number * number <= upper_bound {
            if *is_prime {
                // the sieve says, if the current number is prime
                // then i*(i+n) where
                // 0<= n <= upper_bound or until i*(i+n) is greater than upper_bound
                // are all not primes!
                // so we iterate over all positive integers from 0
                // until we have i*(i+n) is greater than upper_bound
                let steps: Vec<usize> = (0..).take_while(|n| i * (i + n) < sieve.len()).collect();

                for step in steps {
                    sieve[i * (i + step)].0 = false;
                }
            }
            continue;
        }
        break;
    }

    let mut primes: Vec<u64> = vec![];

    for (is_prime, number) in sieve {
        if is_prime {
            primes.push(number);
        }
    }

    return primes;
}
