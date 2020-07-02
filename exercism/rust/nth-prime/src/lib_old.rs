fn is_prime(x: u32, primes: &Vec<u32>) -> bool {
	for prime in primes.iter() {
		if x % prime == 0 {
			return false;
		}
	}
	return true;
}

fn is_prime2(n: u32) -> bool {
	!(2..n - 1).any(|i| n % i == 0)
}

fn next_prime(last: u32, primes: &Vec<u32>) -> u32 {
	let mut x = last;
	while !is_prime(x, primes) {
		x += 1;
	}
	return x;
}

fn get_primes(n: usize) -> Vec<u32> {
	let mut primes = vec![2];
	let mut last_prime = 2;
	let num_primes = primes.len() - 1;
	for _x in num_primes..n {
		let new_prime = next_prime(last_prime, &primes);
		primes.push(new_prime);
		last_prime = new_prime;
	}
	primes
}
