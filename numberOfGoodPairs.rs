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
    println!("{:?}", num_identical_pairs([1, 2, 3, 1, 1].to_vec())); // enter list of i32's where the !!!'s are
}
