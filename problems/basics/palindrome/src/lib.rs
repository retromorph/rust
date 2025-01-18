#![forbid(unsafe_code)]

fn reverse_number(mut number: u64) -> u64 {
    let mut reversed = 0;
    while number > 0 {
        let digit = number % 10;
        number /= 10;
        reversed = reversed * 10 + digit
    }

    reversed
}

pub fn is_palindrome(number: u64) -> bool {
    number == reverse_number(number)
}
