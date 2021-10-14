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
fn main() {
    let my_name = "Loki Laufeyson";

    assert!(
        my_name == "Loki Laufeyson",
        "{} should be Loki Laufeyson",
        my_name
    );
    assert_eq!(
        my_name, "Loki Laufeyson",
        "{} and Loki Laufeyson should be equal",
        my_name
    );
    assert_ne!(
        my_name, "Mithridates",
        "You entered {}. Input must not equal Mithridates",
        my_name
    );
// let vec = vec![(1, 'a'), (1, 'b'), (1, 'c'), (2, 'a'), (2, 'b')];
// let uniq_by_fst_comp = || {
//     let mut seen = std::collections::HashSet::new();
//     vec.iter().copied().filter(move |x| seen.insert(x.0))
// };
//
// assert_eq!(uniq_by_fst_comp().last(), Some((2, 'a')));
// assert_eq!(uniq_by_fst_comp().next_back(), Some((2, 'b')));
//
// assert_eq!(
//     uniq_by_fst_comp().fold(vec![], |mut v, x| {v.push(x); v}),
//     vec![(1, 'a'), (2, 'a')]
// );
// assert_eq!(
//     uniq_by_fst_comp().rfold(vec![], |mut v, x| {v.push(x); v}),
//     vec![(2, 'b'), (1, 'c')]
// );
}
