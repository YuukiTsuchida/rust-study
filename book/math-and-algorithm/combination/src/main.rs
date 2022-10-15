use std::io;
use std::io::Write;

fn factorial(value: i32) -> i32 {
    let mut result = 1;
    for i in 1..=value {
        result *= i;
    }

    result
}

#[derive(Debug)]
struct Combination {
    n: i32,
    r: i32,
}

impl Combination {
    fn new(n: i32, r: i32) -> Combination {
        Combination { n, r }
    }

    pub fn get_pattern(&self) -> i32 {
        factorial(self.n) / (factorial(self.r) * factorial(self.n - self.r))
    }
}

fn main() {
    print!("input number> ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("error");
    let n = buffer.trim().parse::<i32>().expect("value1 input error");

    print!("input number> ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("error");
    let r = buffer.trim().parse::<i32>().expect("value1 input error");

    let comb = Combination::new(n, r);

    println!("{}C{} = {}", n, r, comb.get_pattern());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn comb_test() {
        let comb = Combination::new(4, 2);

        assert_eq!(comb.get_pattern(), 6);
    }
}
