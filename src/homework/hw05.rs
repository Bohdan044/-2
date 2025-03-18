fn gcd(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let num1 = 56;
    let num2 = 98;
    println!("GCD of {} and {} is {}", num1, num2, gcd(num1, num2));
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test() {
        let data =
            [
                ((24,  60), 12),
                ((15,   9),  3),
                ((15,   6),  3),
                ((140, 40), 20),
                ((24,  16),  8),
                ((100, 10), 10),
                ((120, 80), 40),
                ((80, 120), 40),
                ((100, 20), 20),
                ((37,  11),  1),
                ((120, 90), 30),
            ];

        for ((a, b), exp) in data.iter() {
            assert_eq!(*exp, gcd(*a, *b));
        }
    }
}
