/*
Problem:
Given a string S, return the longest palindromic substring in S
 */

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

pub fn expand_around_center(chars: &[char], mut left: i32, mut right: i32) -> String {
    println!("left: {}, right: {}", left, right);
    while left >= 0 && right < chars.len() as i32 && chars[left as usize] == chars[right as usize] {
        println!("expand char[left]: {}, char[right]: {}", chars[left as usize], chars[right as usize]);
        left -= 1;
        right += 1;
    }
    chars[(left+1) as usize..right as usize].iter().collect()
}

pub fn longest_palindrome(s: String) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut longest = String::new();
    let size = chars.len() as i32;

    for i in 0..size {
        // Check for odd-length palindromes
        println!("check odd pal");
        let odd_pal = expand_around_center(&chars, i, i);
        println!("odd pal: {}", odd_pal);
        if odd_pal.len() > longest.len() {
            longest = odd_pal;
        }

        // Check for even-length palindromes
        if i < size - 1 {
            println!("check even pal");
            let even_pal = expand_around_center(&chars, i, i+1);
            println!("even pal: {}", even_pal);
            if even_pal.len() > longest.len() {
                longest = even_pal;
            }
        }
    }

    longest
}

fn main() {

    let s = String::from("radar");
    //println!("s: {} is palindrome: {}", s, is_palindrome(s));

    let longest = longest_palindrome(s);
    println!("longest: {}", longest);

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
