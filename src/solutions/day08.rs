#![allow(unused_parens)]

use std::vec::Vec;

pub fn part1() -> i32 {
    const INPUT: &str = include_str!("../../inputs/8.input");
    let grid = &mut parse_grid(INPUT);

    (count_from_left(grid)
        + count_from_right(grid)
        + count_from_top(grid)
        + count_from_bottom(grid))
}

pub fn part2() -> i32 {
    const INPUT: &str = include_str!("../../inputs/8.input");
    let grid = parse_grid(INPUT);

    let mut highest = 0;
    for row in grid.iter() {
        for x in row {
            let score = score(x, &grid);
            if score > highest {
                highest = score;
            }
        }
    }
    highest
}

fn score(xate: &Xate, grid: &Grid) -> i32 {
    // Left
    let mut score_left = 0;
    for col_ix in (0..xate.x()).rev() {
        score_left += 1;
        if grid[xate.y()][col_ix].height >= xate.height {
            break;
        }
    }
    // Right
    let mut score_right = 0;
    for col_ix in (xate.x() + 1)..grid[0].len() {
        if xate.x() == grid[0].len() - 1 {}
        score_right += 1;
        if grid[xate.y()][col_ix].height >= xate.height {
            break;
        }
    }
    // Up
    let mut score_up = 0;
    for row_ix in (0..xate.y()).rev() {
        score_up += 1;
        if grid[row_ix][xate.x()].height >= xate.height {
            break;
        }
    }
    // Down
    let mut score_down = 0;
    for row_ix in (xate.y() + 1)..grid.len() {
        score_down += 1;
        if grid[row_ix][xate.x()].height >= xate.height {
            break;
        }
    }

    score_left * score_right * score_up * score_down
}

fn count_from_top(grid: &mut Grid) -> i32 {
    let mut total = 0;
    for col_ix in 0..grid[0].len() {
        let mut highest = -1;
        let mut count = 0;
        for row_ix in 0..grid.len() {
            let x = &mut grid[row_ix][col_ix];
            if x.height > highest {
                highest = x.height;
                if !x.visited {
                    count += 1;
                    x.visited = true;
                }
            }
        }
        total += count;
    }
    total
}

fn count_from_bottom(grid: &mut Grid) -> i32 {
    let mut total = 0;
    for col_ix in 0..grid[0].len() {
        let mut highest = -1;
        let mut count = 0;
        for row_ix in (0..grid.len()).rev() {
            let x = &mut grid[row_ix][col_ix];
            if x.height > highest {
                highest = x.height;
                if !x.visited {
                    count += 1;
                    x.visited = true;
                }
            }
        }
        total += count;
    }
    total
}

fn count_from_left(grid: &mut Grid) -> i32 {
    let mut total = 0;
    for row in grid {
        let mut highest = -1;
        let mut count = 0;
        for x in row.iter_mut() {
            if x.height > highest {
                highest = x.height;
                if !x.visited {
                    count += 1;
                    x.visited = true;
                }
            }
        }
        total += count;
    }
    total
}

fn count_from_right(grid: &mut Grid) -> i32 {
    let mut total = 0;
    for row in grid {
        let mut highest = -1;
        let mut count = 0;
        for x in row.iter_mut().rev() {
            if x.height > highest {
                highest = x.height;
                if !x.visited {
                    count += 1;
                    x.visited = true;
                }
            }
        }
        total += count;
    }
    total
}

type Grid = Vec<Vec<Xate>>;

#[derive(Debug)]
struct Xate {
    visited: bool,
    point: (usize, usize),
    height: i32,
}

impl Xate {
    fn new(x: usize, y: usize, height: i32) -> Self {
        Self {
            visited: false,
            point: (x, y),
            height,
        }
    }

    fn x(&self) -> usize {
        self.point.0
    }

    fn y(&self) -> usize {
        self.point.1
    }
}

impl std::fmt::Display for Xate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}@({}, {}) (Added: {})",
            self.height,
            self.x(),
            self.y(),
            self.visited
        )
    }
}

fn parse_grid(css: &str) -> Grid {
    css.split('\n')
        .filter(|s| !s.is_empty())
        .enumerate()
        .map(|(row, text)| parse_row(text, row))
        .collect()
}

fn parse_row(cs: &str, y: usize) -> Vec<Xate> {
    cs.chars()
        .enumerate()
        .map(|(col, c)| parse_xate(c, col, y))
        .collect()
}

fn parse_xate(c: char, x: usize, y: usize) -> Xate {
    let height = c
        .to_string()
        .parse()
        .expect("Failed to parse char {c} into digit.");
    Xate::new(x, y, height)
}
