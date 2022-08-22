pub fn get_max(nums: &[u32]) -> Option<u32> {
    if nums.len() == 0 {
        None
    } else {
        let mut max = nums[0];
        for num in nums {
            if *num > max {
                max = *num;
            }
        }
        Some(max)
    }
}

#[test]
fn test_max() {
    let max = get_max(&[2, 4, 1, 3]);
    assert_eq!(max, Some(4));
}

#[test]
fn test_max_none() {
    let max = get_max(&[]);
    assert_eq!(max, None);
}
