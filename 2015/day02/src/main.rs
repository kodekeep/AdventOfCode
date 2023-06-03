use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    // read the file to a buffer
    let file = File::open("src/data.txt")?;
    let reader = BufReader::new(file);
    let dimensions: Vec<_> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            if line.is_empty() {
                (0, 0, 0)
            } else {
                let indices: Vec<_> = line
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| *c == 'x')
                    .map(|(i, _)| i)
                    .collect();

                let l: u32 = line[0..indices[0]].parse().unwrap();
                let w: u32 = line[indices[0] + 1..indices[1]].parse().unwrap();
                let h: u32 = line[indices[1] + 1..line.len()].parse().unwrap();

                (w, h, l)
            }
        })
        .collect();

    let mut paper = 0;
    let mut ribbon = 0;

    for dim in dimensions {
        // part 1
        let lw = dim.2 * dim.0;
        let wh = dim.0 * dim.1;
        let hl = dim.1 * dim.2;

        let mut area = vec![lw, wh, hl];
        area.sort();
        paper += 3 * area[0] + 2 * area[1] + 2 * area[2];

        // part 2
        let mut rib = vec![dim.0, dim.1, dim.2];
        rib.sort();
        let single_ribbon = 2 * rib[0] + 2 * rib[1];
        let single_bow = rib[0] * rib[1] * rib[2];

        ribbon += single_ribbon + single_bow;
    }
    println!("Paper needed: {:#?}", paper);
    println!("Ribbon needed: {:#?}", ribbon);
    Ok(())
}
