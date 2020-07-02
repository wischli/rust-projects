pub fn factors(mut n: u64) -> Vec<u64> {
    let mut vec: Vec<u64> = Vec::new();
    while n > 1 {
        vec.push((2..=n).find(|x| n % x == 0).unwrap());
        n /= vec.last().unwrap();
    }
    vec
}
