const TRIANGLES: usize = 3;

fn main() {
    let mut output = String::new();
    
    (0..TRIANGLES).for_each(|t| {
        (0..=t * 2).for_each(|i| {
            let spaces = " ".repeat(TRIANGLES * 2 - i);
            let stars = "*".repeat(i * 2 + 1);
            output.push_str(&format!("{}{}
", spaces, stars));
        });
    });
    
    // Малюємо стовбур
    let trunk_width = TRIANGLES / 2 * 2 + 1;
    let trunk_spaces = " ".repeat(TRIANGLES * 2 - trunk_width / 2);
    (0..TRIANGLES).for_each(|_| {
        output.push_str(&format!("{}{}
", trunk_spaces, "|".repeat(trunk_width)));
    });
    
    print!("{}", output);
}
