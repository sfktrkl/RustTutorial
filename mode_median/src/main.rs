use std::collections::HashMap;

fn average(values: &Vec<i32>) -> f32 {
    values.iter().sum::<i32>() as f32 / values.len() as f32
}

fn median(values: &Vec<i32>) -> f32 {
    let mut numbers = values.clone();
    numbers.sort_unstable();

    let middle = numbers.len() / 2;
    match numbers.len() % 2 {
        1 => numbers[middle] as f32,
        _ => (numbers[middle - 1] + numbers[middle]) as f32 / 2.0,
    }
}

fn mode(values: &Vec<i32>) -> Vec<i32> {
    let mut frequency = HashMap::new();
    for &value in values {
        *frequency.entry(value).or_insert(0) += 1;
    }

    let mut max_value = 0;
    let mut max_keys = vec![0];
    for (key, value) in frequency {
        if value > max_value {
            max_value = value;
            max_keys.clear();
            max_keys.push(key);
        } else if value == max_value {
            max_keys.push(key);
        }
    }
    max_keys.sort_unstable();
    max_keys
}

fn main() {
    let values = vec![1, 9, 5, 6, 7, 5, 6, 3, 8, 2, 6, 8, 1, 1, 8, 5, 6, 5, 7];
    println!("Average: {}", average(&values));
    println!("Median: {}", median(&values));
    println!("Mode: {:?}", mode(&values));
}
