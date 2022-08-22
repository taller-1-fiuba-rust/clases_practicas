extern crate clase1;

use clase1::sort::sort;

#[test]
fn sort_short() {
    let numbers = vec![4, 5, 2, 7];
    let sorted = sort(&numbers);
    assert_eq!(sorted, vec![2, 4, 5, 7]);
}
