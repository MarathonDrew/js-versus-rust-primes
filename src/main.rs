use std::time::Instant;

const LIMIT: i64 = 1000000;

fn main() {
    let now = Instant::now();
    let mut primes = vec![1, 2];
    println!("generating primes through {}", LIMIT);
    for i in 3..=LIMIT {
        let mut prime = true;
        for j in 1..primes.len() {
            if i % primes[j] == 0 {
                prime = false;
                break;
            }
        }
        if prime {
            primes.push(i);
        }
    }
    let elapsed = now.elapsed();
    println!("generated primes in {:.2?}", elapsed);
    println!("generated {} primes", primes.len());
}
