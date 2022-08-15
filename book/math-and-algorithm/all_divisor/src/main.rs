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
    let mut target_value = n;
    let mut index = 2;
    let mut result: Vec<i64> = vec![];
    while index * index <= target_value {
        if target_value % index == 0 {
            target_value /= index;
            result.push(index);
        }

        index += 1;
    }

    if target_value > 2 {
        result.push(target_value);
    }

    result.sort();
    result
}

#[cfg(test)]
mod test {
    #[test]
    fn get_all_divisor() {
        let v = crate::all_divisor(100);
        assert_eq!(v, vec![2, 5, 10]);
    }
}
