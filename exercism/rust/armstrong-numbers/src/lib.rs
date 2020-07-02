pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = num.to_string();
    let num_digits = num_str.len() as u32;
    let digits: Vec<u32> = num_str
        .chars()
        .map(|d| d.to_digit(10).unwrap().pow(num_digits))
        .collect();
    digits.iter().sum::<u32>() == num
}
