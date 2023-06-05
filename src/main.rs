fn calculate_sum(numbers: &[i32]) -> Result<i32, String> {
    let sum = numbers.iter().sum();
    Ok(sum)
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    match calculate_sum(&numbers) {
        Ok(sum) => println!("Sum of numbers: {}", sum),
        Err(err) => println!("Error: {}", err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculation() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(calculate_sum(&numbers), Ok(15));
    }

    #[test]
    fn test_calculation2() {
        let numbers = vec![1, 2, 3, 4, 6];
        assert_eq!(calculate_sum(&numbers), Ok(16));
    }
}
