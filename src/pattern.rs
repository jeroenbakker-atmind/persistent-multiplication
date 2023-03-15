//! Module is used to investigate the pattern between valid and invalid numbers.
//!
//! The result of this investigation would be to consistently determine the next valid
//! number of a valid number. Creating a sequence. This sequence might be useful to
//! pre-process areas where we want to focus on.
use std::ops::Range;

#[test]
fn pattern() {
    let mut value = 0;
    for (times, delta) in rle(&deltas(1..1000000)) {
        let prev = value + 1;
        value += times as u128 * delta;
        if delta == 1 {
            if times == 1 {
                println!(" - valid {times} {value}");
            } else {
                println!(" - valid {times} {prev}..{value}");
            }
        } else {
            println!(" - skipped {delta} {value}");
        }
    }
}

fn rle(deltas: &[u128]) -> Vec<(u8, u128)> {
    let mut result = Vec::new();
    let mut prev = None;
    for delta in deltas {
        match prev {
            None => {
                prev = Some((1, *delta));
            }
            Some((times, prev_delta)) => {
                if prev_delta == *delta {
                    prev = Some((times + 1, *delta));
                } else {
                    result.push((times, prev_delta));
                    prev = Some((1, *delta));
                }
            }
        }
    }
    result.push(prev.unwrap());

    return result;
}

fn deltas(range: Range<u128>) -> Vec<u128> {
    let mut result = Vec::new();

    let mut last_valid = range.start - 1;
    for value in range {
        if is_valid(value) {
            result.push(value - last_valid);
            last_valid = value;
        }
    }

    result
}

// TODO: Use % and; String conversion not needed.
fn is_valid(value: u128) -> bool {
    let value_s = format!("{value}");
    let mut last_digit = -1;
    for ch in value_s.chars() {
        let digit = format!("{ch}").parse::<i8>().unwrap();
        if digit < last_digit {
            return false;
        }

        last_digit = digit;
    }
    true
}
