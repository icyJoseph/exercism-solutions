pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    // start with a list of length upper_bound, filled with true
    // we assume all to be primes and loop over to prove it
    // false means it is not a prime
    let mut sieve: Vec<bool> = (2..=upper_bound).map(|_| true).collect();
    let number_list: Vec<u64> = (2..=upper_bound).collect();
    let length = number_list.len();
    // upper_bound to usize when optimizing the loop

    for (i, number) in number_list.iter().enumerate() {
        // optimization, run only until the sqrt of upper_bound (length)
        if number * number <= upper_bound {
            let is_prime = sieve[i];
            // the index in the sieve + 2 = number
            // can't use number directly because it is u64 and indexes are usize
            // u64 <-> usize is unsafe
            let number_value = i + 2;
            if is_prime {
                // so calculate the non primes ahead by doing:
                //  number_value * (number_value + step)
                // and then calculate the index in the sive by mapping with:
                // x => x - 2
                let non_primes_index_ahead: Vec<usize> = (0..=length)
                    .map(|step| number_value * (number_value + step))
                    .map(|x| x - 2)
                    .collect();

                for index in non_primes_index_ahead {
                    if index < length {
                        sieve[index] = false;
                    }
                }
            }
            continue;
        }
        break;
    }

    let mut primes: Vec<u64> = vec![];

    for (i, is_prime) in sieve.iter().enumerate() {
        if *is_prime {
            primes.push(number_list[i]);
        }
    }

    return primes;
}
