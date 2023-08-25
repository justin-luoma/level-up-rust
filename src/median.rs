
fn median(nums: Vec<f32>) -> Option<f32> {

    if nums.is_empty() {
        return None;
    }

    let mut nums = nums;
    nums.sort_by(|a, b| a.total_cmp(b));

    if nums.len() % 2 == 0 {
        let half = nums.len() / 2;
        Some((nums[half] + nums[half - 1]) / 2.0)
    } else {
        Some(nums[nums.len() / 2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_even_elements() {
        assert_eq!(median(vec![1.0, 6.0, 5.0, 3.0]), Some(4.0));
    }

    #[test]
    fn test_median_odd_elements() {
        assert_eq!(median(vec![1.0, 6.0, 5.0]), Some(5.0));
    }

    #[test]
    fn test_median_no_elements() {
        assert_eq!(median(vec![]), None);
    }
}
