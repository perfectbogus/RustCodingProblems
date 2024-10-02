/*
Problem:
Given a string S, return the longest palindromic substring in S
 */

pub fn longest_palindrome(s: String) -> String {
    let left_s: &str;
    let right_s: &str;
    let size  = s.chars().count();
    String::from("")
}

pub fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let mut low = 0;
    let mut up = chars.len() - 1;
    while low < up {
        if chars[low] != chars[up] {
            return false;
        }
        low += 1;
        up -= 1;
    }
    true
}


fn main() {

    let s = "radar";
    println!("s: {} is palindrome: {}", s, is_palindrome(s));


    //
    // let test_cases = vec![
    //     "babad".to_string(),
    //     "cbbd".to_string(),
    //     "a".to_string(),
    //     "ac".to_string(),
    // ];
    //
    // for case in test_cases {
    //     println!("Input: {}", case);
    //     println!("Output: {}", longest_palindrome(case));
    //     println!();
    // }
}

/*
Approaches:
    1: two pointers
    2: split the word in two parts from the max size of the word
        to small size. and reverse on of them.
        Size 6:
            0..2 - 3..5 -> 0..2 - 5..3
        Size 5:
            0..1 - 3..4 -> 0..1 - 4..3

        if they are not palindrome reduce the size in one
        Size 6: (reduced)
        Size 5 (left) Size 5 (right)
        0..4 (left) 1..5 (right)

 */
