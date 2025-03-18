fn gray(n: u8) -> Vec<String> {
    let size = 1 << n; // 2^n
    (0..size)
        .map(|i| format!("{:0width$b}", i ^ (i >> 1), width = n as usize))
        .collect()
}

#[test]
fn test() {
    let test_data = [
        (0, vec!["".to_string()]),
        (1, vec!["0".to_string(), "1".to_string()]),
        (2, vec!["00".to_string(), "01".to_string(), "10".to_string(), "11".to_string()]),
        (3, vec!["000".to_string(), "001".to_string(), "010".to_string(), "011".to_string(), 
                 "100".to_string(), "101".to_string(), "110".to_string(), "111".to_string()]),
        (4, vec!["0000".to_string(), "0001".to_string(), "0010".to_string(), "0011".to_string(), 
                 "0100".to_string(), "0101".to_string(), "0110".to_string(), "0111".to_string(), 
                 "1000".to_string(), "1001".to_string(), "1010".to_string(), "1011".to_string(),
                 "1100".to_string(), "1101".to_string(), "1110".to_string(), "1111".to_string()]),
    ];

    test_data.iter().for_each(|(n, out)| assert_eq!(gray(*n), *out));
}
