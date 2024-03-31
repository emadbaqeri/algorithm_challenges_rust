#![allow(unused)]
use std::collections::HashMap;

pub fn two_sum_naive(input: Vec<i32>, target: i32) -> (i32, i32) {
    let mut result: (i32, i32) = (0, 0);

    for i in 0..input.len() {
        for j in i..input.len() {
            if input[i] + input[j] == target {
                result = (input[i], input[j])
            }
        }
    }

    result
}

pub fn two_sum_with_hashtable(input: Vec<i32>, target: i32) -> (i32, i32) {
    let mut result: (i32, i32) = (0, 0);
    let mut history: HashMap<i32, usize> = HashMap::new();

    for &current in &input {
        if let Some(&index) = history.get(&(target - current)) {
            result = (input[index], current);
            break;
        }
        history.insert(current, history.len());
    }

    result
}

pub fn two_sum_with_sorted_input(input: Vec<i32>, target: i32) -> (i32, i32) {
    let mut input = input.clone();
    input.sort();

    let mut left: usize = 0;
    let mut right: usize = input.len() - 1;
    let mut result: (i32, i32) = (0, 0);

    while left < right {
        match input[left] + input[right] {
            sum if sum == target => {
                result = (input[left], input[right]);
                break;
            }
            sum if sum < target => left += 1,
            sum if sum > target => right -= 1,
            _ => result = (0, 0),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{two_sum_naive, two_sum_with_hashtable, two_sum_with_sorted_input};

    #[test]
    fn two_sum_naive_works_fine() {
        let input = vec![3, 4, -1, 11, 9, 8, -4];
        let target = 10;
        let result = two_sum_naive(input, target);
        assert_eq!(result, (-1, 11));
    }

    #[test]
    fn two_sum_with_hashtable_works_fine() {
        let input = vec![3, 4, -1, 11, 9, 8, -4];
        let target = 10;
        let result = two_sum_with_hashtable(input, target);
        assert_eq!(result, (-1, 11));
    }

    #[test]
    fn two_sum_with_sorted_input_works_fine() {
        let input = vec![3, 4, -1, 11, 9, 8, -4];
        let target = 10;
        let result = two_sum_with_sorted_input(input, target);
        assert_eq!(result, (-1, 11));
    }
}
