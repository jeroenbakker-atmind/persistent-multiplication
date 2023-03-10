use std::cmp::Ordering;

pub trait MultiplicationSteps {
    /** Calculate the number of multiplication steps. */
    fn multiplication_steps(&self, ten: &Self, zero: &Self) -> u8;
    fn has_1_digit(&self, ten: &Self) -> bool;
    fn next_multiplication(&self, ten: &Self, zero: &Self) -> Self;
    fn extract_digit(&mut self, ten: &Self) -> Self;
    fn increase(&mut self, one: &Self);
    fn is_same(&self, o: &Self) -> bool;
}

impl<T> MultiplicationSteps for T
where
    T: std::ops::Div<T, Output = T>
        + std::ops::MulAssign<T>
        + std::ops::DivAssign<T>
        + std::ops::Rem<T, Output = T>
        + std::cmp::Ord
        + std::ops::AddAssign<T>
        + Copy,
{
    fn multiplication_steps(&self, ten: &Self, zero: &Self) -> u8 {
        if self.has_1_digit(ten) {
            return 0;
        }

        self.next_multiplication(ten, zero)
            .multiplication_steps(ten, zero)
            + 1
    }

    fn has_1_digit(&self, ten: &Self) -> bool {
        self.cmp(ten) == Ordering::Less
    }

    fn next_multiplication(&self, ten: &T, zero: &T) -> T {
        let mut value = *self;
        let mut result = value.extract_digit(ten);
        while !(value.cmp(zero) == Ordering::Equal) {
            let digit = value.extract_digit(ten);
            result *= digit;
        }
        result
    }

    fn extract_digit(&mut self, ten: &Self) -> Self {
        let digit = *self % *ten;
        *self /= *ten;
        digit
    }

    fn increase(&mut self, one: &Self) {
        *self += *one
    }

    fn is_same(&self, o: &Self) -> bool {
        *self == *o
    }
}

#[cfg(test)]
fn test_persistency<T>(from: &T, to: &T, step: u8, ten: &T, one: &T, zero: &T) -> T
where
    T: MultiplicationSteps + Clone,
{
    let mut value = from.clone();
    while !value.is_same(to) {
        if value.multiplication_steps(ten, zero) == step {
            return value;
        }
        value.increase(one);
    }
    assert!(false);
    return value;
}

#[test]
fn persistent_step_0() {
    let zero = 0 as u32;
    let one = 1 as u32;
    let ten = 10 as u32;
    let from = 1 as u32;
    let to = u32::MAX;
    assert_eq!(test_persistency(&from, &to, 0, &ten, &one, &zero), 1);
}

#[test]
fn persistent_step_1() {
    let zero = 0 as u32;
    let one = 1 as u32;
    let ten = 10 as u32;
    let from = 1 as u32;
    let to = u32::MAX;
    assert_eq!(test_persistency(&from, &to, 1, &ten, &one, &zero), 10);
}

#[test]
fn persistent_step_2() {
    let zero = 0 as u32;
    let one = 1 as u32;
    let ten = 10 as u32;
    let from = 10 as u32;
    let to = u32::MAX;
    assert_eq!(test_persistency(&from, &to, 2, &ten, &one, &zero), 25);
}

#[test]
fn persistent_step_3() {
    let zero = 0 as u32;
    let one = 1 as u32;
    let ten = 10 as u32;
    let from = 25 as u32;
    let to = u32::MAX;
    assert_eq!(test_persistency(&from, &to, 3, &ten, &one, &zero), 39);
}

#[test]
fn persistent_step_4() {
    let zero = 0 as u32;
    let one = 1 as u32;
    let ten = 10 as u32;
    let from = 39 as u32;
    let to = u32::MAX;
    assert_eq!(test_persistency(&from, &to, 4, &ten, &one, &zero), 77);
}

#[test]
fn persistent_step_5() {
    let zero = 0 as u32;
    let one = 1 as u32;
    let ten = 10 as u32;
    let from = 77 as u32;
    let to = u32::MAX;
    assert_eq!(test_persistency(&from, &to, 5, &ten, &one, &zero), 679);
}

#[test]
fn persistent_step_6() {
    let zero = 0 as u32;
    let one = 1 as u32;
    let ten = 10 as u32;
    let from = 679 as u32;
    let to = u32::MAX;
    assert_eq!(test_persistency(&from, &to, 6, &ten, &one, &zero), 6788);
}

#[test]
fn persistent_step_7() {
    let zero = 0 as u32;
    let one = 1 as u32;
    let ten = 10 as u32;
    let from = 6788 as u32;
    let to = u32::MAX;
    assert_eq!(test_persistency(&from, &to, 7, &ten, &one, &zero), 68889);
}

#[test]
fn persistent_step_8() {
    let zero = 0 as u32;
    let one = 1 as u32;
    let ten = 10 as u32;
    let from = 68889 as u32;
    let to = u32::MAX;
    assert_eq!(test_persistency(&from, &to, 8, &ten, &one, &zero), 2677889);
}

#[test]
fn persistent_step_9() {
    let zero = 0 as u32;
    let one = 1 as u32;
    let ten = 10 as u32;
    let from = 2677889 as u32;
    let to = u32::MAX;
    assert_eq!(test_persistency(&from, &to, 9, &ten, &one, &zero), 26888999);
}

#[test]
fn persistent_step_10() {
    let zero = 0 as u32;
    let one = 1 as u32;
    let ten = 10 as u32;
    let from = 26888999 as u32;
    let to = u32::MAX;
    assert_eq!(
        test_persistency(&from, &to, 10, &ten, &one, &zero),
        3778888999
    );
}

#[test]
fn persistent_step_11() {
    let zero = 0 as u64;
    let one = 1 as u64;
    let ten = 10 as u64;
    let from = 277777788888899;
    let to = u64::MAX;
    assert_eq!(
        test_persistency(&from, &to, 11, &ten, &one, &zero),
        277777788888899
    );
}
