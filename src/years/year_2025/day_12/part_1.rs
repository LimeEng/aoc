#[derive(Debug)]
struct Present {
    size: u64,
}

#[derive(Debug)]
struct Tree {
    width: u64,
    height: u64,
    counts: Vec<u64>,
}

#[must_use]
pub fn solve(input: &str) -> usize {
    let (presents, trees) = parse(input);

    trees
        .iter()
        .filter(|tree| {
            let required_area: u64 = presents
                .iter()
                .zip(&tree.counts)
                .map(|(present, &count)| present.size * count)
                .sum();
            required_area <= tree.width * tree.height
        })
        .count()
}

fn parse(input: &str) -> (Vec<Present>, Vec<Tree>) {
    let (presents, trees) = input.rsplit_once("\n\n").unwrap();

    let presents = presents
        .split("\n\n")
        .map(|present| {
            let size = present
                .lines()
                .skip(1)
                .flat_map(|line| line.chars())
                .filter(|&c| c == '#')
                .count();
            Present {
                size: u64::try_from(size).unwrap(),
            }
        })
        .collect();

    let trees = trees
        .lines()
        .map(|line| {
            let (size, counts) = line.split_once(": ").unwrap();
            let (width, height) = size.split_once('x').unwrap();

            Tree {
                width: width.parse().unwrap(),
                height: height.parse().unwrap(),
                counts: counts
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            }
        })
        .collect();

    (presents, trees)
}
