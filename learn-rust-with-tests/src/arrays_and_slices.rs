pub fn sum_them(numbers: &[usize]) -> usize {
    numbers.iter().sum()
}

pub fn sum_all(numbers_to_sum: Vec<Vec<usize>>) -> Vec<usize> {
    numbers_to_sum
        .into_iter()
        .map(|numbers| sum_them(&numbers))
        .collect()
}

pub fn sum_all_tails(numbers_to_sum: Vec<Vec<usize>>) -> Vec<usize> {
    numbers_to_sum
        .into_iter()
        .map(|numbers| match &numbers.len() {
            0 => 0,
            _ => sum_them(&numbers[1..]),
        })
        .collect()
}
