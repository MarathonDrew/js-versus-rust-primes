function generatePrimes(limit = 1000000) {
    console.log(`generating primes through ${limit}`);
    console.time("generating primes");
    const primes = [1, 2];
    for (let i = 3; i < limit; i += 2) {
        let prime = true;
        for (let j = 1; j < primes.length; j++) {
            if (i % primes[j] === 0) {
                prime = false;
                break;
            }
        }
        if (prime) {
            primes.push(i);
        }
    }
    console.timeEnd("generating primes");
    console.log(`generated ${primes.length} primes`);
}

generatePrimes();
