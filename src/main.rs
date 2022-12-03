mod solutions;

use solutions::day01;

use aoc_table::table_gen::TableGen;


fn main() {
    // Run solvers and output to a nice table.
    TableGen::new("AoC 2022 solutions")
        .add(day01::part1, day01::part2)
        .run();
}
