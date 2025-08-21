pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let predicate = |x: &u32| {
        factors
            .iter()
            .any(|factor| (factor > &0) && (x % factor == 0))
    };
    (1..limit).filter(|n| predicate(n)).sum()
}
