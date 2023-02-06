#[allow(dead_code)]
pub fn bubble_sort(values: &[i32]) -> Vec<i32>{
    let mut result = values.to_vec();

    let result_size = result.len();
    for i in 0..result_size - 1 {
        for j in i..result_size - 1 {
            if result[j] > result[j + 1] {
                let value = result[j];
                result[j] = result[j + 1];
                result[j + 1] = value;
            }
        }
    }

    return result;
}

#[allow(dead_code)]
pub fn insertion_sort(values: &[i32]) -> Vec<i32>{
    let mut result = values.to_vec();

    for i in 1..result.len() {
        let mut j = i;
        let element_next = result[i];
        while j > 0 && result[j-1] > element_next {
            result[j] = result[j - 1];
            j = j - 1;
        }
        result[j] = element_next;
    }

    return result
}

#[allow(dead_code)]
pub fn merge_sort(values: &[i32]) -> Vec<i32> {

    return vec![1, 2];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_sort_test() {
        let values = [25, 21, 22, 24, 23, 27, 26];
        let result = bubble_sort(&values);
        assert_eq!(result, [21, 22, 23, 24, 25, 26, 27]);
    }

    #[test]
    fn insertion_sort_test() {
        let values = [25, 21, 22, 24, 23, 27, 26];
        let result = insertion_sort(&values);
        assert_eq!(result, [21, 22, 23, 24, 25, 26, 27]);
    }

    #[test]
    fn merge_sort_test() {
        let mut values = [25, 21, 22, 24, 23, 27, 26];
        let mut t = values.clone();
        merge_sort(&mut values);
        // assert_eq!(values, [21, 22, 23, 24, 25, 26, 27]);

        t[0] = 0;
        println!("{:?}", t);
        println!("{:?}", values);
    }
}