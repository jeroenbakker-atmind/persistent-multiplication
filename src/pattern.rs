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
