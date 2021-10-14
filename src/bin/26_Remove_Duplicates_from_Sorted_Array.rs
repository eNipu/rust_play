struct Solution;

impl Solution {
    fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let mut last = nums[0];
        let mut size = 1;
        for i in 1..n {
            println!("Out={}, last={}",nums[i],last);
            if nums[i] != last {
                println!("Out={}, last={}, size = {}",nums[i],last, size);
                last = nums[i];
                nums[size] = nums[i];
                size += 1;
            }
        }
        size as i32
    }
}

#[test]
fn test() {
    let mut nums = vec![1, 1, 2];
    assert_eq!(Solution::remove_duplicates(&mut nums), 2);
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(Solution::remove_duplicates(&mut nums), 5);
}

fn main() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(Solution::remove_duplicates(&mut nums), 5);
}