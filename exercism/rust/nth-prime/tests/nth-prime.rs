use nth_prime as np;

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), Some(2));
}

#[test]
#[ignore]
fn test_second_prime() {
    assert_eq!(np::nth(1), Some(3));
}

#[test]
#[ignore]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), Some(13));
}

#[test]
#[ignore]
fn test_big_prime() {
    assert_eq!(np::nth(10_000), Some(104_743));
}

#[test]
#[ignore]
fn test_zeroth_prime() {
    assert_eq!(np::nth(0), None);
}
