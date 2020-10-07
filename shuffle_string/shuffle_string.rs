// This is an easy level challenge on leetcode
// The idea is to shuffle the given string array
// in the order that is indicated in the given Vec<i32>
// e.g. [1, 0, ...] -> "ehllo" = "e" goes in position 1
// and "h" in position 0

// As of the submission, this solution is the fastest with Rust

pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let copy: Vec<char> = s.chars().collect();
    let mut ret: String = String::new();

    for j in 0..s.len() {
        ret.push(
            copy[
                indices
                .iter()
                .position(|&x| x == j as i32)
                .unwrap()
            ]
        )
    }
    return ret;
}

fn main() {
    println!("{:?}",
             restore_string("codeleet".to_string(),
                            [4, 5, 6, 7, 0, 2, 1, 3].to_vec())); // swap string and vector inputs for testing
}
