fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s;
    }
    let shift = ((n % len) + len) % len; // Ensure shift is positive within bounds
    let (left, right) = s.split_at(len as usize - shift as usize);
    format!("{}{}", right, left)
}

fn main() {
    let s = "abcdefgh".to_string();
    let rotated = rotate(s.clone(), 2);
    println!("Rotated: {}", rotated);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0, "abcdefgh"),
            (8, "abcdefgh"),
            (-8, "abcdefgh"),
            (1, "habcdefg"),
            (2, "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10, "cdefghab"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(rotate(s.clone(), *n), exp.to_string());
        });
    }
}
