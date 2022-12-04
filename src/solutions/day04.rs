pub fn part1() -> i32 {
    const INPUT: &str = include_str!("../../inputs/4.input");
    let pairs = parse_input(INPUT).unwrap_or_default();

    pairs
        .iter()
        .map(|(r1, r2)| r1.subsumes(r2) || r2.subsumes(r1))
        .map(i32::from)
        .sum()
}

pub fn part2() -> i32 {
    0
}

struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn subsumes(&self, victim: &Range) -> bool {
        self.start <= victim.start && self.end >= victim.end
    }
}

fn parse_input(input: &str) -> Result<Vec<(Range, Range)>, ParseError> {
    input
        .split('\n')
        .map(parse_row)
        .collect::<Result<Vec<(Range, Range)>, ParseError>>()
}

fn parse_row(input: &str) -> Result<(Range, Range), ParseError> {
    match &input.split(',').collect::<Vec<&str>>()[..] {
        [range1, range2] => {
            let range1 = parse_range(range1)?;
            let range2 = parse_range(range2)?;
            Ok((range1, range2))
        }
        x => {
            let err_msg = format!("Failed to parse valid row from input: {x:?}");
            Err(ParseError::new(err_msg))
        }
    }
}

fn parse_range(input: &str) -> Result<Range, ParseError> {
    match &input.split('-').collect::<Vec<&str>>()[..] {
        [start, end] => {
            let start = start.parse::<i32>()?;
            let end = end.parse::<i32>()?;
            Ok(Range { start, end })
        }
        x => {
            let err_msg = format!("Failed to parse a range from input: {x:?}");
            Err(ParseError::new(err_msg))
        }
    }
}

#[derive(Debug)]
struct ParseError {
    message: String,
}

impl ParseError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<std::num::ParseIntError> for ParseError {
    fn from(err: std::num::ParseIntError) -> ParseError {
        ParseError {
            message: err.to_string(),
        }
    }
}
