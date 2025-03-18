fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let len = shipments.len() as u32;
    
    if total % len != 0 {
        return None; // Неможливо зробити рівномірний розподіл
    }
    
    let average = total / len;
    let mut moves = 0;
    let mut balance = 0;
    
    for &shipment in shipments {
        balance += shipment as i32 - average as i32;
        moves += balance.abs() as usize;
    }
    
    Some(moves)
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let avg = 5; // Базове значення для рівномірного розподілу
    vec![avg; n]
}

fn main() {
    let shipments = vec![8, 2, 2, 4, 4];
    match count_permutation(&shipments) {
        Some(moves) => println!("Мінімальна кількість переміщень: {}", moves),
        None => println!("Рівномірний розподіл неможливий"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_permutation() {
        assert_eq!(count_permutation(&vec![8, 2, 2, 4, 4]), Some(4));
        assert_eq!(count_permutation(&vec![9, 3, 7, 2, 9]), Some(7));
        assert_eq!(count_permutation(&vec![10, 10, 10, 10]), Some(0));
        assert_eq!(count_permutation(&vec![5, 5, 5, 6]), Some(1));
        assert_eq!(count_permutation(&vec![1, 2, 3]), None);
    }
}
