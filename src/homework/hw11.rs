use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..99)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, usize, i32) {
    data.windows(2)
        .enumerate()
        .map(|(i, w)| (i, i + 1, w[0] + w[1]))
        .min_by_key(|&(_, _, sum)| sum)
        .unwrap()
}

fn print_result(data: &[i32]) {
    let (idx1, idx2, min_sum) = min_adjacent_sum(data);
    
    println!("indexes: {}", (0..data.len()).map(|i| format!("{:2}.", i)).collect::<Vec<_>>().join(" "));
    println!("data:   [{}]", data.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(", "));
    println!("indexes: {}\\__ __/{}", " ".repeat(idx1 * 3), " ".repeat((data.len() - idx2 - 1) * 3));
    println!("min adjacent sum={}+{}={} at indexes:{},{}", data[idx1], data[idx2], min_sum, idx1, idx2);
}

fn main() {
    let data = gen_random_vector(20);
    print_result(&data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_adjacent_sum() {
        let data = vec![45, 87, 49, 64, 50, 37, 45, 72, 55, 64, 90, 86, 60, 54, 78, 72, 83, 44, 89, 22];
        let (idx1, idx2, sum) = min_adjacent_sum(&data);
        assert_eq!((idx1, idx2, sum), (5, 6, 82));
    }
}
