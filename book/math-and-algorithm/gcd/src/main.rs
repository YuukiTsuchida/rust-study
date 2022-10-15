use std::io;
use std::io::Write;

fn main() {
    print!("input number> ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("error");
    let value1 = buffer.trim().parse::<i64>().expect("value1 input error");

    print!("input number> ");
    io::stdout().flush().unwrap();
    buffer.clear();
    io::stdin().read_line(&mut buffer).expect("error");
    let value2 = buffer.trim().parse::<i64>().expect("value2 input error");

    let result = gcd(value1, value2);

    println!("gcd = {}", result);
}

fn gcd(a: i64, b: i64) -> i64 {
    //     let mut value1 = a;
    //     let mut value2 = b;
    //
    //     while value1 >= 1 && value2 >= 1 {
    //         if value1 < value2 {
    //             value2 = value2 % value1;
    //         } else {
    //             value1 = value1 % value2;
    //         }
    //     }

    if a <= 1 || b <= 1 {
        if a >= 1 {
            return a;
        } else {
            return b;
        }
    }

    if a < b {
        return gcd(a, b % a);
    } else {
        return gcd(a % b, b);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn gcd_test() {
        let value1: i64 = 123;
        let value2: i64 = 777;

        assert!(crate::gcd(value1, value2) == 3);
    }
}
