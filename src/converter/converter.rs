pub fn decimal_to_bin(decimal: i32) -> String {
    let mut binary: String = String::new();
    let mut n: i32 = decimal;

    if n == 0 {
        return '0'.to_string();
    }

    while n > 0 {
        let reminder: i32 = n % 2;
        binary.push_str(&reminder.to_string());
        n /= 2;
    }

    return binary.chars().rev().collect();
}

pub fn decimal_to_hex(decimal: i32) -> String {
    let mut hex: String = String::new();
    let mut n: i32 = decimal;

    if n == 0 {
        return '0'.to_string();
    }

    while n > 0 {
        let remainder: i32 = n % 16;
        let hex_digit: char = match remainder {
            0..=9 => (b'0' + remainder as u8) as char,
            _ => (b'A' + (remainder-10) as u8) as char,
        };
        hex.push(hex_digit);
        n /= 16;

    }
    return hex.chars().rev().collect();
}

pub fn decimal_to_octal(decimal: i32) -> String {
    let mut octal: String = String::new();
    let mut n: i32 = decimal;

    if n == 0 {
        return '0'.to_string();
    }

    while n > 0 {
        let remainder = n % 8;
        octal.push((b'0' + remainder as u8) as char);
        n /= 8;
    }

    return octal.chars().rev().collect();
}