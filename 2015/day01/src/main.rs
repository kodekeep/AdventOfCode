// Andventure of Code 2015 Day 1

fn main() -> std::io::Result<()> {
    let data = std::fs::read_to_string("src/data.txt")?;
    // Part 1
    let mut floor = 0;
    let mut basement: Vec<usize> = Vec::new();
    for (charidx, c) in data.chars().enumerate() {
        match c {
            '(' => floor += 1,

            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            // charidx is zero based -> charidx + 1
            basement.push(charidx + 1)
        }
    }
    println!("Santa has to go to floor {}", floor);

    // Part 2
    if let Some(val) = basement.iter().min() {
        println!("First character is {}", val)
    };

    Ok(())
}
