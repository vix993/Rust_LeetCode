// This is a medium level coding challenge
// from leetCode.

// The idea is to receive a matrix where the
// vectors are populated by binary data.
// Each row or column can be 'moved'
// When moved, the row or column's 0s become 1s
// and Vice Versa.

// We must find the maximum value of the sum
// of the possible joined vector content in decimal

// As of my submission this is the fastest
// and least costly solution with Rust

use std::cmp;

pub fn matrix_score(a: Vec<Vec<i32>>) -> i32 {
    let m = a.len();
    let n = a[0].len();
    let mut ret = m;
        
    for j in 1..n {
        let mut n_ones: usize = 0;
        for i in 0..m {
            n_ones += (a[i][j] == a[i][0]) as usize;
        }
        ret = (ret << 1) + cmp::max(n_ones, m -n_ones);
    }
    return ret as i32;
}

fn main() {
    println!(
        "{}",matrix_score([
                    [1,1,0,1,1].to_vec(),
                    [0,0,1,0,0].to_vec(),
                    [1,0,0,0,1].to_vec()].to_vec())); // change vectors for different conditions
}
