use std::time::Instant;

const LIMIT: i32 = 1000000;
const PRIMES_SIZE: usize = 200000;

fn main() {
    println!("generating primes through {}", LIMIT);
    let now = Instant::now();
    let mut primes = vec![0, 1];
    primes.resize(PRIMES_SIZE, -1);
    for i in 3..=LIMIT {
        let total_primes = primes.len();
        for j in 1..total_primes {
            if i % primes[j] == 0 {
                break;
            }
            if j + 1 == total_primes {
                primes.push(i);
            }
        }
    }
    let elapsed = now.elapsed();
    println!("generated primes in {:.3?}", elapsed);
    println!("generated {} primes", primes.len());
}
