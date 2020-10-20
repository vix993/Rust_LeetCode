// This is an easy level coding challenge on leetCode
// the idea is to sort the even numbers into the first half
// and the odd numbers to the second half of the array

// As of my submission this was the fastest and most efficient solution
// built with Rust.

pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut ret: Vec<i32> = [].to_vec();
        
        for num in a {
            if num % 2 == 0 {
                ret.insert(0, num);
            } else {
                ret.push(num);
            }
        }
        return ret;
    }
    
pub fn main() {
    println!("{:?}", sort_array_by_parity([2, 3, 4, 5].to_vec()));
}
