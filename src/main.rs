mod solutions;

use solutions::day01;
use solutions::day04;
use solutions::day06;

use aoc_table::table_gen::TableGen;

fn main() {
    // Run solvers and output to a nice table.
    TableGen::new("AoC 2022 solutions")
        .add(day01::part1, day01::part2)
        .add(day01::part1, day01::part2) // padding
        .add(day01::part1, day01::part2) // padding
        .add(day04::part1, day04::part2)
        .add(day04::part1, day04::part2) // padding
        .add(day06::part1, day06::part2)
        .run_day(6);
}
