use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("stdin error");

    if let Ok(value) = buffer.trim().parse::<i64>() {
        let result = all_divisor(value);
        println!("divisor list: {:?}", result);
    }
}

fn all_divisor(n: i64) -> Vec<i64> {
    let mut index = 1;
    let mut result: Vec<i64> = vec![];
    while index * index <= n {
        if n % index != 0 {
            index += 1;
            continue;
        }

        result.push(index);
        if index != n / index {
            result.push(n / index);
        }

        index += 1;
    }

    result.sort();
    result
}

#[cfg(test)]
mod test {
    #[test]
    fn get_all_divisor() {
        let v = crate::all_divisor(100);
        assert_eq!(v, vec![1, 2, 4, 5, 10, 20, 25, 50, 100]);
    }
}
