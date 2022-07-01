use leetcode::Solution;

fn main() {
    let solution = Solution::new();
    assert_eq!(solution.roman_to_int(String::from("XII")), 12);
    assert_eq!(solution.roman_to_int(String::from("XCIV")), 94);
    assert_eq!(solution.roman_to_int(String::from("MMMDXXIV")), 3524);
}