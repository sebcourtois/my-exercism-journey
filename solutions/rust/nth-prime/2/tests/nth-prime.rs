use nth_prime as np;

const VALID_PRIME_NUMBERS: [u32; 25] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47,
    53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
];

#[test]
fn test_is_prime() {
    for each_num in 0..100 {
        println!("{}", each_num);
        let is_prime = np::is_prime(each_num);
        assert_eq!(VALID_PRIME_NUMBERS.contains(&each_num), is_prime);
    }
}

#[test]
fn test_prime_iterator() {
    let mut prime_nums_iter = np::PrimeNumbers::new();
    for test_prime_num in VALID_PRIME_NUMBERS.iter() {
        let gen_prime_num = prime_nums_iter.next().unwrap();
        assert_eq!(gen_prime_num, *test_prime_num);
    }
}


#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]
fn test_big_prime() {
    assert_eq!(np::nth(10_000), 104_743);
}
