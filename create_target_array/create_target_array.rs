// This is an easy level challenge on leetCode.
// The idea is to create an array where the given elements
// are concatenated at a specific index.

// As of my submission this was the fastest and most efficient
// on the platform with Rust.


pub fn create_target_array(mut nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
          let mut ret: Vec<i32> = [].to_vec();
          let mut j = 0;

          for i in index {
              ret.insert(i as usize, nums[j]);
              j += 1;
          }
          return ret;
      }
      
pub fn main() {
    println!("{:?}", create_target_array([1,2,3,4,5].to_vec(), [0,1,2,2,1].to_vec()));
}
