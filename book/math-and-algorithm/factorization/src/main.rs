use std::io;

fn main() {
    println!("input ");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("input error");
    let input_vaule = buffer.trim().parse::<i64>().unwrap();

    let result = factorization(input_vaule);

    println!("result: {:?}", result);
}

fn factorization(n: i64) -> Vec<i64> {
    let mut i = 2;
    let mut value = n;
    let mut result = Vec::<i64>::new();

    while i * i <= n {
        while value % i == 0 {
            value /= i;
            result.push(i);
        }
        i += 1;
    }

    if value >= 2 {
        result.push(value);
    }

    result
}
