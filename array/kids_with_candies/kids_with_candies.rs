// This is an easy level challenge on leetCode.
// The idea is to create an array where the given elements
// are concatenated at a specific index.

// As of my submission this was the fastest and most efficient
// on the platform with Rust.


impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_candies = candies.iter().max().unwrap();
        let mut ret: Vec<bool> = vec![];
        
        for candy in candies.clone() {
            if (candy + extra_candies) >= *max_candies {
                ret.push(true);
                continue;
            }
            ret.push(false);
        }
        return ret;
    }
}
      
pub fn main() {
    println!("{:?}", assert_eq!(Solution::kids_with_candles([1,2,3,1,10].to_vec(), 7), [false, false, true, false, true]));
}
