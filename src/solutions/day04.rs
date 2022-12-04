use workrange::WorkRange;

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
    const INPUT: &str = include_str!("../../inputs/4.input");
    let pairs = parse_input(INPUT).unwrap_or_default();

    pairs
        .iter()
        .map(|(r1, r2)| r1.overlaps(r2))
        .map(i32::from)
        .sum()
}

mod workrange {
    use std::ops::RangeInclusive;

    pub struct WorkRange {
        _range: RangeInclusive<i32>,
    }

    impl WorkRange {
        pub fn new(start: i32, end: i32) -> Self {
            Self {
                _range: RangeInclusive::new(start, end),
            }
        }

        pub fn start(&self) -> &i32 {
            self._range.start()
        }

        pub fn end(&self) -> &i32 {
            self._range.end()
        }

        pub fn subsumes(&self, victim: &WorkRange) -> bool {
            self.start() <= victim.start() && self.end() >= victim.end()
        }

        pub fn overlaps(&self, victim: &WorkRange) -> bool {
            victim._range.contains(self.start())
                || victim._range.contains(self.end())
                || self._range.contains(victim.start())
                || self._range.contains(victim.end())
        }
    }

    impl std::fmt::Display for WorkRange {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}-{}", self.start(), self.end())
        }
    }
}

fn parse_input(input: &str) -> Result<Vec<(WorkRange, WorkRange)>, ParseError> {
    input
        .split('\n')
        .map(parse_row)
        .collect::<Result<Vec<(WorkRange, WorkRange)>, ParseError>>()
}

fn parse_row(input: &str) -> Result<(WorkRange, WorkRange), ParseError> {
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

fn parse_range(input: &str) -> Result<WorkRange, ParseError> {
    match &input.split('-').collect::<Vec<&str>>()[..] {
        [start, end] => {
            let start = start.parse::<i32>()?;
            let end = end.parse::<i32>()?;
            Ok(WorkRange::new(start, end))
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
