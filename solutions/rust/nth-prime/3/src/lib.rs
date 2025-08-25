pub fn nth(n: u32) -> u32 {
    PrimeNumbers::new().nth(n as usize).unwrap()
}

pub struct PrimeNumbers {
    generated: Vec<u32>,
}

impl PrimeNumbers {
    pub fn new() -> Self {
        Self { generated: Vec::new() }
    }
}

impl Iterator for PrimeNumbers {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut number = *self.generated.last().unwrap_or(&1);
        loop {
            number += 1;
            if is_prime(number) {
                self.generated.push(number);
                break Some(number);
            }
        }
    }
}

pub fn is_prime(number: u32) -> bool {
    // by convention, 1 is not prime
    if number == 1 {
        return false;
    }
    // the only even primary number being 2
    if number % 2 == 0 {
        return number == 2;
    }
    let mut divider = 3;
    loop {
        if (divider * divider) > number {
            break true;
        }
        if number % divider == 0 {
            break false;
        }
        divider += 2;
    }
}
