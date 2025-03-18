const WIDTH: usize = 5;
const HEIGHT: usize = 3;

fn main() {
    let mut output = String::new();
    
    // Верхня частина ромба
    for i in 0..HEIGHT {
        output.push_str(&" ".repeat(HEIGHT - i));
        output.push_str(&"* ".repeat(i + WIDTH - HEIGHT));
        output.push('\n');
    }
    
    // Нижня частина ромба
    for i in (0..HEIGHT - 1).rev() {
        output.push_str(&" ".repeat(HEIGHT - i));
        output.push_str(&"* ".repeat(i + WIDTH - HEIGHT));
        output.push('\n');
    }
    
    print!("{}", output);
}
