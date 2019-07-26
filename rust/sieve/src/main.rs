use sieve;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_1 = &args[1];
    let target: u64 = String::from(arg_1)
        .trim()
        .parse()
        .expect("Number required!");
    let primes = sieve::primes_up_to(target);
    println!("Found: {} primes", primes.len());
}
