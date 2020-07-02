pub fn nth(n: u32) -> Option<u32> {
	match n {
		n if n <= 0 => None,
		n => u32::primes().nth(n as usize),
	}
}

trait Prime {
	fn is_prime(self) -> bool;
	fn primes() -> Primes;
}

struct Primes {
	last: Option<u32>,
}

impl Prime for u32 {
	fn is_prime(self) -> bool {
		!(2..self - 1).any(|i| self % i == 0)
	}
	fn primes() -> Primes {
		Primes::new()
	}
}

impl Primes {
	fn new() -> Primes {
		Primes { last: None }
	}
}

impl Iterator for Primes {
	type Item = u32;

	fn next(&mut self) -> Option<u32> {
		let start = match self.last {
			None => 2,
			Some(prime) => prime + 1,
		};

		for n in start..u32::max_value() {
			if n.is_prime() {
				self.last = Some(n);
				return Some(n);
			}
		}
		None
	}
}
