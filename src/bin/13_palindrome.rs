struct Solution;

impl Solution {
    fn is_plindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }
        let mut rev = 0;
        let mut number = x;
        while number > 0 {
            rev = rev * 10 + number % 10;
            number /= 10;
        }
        x == rev
    }
}

fn main() {
    println!("{}", Solution::is_plindrome(-123))
}
