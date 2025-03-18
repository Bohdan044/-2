fn find_solutions() -> Vec<(i32, i32, i32)> {
    let mut solutions = Vec::new();
    
    for m in 1..10 {
        for u in 0..10 {
            if m == u { continue; }
            for x in 0..10 {
                if [m, u].contains(&x) { continue; }
                for a in 1..10 {
                    if [m, u, x].contains(&a) { continue; }
                    for s in 1..10 {
                        if [m, u, x, a].contains(&s) { continue; }
                        for l in 0..10 {
                            if [m, u, x, a, s].contains(&l) { continue; }
                            for o in 0..10 {
                                if [m, u, x, a, s, l].contains(&o) { continue; }
                                for n in 0..10 {
                                    if [m, u, x, a, s, l, o].contains(&n) { continue; }
                                    let muxa = m * 1000 + u * 100 + x * 10 + a;
                                    let slon = s * 1000 + l * 100 + o * 10 + n;
                                    if muxa * a == slon {
                                        solutions.push((muxa, a, slon));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    solutions
}

fn main() {
    let solutions = find_solutions();
    for (muxa, a, slon) in &solutions {
        println!("{} x {} = {}", muxa, a, slon);
    }
    println!("Знайдено {} рішень.", solutions.len());
}
