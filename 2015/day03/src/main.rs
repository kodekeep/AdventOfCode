use std::collections::HashSet;
use std::fs;

fn main() -> std::io::Result<()> {
    let data = fs::read_to_string("src/input.txt")?;
    let mut x = 0;
    let mut y = 0;

    // Part A

    let mut part_a: HashSet<(i32, i32)> = HashSet::new();
    part_a.insert((x, y));
    for c in data.chars() {
        match c {
            '>' => x += 1,
            '<' => x -= 1,
            '^' => y += 1,
            'v' => y -= 1,
            _ => (),
        }
        part_a.insert((x, y));
    }

    println!("Part A\n: {}", part_a.len());

    // Part B
    // https://www.youtube.com/watch?v=eXXgRynkxzU

    let mut part_b = HashSet::new();
    //part_b.insert((0, 0));
    let mut x = [0, 0];
    let mut y = [0, 0];
    let mut which = 0;
    for c in data.chars() {
        part_b.insert((x[which], y[which]));
        match c {
            '>' => x[which] += 1,
            '<' => x[which] -= 1,
            '^' => y[which] += 1,
            'v' => y[which] -= 1,
            _ => (),
        }
        which = 1 - which;
    }

    part_b.insert((x[which], y[which]));
    println!("Part B\n: {}", part_b.len());
    Ok(())
} // Part B
