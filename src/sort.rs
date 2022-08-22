/// Implementación sencilla de insertion sort
pub fn sort(nums: &Vec<u32>) -> Vec<u32> {
    let mut result = nums.clone();
    for i in 0..result.len() {
        let mut min = i;
        for j in i..result.len() {
            if result[j] < result[min] {
                min = j;
            }
        }
        result.swap(i, min);
    }
    result
}
