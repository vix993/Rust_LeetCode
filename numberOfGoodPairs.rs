// This task is an easy level problem on leetCode.
// The idea is to iterate through a list of i32's
// and count the amount of times that the elements
// in the i and j positions of the array are equal.

// This solution runs  in 0ms and is the fastest on the platform.

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut i = 0;
    let mut j = 0;
    for _num in &nums {
        for _number in &nums {
            if nums[i] == nums[j] && i < j {
                res += 1;
            }
            j += 1;
        }
        i += 1;
        j = 0;
    }
    return res;
}

fn main() {
    println!("{:?}", num_identical_pairs([1, 2, 3, 1, 1].to_vec())); // enter list of i32's where the Vec<i32> is
}
