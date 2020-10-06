// This is an easy level leetCode challenge
// The idea is to receive an array and a number
// that represents where the second half of the
// vector starts. Then we must return an array with
// the latter half shuffle into the array in alternating
// fashion.

pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut shuffled_array = Vec::new();
    let temp: usize = n as usize;
        
    for i in  0..nums.len() / 2 {
        shuffled_array.push(nums[i]);
        shuffled_array.push(nums[temp + i]);
    }
    return shuffled_array;
}

fn main() {
    println!("{:?}", shuffle([2,5,1,3,4,7].to_vec(), 3));
    // enter vector of i32, and the start index of the second half of the vector
}
