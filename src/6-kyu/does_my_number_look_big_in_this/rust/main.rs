fn narcissistic(num: u64) -> bool {
    let len = (num as f64).log10().floor() as u32 + 1;
    // Fold over the digits, dividing by 10 each time to get the next digit, and summing the digit to the power of the number of digits
    (0..len).fold((num, 0), |(n, sum), _| (n / 10, sum + (n % 10).pow(len))).1 == num
}

fn main() {
    println!("{}", narcissistic(7));
    println!("{}", narcissistic(371));
    println!("{}", narcissistic(122));
    println!("{}", narcissistic(4887));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(input: u64, expected: bool) {
        let actual = narcissistic(input);
        assert_eq!(actual, expected, "\nIncorrect answer for n={}\nExpected: {expected}\nActual: {actual}", input)
    }

    #[test]
    fn basic_tests() {
        dotest(   7,  true);
        dotest( 371,  true);
        dotest( 122, false);
        dotest(4887, false);
    }
}
