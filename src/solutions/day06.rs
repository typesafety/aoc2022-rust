use std::collections::VecDeque;

pub fn part1() -> i32 {
    const INPUT: &str = include_str!("../../inputs/6.input");
    const MARKER_LENGTH: usize = 4;
    let mut buffer: VecDeque<char> = VecDeque::with_capacity(MARKER_LENGTH);
    let mut count: usize = MARKER_LENGTH + 1;
    for (ix, c) in INPUT.chars().enumerate() {
        count -= 1;

        if ix < MARKER_LENGTH {
            buffer.push_front(c);
        }

        let mut i = 0;
        while i < buffer.len() {
            if c == buffer[i] {
                break;
            }
            i += 1;
        }
        let pad = buffer.len() - i;

        count = std::cmp::max(pad, count);

        if count == 0 {
            return ix as i32;
        }
        buffer.pop_back();
        buffer.push_front(c);
    }
    -1
}

pub fn part2() -> i32 {
    const INPUT: &str = include_str!("../../inputs/6.input");
    const SEQUENCE_LENGTH: usize = 14;
    let mut buffer: VecDeque<char> = VecDeque::with_capacity(SEQUENCE_LENGTH);
    let mut count: usize = SEQUENCE_LENGTH + 1;
    for (ix, c) in INPUT.chars().enumerate() {
        count -= 1;

        if count == 0 {
            return ix as i32;
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
        let pad = SEQUENCE_LENGTH - at().unwrap_or(SEQUENCE_LENGTH);

        if ix < SEQUENCE_LENGTH {
            buffer.push_front(c);
        }

        count = std::cmp::max(pad, count);

        if ix < SEQUENCE_LENGTH {
            continue;
        }

        buffer.pop_back();
        buffer.push_front(c);
    }
    -1
}
