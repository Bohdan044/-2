const WIDTH: usize = 10; // Задайте ширину конверта
const HEIGHT: usize = 5; // Задайте висоту конверта

fn main() {
    let mut envelope = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1 || x == y || x == WIDTH - y - 1 {
                envelope.push('*');
            } else {
                envelope.push(' ');
            }
        }
        envelope.push('\n');
    }

    print!("{}", envelope);
}
