use std::collections::VecDeque;

pub fn part1() -> i32 {
    const INPUT: &str = include_str!("../../inputs/6.input");
    solve(4, INPUT).unwrap()
}

pub fn part2() -> i32 {
    const INPUT: &str = include_str!("../../inputs/6.input");
    solve(14, INPUT).unwrap()
}

fn solve(marker_length: usize, input: &str) -> Option<i32> {
    let mut buffer: VecDeque<char> = VecDeque::with_capacity(marker_length);
    let mut count: usize = marker_length + 1;
    for (ix, c) in input.chars().enumerate() {
        count -= 1;

        if count == 0 {
            return Some(ix as i32);
        }

        let at = || {
            let mut i = 0;
            while i < buffer.len() {
                if c == buffer[i] {
                    return Some(i);
                }
                i += 1;
            }
            None
        };
        let pad = marker_length - at().unwrap_or(marker_length);

        if ix < marker_length {
            buffer.push_front(c);
        }

        count = pad.max(count);

        if ix < marker_length {
            continue;
        }

        buffer.pop_back();
        buffer.push_front(c);
    }
    None
}
