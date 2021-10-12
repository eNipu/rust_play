struct Solution;

use std::collections::HashMap;

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - num)) {
                return vec![j, i as i32];
            } else {
                map.insert(num, i as i32);
            }
        }
        vec![]
    }
}
#[test]
fn test() {
    let nums = vec![2, 7, 11, 15];
    let taget = 9;
    assert_eq!(Solution::two_sum(nums, taget), vec![0, 1])
}
fn main() {}
