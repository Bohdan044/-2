fn is_prime(n: &u32) -> bool {
    if *n < 2 {
        return false;
    }
    if *n == 2 {
        return true;
    }
    if *n % 2 == 0 {
        return false;
    }
    let limit = (*n as f64).sqrt() as u32;
    (3..=limit).step_by(2).all(|i| *n % i != 0)
}

fn main() {
    let num = 29;
    println!("Is {} prime? {}", num, is_prime(&num));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_prime() {
        let test_data = [
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (100, false),
            (10007, true),
        ];
    
        test_data
            .iter()
            .for_each(|(n, prime)|
                assert_eq!(is_prime(n), *prime)
            )
    }
}
