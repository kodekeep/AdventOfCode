// Andventure of Code 2015 Day 1

fn main() -> std::io::Result<()> {
    let data = std::fs::read_to_string("src/data.txt")?;
    // Part 1
    let mut floor = 0;
    for c in data.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
    }
    println!("Santa has to go to floor {}", floor);

    Ok(())
}
