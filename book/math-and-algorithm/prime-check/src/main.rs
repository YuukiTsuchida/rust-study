use std::io;
use std::time::{Duration, Instant};

fn main() {
    println!("input number");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("failed readline");
    println!("input value: {}", &buffer);

    let prime_time = Instant::now();
    for _ in 0..10000 {
        let num = buffer.trim().parse::<i64>();

        if let Ok(input) = num {
            is_prime(input);
        } else {
            println!("parse error: {:?}", num);
        }
    }
    let end = prime_time.elapsed();
    println!("is_prime {}ns", end.as_millis());

    let prime_time = Instant::now();
    for _ in 0..1000 {
        let num = buffer.trim().parse::<i64>();

        if let Ok(input) = num {
            is_prime_fast(input);
        } else {
            println!("parse error: {:?}", num);
        }
    }
    let end = prime_time.elapsed();
    println!("is_prime_fast {}ns", end.as_millis());
}

fn is_prime(n: i64) -> bool {
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn is_prime_fast(n: i64) -> bool {
    let mut start_index = 2;
    while (start_index * start_index) <= n {
        start_index += 1;
        if n % start_index == 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {

    #[test]
    fn prime_test() {
        let t1 = 53;
        let prime = crate::is_prime(t1);
        println!("{} = prime {}", t1, prime);
        assert!(prime);
    }

    #[test]
    fn prime_fast_test() {
        let t1 = 53;
        let prime = crate::is_prime_fast(t1);
        println!("{} = prime {}", t1, prime);
        assert!(prime);
    }
}
