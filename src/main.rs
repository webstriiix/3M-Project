use std::{collections::HashMap};

fn main() {
    let numbers = vec![5, 2, 7, 2, 8, 4, 5, 6, 1, 8, 8]; 

    let mean = calculate_mean(&numbers);
    let median = calculate_median(&numbers);
    let mode = calculate_mode(&numbers);
    
    println!("Numbers: {:?}",numbers);
    println!("Mean: {:.2}",mean);
    println!("Median: {:.2}",median);
    match mode {
        Some(value) => println!("Mode: {}",value),
        None => println!("No Mode Found!!!"),
    }
}

fn calculate_mean(numbers: &Vec<i32>) -> f64 {
    let sum:i32 = numbers.iter().sum();
    let count = numbers.len() as f64;

    sum as f64 / count
}

fn calculate_median(numbers: &Vec<i32>) -> f64 {
    let mut sorted_number = numbers.clone();
    sorted_number.sort();

    let middle = sorted_number.len() / 2;

    if sorted_number.len() % 2 == 0 {
        let middle1 = sorted_number[middle - 1] as f64;
        let middle2 = sorted_number[middle] as f64;

        (middle1 + middle2) / 2.0
    } else{
        sorted_number[middle] as f64
    }
}

fn calculate_mode(numbers: &Vec<i32>) -> Option<i32> {
    let mut count_map = HashMap::new();

    for &num in numbers {
        *count_map.entry(num).or_insert(0) += 1;
    }
    
    let mut mode = None;
    let mut max_count = 0;
    
    for (&num, &count) in &count_map{
        if count > max_count {
            mode = Some(num);
            max_count = count;
        }
    }

    mode
}

