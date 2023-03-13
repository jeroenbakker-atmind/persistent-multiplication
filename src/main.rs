extern crate vulkano;
extern crate vulkano_shaders;

mod rust_native;
mod vulkan_compute;
mod vulkan_shaders;

/// Is the given range interesting to test.
///
/// - common high values should be in increasing order.
fn is_interesting(from: u128, to: u128) -> bool {
    let from_s = format!("{from}");
    let to_s = format!("{to}");

    let mut index = 0;
    let mut last_digit = -1;
    while from_s.chars().nth(index) == to_s.chars().nth(index) {
        let new_digit = String::from(from_s.chars().nth(index).unwrap())
            .parse::<i32>()
            .unwrap();
        if index != 0 {
            if new_digit < last_digit {
                return false;
            }
        }
        last_digit = new_digit;
        index += 1;
    }
    /* Final check */
    let from_digit = String::from(from_s.chars().nth(index).unwrap())
        .parse::<i32>()
        .unwrap();
    let to_digit = String::from(from_s.chars().nth(index).unwrap())
        .parse::<i32>()
        .unwrap();
    return last_digit <= from_digit && last_digit <= to_digit;
}

fn main() {
    let base = u32::MAX as u128 / 64;
    for i in 68219..u32::MAX {
        let from = base * i as u128;
        let to = from + base;
        if !is_interesting(from, to) {
            continue;
        }
        let result = vulkan_compute::calc_range(from, to);
        if result >= 10 {
            println!("{i}: {from}-{to} = {result}");
        }
    }
}
