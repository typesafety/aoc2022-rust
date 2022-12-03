pub fn part1() -> i32 {
    const INPUT: &str = include_str!("../../inputs/1.input");
    calc_chunks(INPUT).max().unwrap()
}

fn calc_chunks(input_str: &'_ str) -> impl Iterator<Item = i32> + '_ {
    input_str.split("\n\n").map(|chunk| {
        chunk
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(str::parse::<i32>)
            .map(std::result::Result::unwrap)
            .sum()
    })
}

pub fn part2() -> i32 {
    const INPUT: &str = include_str!("../../inputs/1.input");
    let mut chunks = calc_chunks(INPUT).collect::<Vec<i32>>();
    chunks.sort_unstable_by(|a, b| a.cmp(b).reverse());

    let [e1, e2, e3, ..] = chunks[..] else { unreachable!() };
    e1 + e2 + e3
}
